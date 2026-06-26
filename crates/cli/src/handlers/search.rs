use anyhow::{Context, Result};
use comfy_table::{presets::UTF8_FULL, Cell, Color, Table};
use sentinel_arc_core::types::node_type::NodeType;
use sentinel_arc_knowledge::database::Database;
use sentinel_arc_knowledge::engine::knowledge_engine::KnowledgeEngine;
use std::path::Path;

pub async fn handle(
    query: &str,
    node_type_str: Option<String>,
    limit: usize,
    json: bool,
) -> Result<()> {
    let db_path = Path::new(".sentinel/knowledge.db");
    let db = Database::init(db_path)
        .await
        .context("Failed to initialize database. Run `sentinel init`.")?;
    let knowledge = KnowledgeEngine::new(&db);

    let parsed_node_type = match node_type_str.as_deref() {
        Some("Feature") => Some(NodeType::Feature),
        Some("Task") => Some(NodeType::Task),
        Some("Module") => Some(NodeType::Module),
        Some("Function") => Some(NodeType::Function),
        Some("File") => Some(NodeType::File),
        Some(other) => {
            println!("Warning: Unknown node type '{}'", other);
            None
        }
        None => None,
    };

    let mut search_query = sentinel_arc_core::domain::search::SearchQuery::new(query);
    search_query.node_types = parsed_node_type.map(|t| vec![t]);
    search_query.limit = limit;

    let results = knowledge.search_advanced(&search_query)?;

    if json {
        let json_output = serde_json::to_string_pretty(&results)?;
        println!("{}", json_output);
        return Ok(());
    }

    if results.hits.is_empty() {
        println!("No results found for query: '{}'", query);
        return Ok(());
    }

    let mut table = Table::new();
    table.load_preset(UTF8_FULL).set_header(vec![
        Cell::new("ID").fg(Color::Yellow),
        Cell::new("Score").fg(Color::Cyan),
        Cell::new("Type").fg(Color::Magenta),
        Cell::new("Title").fg(Color::Green),
    ]);

    for hit in results.hits {
        table.add_row(vec![
            Cell::new(hit.entity_id),
            Cell::new(format!("{:.2}", hit.score)),
            Cell::new(hit.entity_kind.to_string()),
            Cell::new(hit.title),
        ]);
    }

    println!("{}", table);

    Ok(())
}
