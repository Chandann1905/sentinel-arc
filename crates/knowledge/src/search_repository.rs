//! Tantivy-backed search index repository.
//!
//! This is the **only** component in the system that interacts with Tantivy.
//! It owns the index schema, writer, and reader. All other components must
//! go through `SearchEngine` to access search functionality.

use std::path::Path;
use std::sync::Mutex;

use tantivy::collector::TopDocs;
use tantivy::query::{BooleanQuery, FuzzyTermQuery, Occur, Query, QueryParser, TermQuery};
use tantivy::schema::{
    FAST, Field, IndexRecordOption, STORED, STRING, Schema, TextFieldIndexing, TextOptions, Value,
};
use tantivy::{DateTime as TantivyDateTime, Directory, Index, IndexReader, IndexWriter, Term};

use sentinel_arc_core::domain::search::{SearchEntityKind, SearchHit};
use sentinel_arc_core::error::{BrainError, BrainResult};
use sentinel_arc_core::{Event, Node, Relationship, Rule};

/// All field handles for the unified search index schema.
#[derive(Debug, Clone)]
struct SearchFields {
    entity_id: Field,
    entity_kind: Field,
    title: Field,
    description: Field,
    node_type: Field,
    status: Field,
    tags: Field,
    metadata_text: Field,
    relationship_type: Field,
    source_node: Field,
    target_node: Field,
    event_type: Field,
    author: Field,
    category: Field,
    severity: Field,
    timestamp: Field,
    confidence: Field,
}

/// The sole Tantivy interface. No other component may import tantivy.
pub(crate) struct SearchRepository {
    index: Index,
    reader: IndexReader,
    writer: Mutex<IndexWriter>,
    fields: SearchFields,
    schema: Schema,
}

impl std::fmt::Debug for SearchRepository {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SearchRepository")
            .field("schema_fields", &self.schema.num_fields())
            .finish()
    }
}

impl SearchRepository {
    /// Build the Tantivy schema used by the unified index.
    fn build_schema() -> (Schema, SearchFields) {
        let mut builder = Schema::builder();

        // Indexed text with custom tokenizer settings for full-text search.
        let text_indexing = TextFieldIndexing::default()
            .set_tokenizer("default")
            .set_index_option(IndexRecordOption::WithFreqsAndPositions);
        let text_options = TextOptions::default()
            .set_indexing_options(text_indexing)
            .set_stored();

        let entity_id = builder.add_text_field("entity_id", STRING | STORED);
        let entity_kind = builder.add_text_field("entity_kind", STRING | STORED);
        let title = builder.add_text_field("title", text_options.clone());
        let description = builder.add_text_field("description", text_options.clone());
        let node_type = builder.add_text_field("node_type", STRING | STORED);
        let status = builder.add_text_field("status", STRING | STORED);
        let tags = builder.add_text_field("tags", text_options.clone());
        let metadata_text = builder.add_text_field("metadata_text", text_options);
        let relationship_type = builder.add_text_field("relationship_type", STRING | STORED);
        let source_node = builder.add_text_field("source_node", STRING | STORED);
        let target_node = builder.add_text_field("target_node", STRING | STORED);
        let event_type = builder.add_text_field("event_type", STRING | STORED);
        let author = builder.add_text_field("author", STRING | STORED);
        let category = builder.add_text_field("category", STRING | STORED);
        let severity = builder.add_text_field("severity", STRING | STORED);
        let timestamp = builder.add_date_field("timestamp", STORED | FAST);
        let confidence = builder.add_u64_field("confidence", STORED | FAST);

        let schema = builder.build();
        let fields = SearchFields {
            entity_id,
            entity_kind,
            title,
            description,
            node_type,
            status,
            tags,
            metadata_text,
            relationship_type,
            source_node,
            target_node,
            event_type,
            author,
            category,
            severity,
            timestamp,
            confidence,
        };

        (schema, fields)
    }

    /// Open or create a Tantivy index at the given filesystem path.
    pub fn open(index_path: &Path) -> BrainResult<Self> {
        let (schema, fields) = Self::build_schema();

        std::fs::create_dir_all(index_path)
            .map_err(|e| BrainError::storage(format!("Failed to create index directory: {e}")))?;

        let dir = tantivy::directory::MmapDirectory::open(index_path)
            .map_err(|e| BrainError::storage(format!("Failed to open index directory: {e}")))?;

        let index = Self::open_or_create_index(dir, &schema)?;
        let reader = index
            .reader()
            .map_err(|e| BrainError::storage(format!("Failed to create index reader: {e}")))?;
        let writer = index
            .writer(15_000_000) // 15 MB heap for writer
            .map_err(|e| BrainError::storage(format!("Failed to create index writer: {e}")))?;

        Ok(Self {
            index,
            reader,
            writer: Mutex::new(writer),
            fields,
            schema,
        })
    }

    /// Open or create a Tantivy index using an in-memory directory (for tests).
    pub fn open_in_memory() -> BrainResult<Self> {
        let (schema, fields) = Self::build_schema();
        let index = Index::create_in_ram(schema.clone());

        let reader = index
            .reader()
            .map_err(|e| BrainError::storage(format!("Failed to create index reader: {e}")))?;
        let writer = index
            .writer(15_000_000) // 15 MB heap for tests (Tantivy minimum)
            .map_err(|e| BrainError::storage(format!("Failed to create index writer: {e}")))?;

        Ok(Self {
            index,
            reader,
            writer: Mutex::new(writer),
            fields,
            schema,
        })
    }

    fn open_or_create_index(dir: impl Directory + Clone, schema: &Schema) -> BrainResult<Index> {
        match Index::open(dir.clone()) {
            Ok(index) => Ok(index),
            Err(_) => Index::create(dir, schema.clone(), Default::default())
                .map_err(|e| BrainError::storage(format!("Failed to create index: {e}"))),
        }
    }

    // ── Indexing Operations ─────────────────────────────────────────

    /// Index (or update) a Node in the search index.
    pub fn index_node(&self, node: &Node) -> BrainResult<()> {
        let mut writer = self
            .writer
            .lock()
            .map_err(|e| BrainError::internal(format!("Failed to lock index writer: {e}")))?;

        // Delete any existing document with this entity_id first (upsert).
        let id_term = Term::from_field_text(self.fields.entity_id, node.id.as_str());
        writer.delete_term(id_term);

        let tags_text = node.tags().join(" ");
        let metadata_text = Self::flatten_metadata(&node.metadata);
        let ts = TantivyDateTime::from_timestamp_secs(node.updated_at.timestamp());

        let mut doc = tantivy::TantivyDocument::new();
        doc.add_text(self.fields.entity_id, node.id.as_str());
        doc.add_text(self.fields.entity_kind, "node");
        doc.add_text(self.fields.title, &node.title);
        doc.add_text(self.fields.description, &node.description);
        doc.add_text(
            self.fields.node_type,
            node.node_type.to_string().to_lowercase(),
        );
        doc.add_text(self.fields.status, node.status.to_string().to_lowercase());
        doc.add_text(self.fields.tags, &tags_text);
        doc.add_text(self.fields.metadata_text, &metadata_text);
        doc.add_date(self.fields.timestamp, ts);
        doc.add_u64(self.fields.confidence, node.confidence.value() as u64);

        writer
            .add_document(doc)
            .map_err(|e| BrainError::storage(format!("Failed to index node: {e}")))?;
        writer
            .commit()
            .map_err(|e| BrainError::storage(format!("Failed to commit index: {e}")))?;

        self.reader
            .reload()
            .map_err(|e| BrainError::storage(format!("Failed to reload reader: {e}")))?;

        Ok(())
    }

    /// Index (or update) a Relationship in the search index.
    pub fn index_relationship(&self, rel: &Relationship) -> BrainResult<()> {
        let mut writer = self
            .writer
            .lock()
            .map_err(|e| BrainError::internal(format!("Failed to lock index writer: {e}")))?;

        let id_term = Term::from_field_text(self.fields.entity_id, rel.id.as_str());
        writer.delete_term(id_term);

        let ts = TantivyDateTime::from_timestamp_secs(rel.created_at.timestamp());

        let mut doc = tantivy::TantivyDocument::new();
        doc.add_text(self.fields.entity_id, rel.id.as_str());
        doc.add_text(self.fields.entity_kind, "relationship");
        doc.add_text(self.fields.title, rel.relationship_type.to_string());
        doc.add_text(
            self.fields.relationship_type,
            rel.relationship_type.to_string(),
        );
        doc.add_text(self.fields.source_node, rel.source_node.as_str());
        doc.add_text(self.fields.target_node, rel.target_node.as_str());
        doc.add_date(self.fields.timestamp, ts);
        doc.add_u64(self.fields.confidence, rel.confidence.value() as u64);

        writer
            .add_document(doc)
            .map_err(|e| BrainError::storage(format!("Failed to index relationship: {e}")))?;
        writer
            .commit()
            .map_err(|e| BrainError::storage(format!("Failed to commit index: {e}")))?;

        self.reader
            .reload()
            .map_err(|e| BrainError::storage(format!("Failed to reload reader: {e}")))?;

        Ok(())
    }

    /// Index (or update) a Rule in the search index.
    pub fn index_rule(&self, rule: &Rule) -> BrainResult<()> {
        let mut writer = self
            .writer
            .lock()
            .map_err(|e| BrainError::internal(format!("Failed to lock index writer: {e}")))?;

        let id_term = Term::from_field_text(self.fields.entity_id, rule.id.as_str());
        writer.delete_term(id_term);

        let mut doc = tantivy::TantivyDocument::new();
        doc.add_text(self.fields.entity_id, rule.id.as_str());
        doc.add_text(self.fields.entity_kind, "rule");
        doc.add_text(self.fields.title, &rule.name);
        doc.add_text(self.fields.description, &rule.value);
        doc.add_text(
            self.fields.category,
            rule.category.to_string().to_lowercase(),
        );
        doc.add_text(
            self.fields.severity,
            rule.severity.to_string().to_lowercase(),
        );

        writer
            .add_document(doc)
            .map_err(|e| BrainError::storage(format!("Failed to index rule: {e}")))?;
        writer
            .commit()
            .map_err(|e| BrainError::storage(format!("Failed to commit index: {e}")))?;

        self.reader
            .reload()
            .map_err(|e| BrainError::storage(format!("Failed to reload reader: {e}")))?;

        Ok(())
    }

    /// Index an Event in the search index.
    pub fn index_event(&self, event: &Event) -> BrainResult<()> {
        let mut writer = self
            .writer
            .lock()
            .map_err(|e| BrainError::internal(format!("Failed to lock index writer: {e}")))?;

        let id_term = Term::from_field_text(self.fields.entity_id, event.id.as_str());
        writer.delete_term(id_term);

        let ts = TantivyDateTime::from_timestamp_secs(event.timestamp.timestamp());

        let mut doc = tantivy::TantivyDocument::new();
        doc.add_text(self.fields.entity_id, event.id.as_str());
        doc.add_text(self.fields.entity_kind, "event");
        doc.add_text(self.fields.title, event.event_type.to_string());
        doc.add_text(
            self.fields.event_type,
            event.event_type.to_string().to_lowercase(),
        );
        doc.add_text(self.fields.author, &event.author);
        doc.add_date(self.fields.timestamp, ts);

        writer
            .add_document(doc)
            .map_err(|e| BrainError::storage(format!("Failed to index event: {e}")))?;
        writer
            .commit()
            .map_err(|e| BrainError::storage(format!("Failed to commit index: {e}")))?;

        self.reader
            .reload()
            .map_err(|e| BrainError::storage(format!("Failed to reload reader: {e}")))?;

        Ok(())
    }

    /// Delete a document from the index by entity ID.
    pub fn delete_by_id(&self, entity_id: &str) -> BrainResult<()> {
        let mut writer = self
            .writer
            .lock()
            .map_err(|e| BrainError::internal(format!("Failed to lock index writer: {e}")))?;

        let id_term = Term::from_field_text(self.fields.entity_id, entity_id);
        writer.delete_term(id_term);
        writer
            .commit()
            .map_err(|e| BrainError::storage(format!("Failed to commit index: {e}")))?;

        self.reader
            .reload()
            .map_err(|e| BrainError::storage(format!("Failed to reload reader: {e}")))?;

        Ok(())
    }

    /// Delete all documents from the index.
    pub fn clear(&self) -> BrainResult<()> {
        let mut writer = self
            .writer
            .lock()
            .map_err(|e| BrainError::internal(format!("Failed to lock index writer: {e}")))?;

        writer
            .delete_all_documents()
            .map_err(|e| BrainError::storage(format!("Failed to clear index: {e}")))?;
        writer
            .commit()
            .map_err(|e| BrainError::storage(format!("Failed to commit index: {e}")))?;

        self.reader
            .reload()
            .map_err(|e| BrainError::storage(format!("Failed to reload reader: {e}")))?;

        Ok(())
    }

    // ── Query Operations ────────────────────────────────────────────

    /// Execute a full-text search and return ranked hits.
    #[allow(clippy::too_many_arguments)]
    pub fn search(
        &self,
        query_text: &str,
        entity_kind_filter: Option<&[SearchEntityKind]>,
        node_type_filter: Option<&[String]>,
        status_filter: Option<&[String]>,
        fuzzy: bool,
        offset: usize,
        limit: usize,
    ) -> BrainResult<(Vec<SearchHit>, usize)> {
        let searcher = self.reader.searcher();
        let mut sub_queries: Vec<(Occur, Box<dyn Query>)> = Vec::new();

        // Text query across title + description + tags
        if !query_text.trim().is_empty() {
            if fuzzy {
                let term = Term::from_field_text(self.fields.title, query_text);
                let fuzzy_query = FuzzyTermQuery::new(term, 1, true);
                sub_queries.push((Occur::Must, Box::new(fuzzy_query)));
            } else {
                let parser = QueryParser::for_index(
                    &self.index,
                    vec![self.fields.title, self.fields.description, self.fields.tags],
                );
                match parser.parse_query(query_text) {
                    Ok(parsed) => sub_queries.push((Occur::Must, parsed)),
                    Err(_) => {
                        // If query parsing fails, fall back to a term query on title.
                        let term = Term::from_field_text(self.fields.title, query_text);
                        sub_queries.push((
                            Occur::Must,
                            Box::new(TermQuery::new(term, IndexRecordOption::WithFreqs)),
                        ));
                    }
                }
            }
        }

        // Entity kind filter
        if let Some(kinds) = entity_kind_filter {
            let kind_queries: Vec<(Occur, Box<dyn Query>)> = kinds
                .iter()
                .map(|k| {
                    let term = Term::from_field_text(self.fields.entity_kind, &k.to_string());
                    (
                        Occur::Should,
                        Box::new(TermQuery::new(term, IndexRecordOption::Basic)) as Box<dyn Query>,
                    )
                })
                .collect();
            if !kind_queries.is_empty() {
                sub_queries.push((Occur::Must, Box::new(BooleanQuery::new(kind_queries))));
            }
        }

        // Node type filter
        if let Some(types) = node_type_filter {
            let type_queries: Vec<(Occur, Box<dyn Query>)> = types
                .iter()
                .map(|t| {
                    let term = Term::from_field_text(self.fields.node_type, t);
                    (
                        Occur::Should,
                        Box::new(TermQuery::new(term, IndexRecordOption::Basic)) as Box<dyn Query>,
                    )
                })
                .collect();
            if !type_queries.is_empty() {
                sub_queries.push((Occur::Must, Box::new(BooleanQuery::new(type_queries))));
            }
        }

        // Status filter
        if let Some(statuses) = status_filter {
            let status_queries: Vec<(Occur, Box<dyn Query>)> = statuses
                .iter()
                .map(|s| {
                    let term = Term::from_field_text(self.fields.status, s);
                    (
                        Occur::Should,
                        Box::new(TermQuery::new(term, IndexRecordOption::Basic)) as Box<dyn Query>,
                    )
                })
                .collect();
            if !status_queries.is_empty() {
                sub_queries.push((Occur::Must, Box::new(BooleanQuery::new(status_queries))));
            }
        }

        // If no queries at all, match everything
        let final_query: Box<dyn Query> = if sub_queries.is_empty() {
            Box::new(tantivy::query::AllQuery)
        } else {
            Box::new(BooleanQuery::new(sub_queries))
        };

        let top_docs = TopDocs::with_limit(limit).and_offset(offset);
        let results = searcher
            .search(&final_query, &(top_docs, tantivy::collector::Count))
            .map_err(|e| BrainError::storage(format!("Search failed: {e}")))?;

        let (top_hits, total_count) = results;

        let mut hits = Vec::with_capacity(top_hits.len());
        for (score, doc_address) in top_hits {
            let doc: tantivy::TantivyDocument = searcher
                .doc(doc_address)
                .map_err(|e| BrainError::storage(format!("Failed to retrieve doc: {e}")))?;

            let entity_id = Self::get_text_field(&doc, self.fields.entity_id);
            let entity_kind_str = Self::get_text_field(&doc, self.fields.entity_kind);
            let title = Self::get_text_field(&doc, self.fields.title);
            let node_type_val = Self::get_text_field(&doc, self.fields.node_type);
            let status_val = Self::get_text_field(&doc, self.fields.status);

            let entity_kind = match entity_kind_str.as_str() {
                "node" => SearchEntityKind::Node,
                "relationship" => SearchEntityKind::Relationship,
                "rule" => SearchEntityKind::Rule,
                "event" => SearchEntityKind::Event,
                _ => SearchEntityKind::Node,
            };

            let mut metadata = serde_json::json!({});
            if !node_type_val.is_empty() {
                metadata["node_type"] = serde_json::Value::String(node_type_val);
            }
            if !status_val.is_empty() {
                metadata["status"] = serde_json::Value::String(status_val);
            }

            hits.push(SearchHit {
                entity_id,
                entity_kind,
                score,
                title,
                snippet: None,
                metadata,
            });
        }

        Ok((hits, total_count))
    }

    // ── Private Helpers ─────────────────────────────────────────────

    /// Extract text from a stored field value.
    fn get_text_field(doc: &tantivy::TantivyDocument, field: Field) -> String {
        doc.get_first(field)
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string()
    }

    /// Flatten a JSON metadata object into a space-separated string of values
    /// for full-text indexing.
    fn flatten_metadata(metadata: &serde_json::Value) -> String {
        match metadata {
            serde_json::Value::Object(map) => {
                let mut parts = Vec::new();
                for (key, val) in map {
                    if key == "tags" {
                        continue; // Tags are indexed separately.
                    }
                    match val {
                        serde_json::Value::String(s) => parts.push(s.clone()),
                        serde_json::Value::Array(arr) => {
                            for item in arr {
                                if let serde_json::Value::String(s) = item {
                                    parts.push(s.clone());
                                }
                            }
                        }
                        _ => {}
                    }
                }
                parts.join(" ")
            }
            _ => String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sentinel_arc_core::domain::node::Node;
    use sentinel_arc_core::domain::relationship::Relationship;
    use sentinel_arc_core::domain::rule::{Rule, RuleCategory};
    use sentinel_arc_core::types::ids::NodeId;
    use sentinel_arc_core::types::node_type::NodeType;
    use sentinel_arc_core::types::relationship_type::RelationshipType;

    fn create_test_repo() -> SearchRepository {
        SearchRepository::open_in_memory().unwrap()
    }

    #[test]
    fn index_node_and_search_by_title() {
        let repo = create_test_repo();
        let node = Node::new(NodeType::Feature, "Wallet Module");
        repo.index_node(&node).unwrap();

        let (hits, total) = repo
            .search("wallet", None, None, None, false, 0, 10)
            .unwrap();
        assert_eq!(total, 1);
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].entity_id, node.id.as_str());
        assert_eq!(hits[0].entity_kind, SearchEntityKind::Node);
    }

    #[test]
    fn search_no_results() {
        let repo = create_test_repo();
        let node = Node::new(NodeType::Feature, "Authentication");
        repo.index_node(&node).unwrap();

        let (hits, total) = repo
            .search("zzzznotfound", None, None, None, false, 0, 10)
            .unwrap();
        assert_eq!(total, 0);
        assert!(hits.is_empty());
    }

    #[test]
    fn upsert_updates_existing() {
        let repo = create_test_repo();
        let mut node = Node::new(NodeType::Feature, "Old Title");
        repo.index_node(&node).unwrap();

        node.title = "New Title".to_string();
        repo.index_node(&node).unwrap();

        let (hits, _) = repo.search("Old", None, None, None, false, 0, 10).unwrap();
        assert!(hits.is_empty());

        let (hits, _) = repo.search("New", None, None, None, false, 0, 10).unwrap();
        assert_eq!(hits.len(), 1);
    }

    #[test]
    fn delete_from_index() {
        let repo = create_test_repo();
        let node = Node::new(NodeType::Feature, "DeleteMe");
        repo.index_node(&node).unwrap();
        repo.delete_by_id(node.id.as_str()).unwrap();

        let (hits, total) = repo
            .search("DeleteMe", None, None, None, false, 0, 10)
            .unwrap();
        assert_eq!(total, 0);
        assert!(hits.is_empty());
    }

    #[test]
    fn search_by_entity_kind_filter() {
        let repo = create_test_repo();
        let node = Node::new(NodeType::Feature, "Wallet");
        let rule = Rule::new("backend", RuleCategory::Technology, "supabase");
        repo.index_node(&node).unwrap();
        repo.index_rule(&rule).unwrap();

        // Filter to rules only
        let (hits, _) = repo
            .search(
                "",
                Some(&[SearchEntityKind::Rule]),
                None,
                None,
                false,
                0,
                10,
            )
            .unwrap();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].entity_kind, SearchEntityKind::Rule);
    }

    #[test]
    fn search_by_node_type_filter() {
        let repo = create_test_repo();
        let feature = Node::new(NodeType::Feature, "Wallet");
        let bug = Node::new(NodeType::Bug, "Login crash");
        repo.index_node(&feature).unwrap();
        repo.index_node(&bug).unwrap();

        let (hits, _) = repo
            .search(
                "",
                Some(&[SearchEntityKind::Node]),
                Some(&["Feature".to_lowercase()].map(|s| s.to_string())),
                None,
                false,
                0,
                10,
            )
            .unwrap();
        assert_eq!(hits.len(), 1);
        assert!(hits[0].title.contains("Wallet"));
    }

    #[test]
    fn index_relationship() {
        let repo = create_test_repo();
        let rel = Relationship::new(
            NodeId::from_string("src"),
            NodeId::from_string("tgt"),
            RelationshipType::DependsOn,
        );
        repo.index_relationship(&rel).unwrap();

        let (hits, total) = repo
            .search(
                "",
                Some(&[SearchEntityKind::Relationship]),
                None,
                None,
                false,
                0,
                10,
            )
            .unwrap();
        assert_eq!(total, 1);
        assert_eq!(hits[0].entity_kind, SearchEntityKind::Relationship);
    }

    #[test]
    fn pagination() {
        let repo = create_test_repo();
        for i in 0..10 {
            let node = Node::new(NodeType::Feature, format!("Feature {i}"));
            repo.index_node(&node).unwrap();
        }

        let (hits, total) = repo
            .search("Feature", None, None, None, false, 0, 3)
            .unwrap();
        assert_eq!(total, 10);
        assert_eq!(hits.len(), 3);

        let (hits2, _) = repo
            .search("Feature", None, None, None, false, 3, 3)
            .unwrap();
        assert_eq!(hits2.len(), 3);
        // Ensure different results from page 1
        assert_ne!(hits[0].entity_id, hits2[0].entity_id);
    }

    #[test]
    fn clear_index() {
        let repo = create_test_repo();
        let node = Node::new(NodeType::Feature, "TestClear");
        repo.index_node(&node).unwrap();
        repo.clear().unwrap();

        let (hits, total) = repo
            .search("TestClear", None, None, None, false, 0, 10)
            .unwrap();
        assert_eq!(total, 0);
        assert!(hits.is_empty());
    }

    #[test]
    fn fuzzy_search() {
        let repo = create_test_repo();
        let node = Node::new(NodeType::Feature, "Wallet");
        repo.index_node(&node).unwrap();

        // "Wallt" should fuzzy-match "Wallet" at distance 1
        let (hits, _) = repo.search("wallt", None, None, None, true, 0, 10).unwrap();
        assert!(
            !hits.is_empty(),
            "Fuzzy search should find 'Wallet' for 'wallt'"
        );
    }
}
