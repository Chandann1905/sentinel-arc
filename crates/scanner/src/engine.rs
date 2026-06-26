use crate::cache::compute_hash;
use crate::config::ScanConfig;
use crate::languages::{ExtractedSymbol, SymbolType, get_scanner_for_file};
use ignore::WalkBuilder;
use sentinel_arc_core::domain::node::Node;
use sentinel_arc_core::domain::relationship::Relationship;
use sentinel_arc_core::error::BrainResult;
use sentinel_arc_core::types::ids::NodeId;
use sentinel_arc_core::types::node_type::NodeType;
use sentinel_arc_core::types::relationship_type::RelationshipType;
use sentinel_arc_knowledge::engine::knowledge_engine::KnowledgeEngine;
use std::collections::HashMap;
use std::fs;

pub struct ScannerEngine;

impl ScannerEngine {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ScannerEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl ScannerEngine {
    /// Scans the workspace, parsing supported files and ingesting symbols into the Knowledge Engine.
    pub async fn scan_workspace(
        &self,
        knowledge: &KnowledgeEngine,
        config: ScanConfig,
    ) -> BrainResult<()> {
        let walker = WalkBuilder::new(&config.workspace_root)
            .hidden(false)
            .git_ignore(true)
            .build();

        // Pre-load file nodes to quickly check the hash cache
        let all_nodes = knowledge.list_nodes().await?;
        let mut file_nodes_cache: HashMap<String, Node> = all_nodes
            .into_iter()
            .filter(|n| n.node_type == NodeType::File)
            .map(|n| (n.title.clone(), n))
            .collect();

        for result in walker {
            let entry = match result {
                Ok(e) => e,
                Err(e) => {
                    eprintln!("Warning: Error walking directory: {}", e);
                    continue;
                }
            };

            if entry.file_type().map_or(true, |ft| ft.is_dir()) {
                continue;
            }

            let path = entry.path();

            if let Ok(metadata) = fs::metadata(path) {
                if metadata.len() as usize > config.max_file_size_bytes {
                    continue;
                }
            }

            let scanner = match get_scanner_for_file(path) {
                Some(s) => s,
                None => continue,
            };

            let content_bytes = match fs::read(path) {
                Ok(c) => c,
                Err(_) => continue,
            };

            let content = match std::str::from_utf8(&content_bytes) {
                Ok(c) => c,
                Err(_) => continue,
            };

            let hash = compute_hash(&content_bytes);

            let relative_path = path
                .strip_prefix(&config.workspace_root)
                .unwrap_or(path)
                .to_string_lossy()
                .to_string();

            let existing_node = file_nodes_cache.get(&relative_path).cloned();

            if let Some(ref node) = existing_node {
                if let Some(stored_hash) = node.metadata.get("file_hash") {
                    if stored_hash == &hash {
                        // Unchanged file, skip parsing
                        continue;
                    }
                }
            }

            let symbols = scanner.scan(content)?;

            let mut file_node = match existing_node {
                Some(n) => n,
                None => Node::new(NodeType::File, relative_path.clone()),
            };

            if let Some(obj) = file_node.metadata.as_object_mut() {
                obj.insert(
                    "file_hash".to_string(),
                    serde_json::Value::String(hash.clone()),
                );
            }

            if knowledge.get_node(&file_node.id).await.is_ok() {
                knowledge.update_node(file_node.clone()).await?;
            } else {
                knowledge.create_node(file_node.clone()).await?;
            }

            // Update cache so subsequent identical paths (if any) don't duplicate
            file_nodes_cache.insert(relative_path.clone(), file_node.clone());

            for symbol in symbols {
                Box::pin(self.ingest_symbol(knowledge, symbol, &file_node.id)).await?;
            }
        }

        Ok(())
    }

    async fn ingest_symbol(
        &self,
        knowledge: &KnowledgeEngine,
        symbol: ExtractedSymbol,
        parent_id: &NodeId,
    ) -> BrainResult<()> {
        let node_type = match symbol.symbol_type {
            SymbolType::Module => NodeType::Module,
            SymbolType::Function => NodeType::Function,
        };

        let node = Node::new(node_type, symbol.name);
        knowledge.create_node(node.clone()).await?;

        let rel = Relationship::new(node.id.clone(), parent_id.clone(), RelationshipType::PartOf);
        knowledge.create_relationship(rel).await?;

        for child in symbol.children {
            Box::pin(self.ingest_symbol(knowledge, child, &node.id)).await?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sentinel_arc_knowledge::database::Database;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    async fn setup_db() -> (tempfile::TempDir, Database) {
        let dir = tempdir().unwrap();
        let db = Database::init(dir.path()).await.unwrap();
        (dir, db)
    }

    #[tokio::test]
    async fn test_scanner_engine() {
        let (_dir, db) = setup_db().await;
        let knowledge = KnowledgeEngine::new(&db);
        let scanner = ScannerEngine::new();

        // Create a fake workspace
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("main.rs");
        let mut file = File::create(&file_path).unwrap();
        writeln!(
            file,
            "fn main() {{ println!(\"Hello\"); }} struct User {{ id: i32 }}"
        )
        .unwrap();

        let config = ScanConfig::new(dir.path().to_path_buf());

        // First scan
        scanner
            .scan_workspace(&knowledge, config.clone())
            .await
            .unwrap();

        // Verify nodes created
        let nodes = knowledge.list_nodes().await.unwrap();
        // File, main, User
        assert_eq!(nodes.len(), 3);
        let mut node_names: Vec<String> = nodes.into_iter().map(|n| n.title).collect();
        node_names.sort();
        assert_eq!(node_names, vec!["User", "main", "main.rs"]);

        // Rescan without changes
        scanner
            .scan_workspace(&knowledge, config.clone())
            .await
            .unwrap();

        let nodes_after = knowledge.list_nodes().await.unwrap();
        assert_eq!(nodes_after.len(), 3); // Unchanged, cache hit!

        // Modify file
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "fn new_main() {{}} struct NewUser {{}}").unwrap();

        // Third scan
        scanner.scan_workspace(&knowledge, config).await.unwrap();

        // MVP: It will duplicate or insert the new nodes.
        let nodes_final = knowledge.list_nodes().await.unwrap();
        // Since we didn't cascade delete for MVP, it keeps the old ones + 2 new ones + updates the file
        // 1 file + 2 old + 2 new = 5 nodes
        assert_eq!(nodes_final.len(), 5);
    }
}
