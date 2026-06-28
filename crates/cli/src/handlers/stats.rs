use anyhow::{Context, Result};
use console::style;
use sentinel_arc_knowledge::database::Database;
use sentinel_arc_knowledge::engine::knowledge_engine::KnowledgeEngine;
use std::fs;
use std::path::Path;

pub async fn handle() -> Result<()> {
    let db_path = Path::new(".sentinel/knowledge.db");
    let db = Database::init(db_path)
        .await
        .context("Failed to initialize database. Run `sentinel init`.")?;
    let knowledge = KnowledgeEngine::new(&db);

    let nodes_count = knowledge.list_nodes().await?.len();
    let rels_count = knowledge.list_all_relationships().await?.len();
    let rules_count = knowledge.list_rules().await?.len();

    let mut events_query = sentinel_arc_core::domain::search::SearchQuery::new("");
    events_query.entity_kinds = Some(vec![
        sentinel_arc_core::domain::search::SearchEntityKind::Event,
    ]);
    let events_count = knowledge
        .search_advanced(&events_query)
        .map(|r| r.total_count)
        .unwrap_or(0);

    let db_size = fs::metadata(db_path).map(|m| m.len()).unwrap_or(0);

    let mut search_size = 0;
    if let Ok(entries) = fs::read_dir("search_index") {
        for entry in entries.flatten() {
            if let Ok(meta) = entry.metadata() {
                search_size += meta.len();
            }
        }
    }

    println!("{}", style("Sentinel Arc Workspace Stats").bold().cyan());
    println!("------------------------------");
    println!("  Total Nodes:         {}", style(nodes_count).yellow());
    println!("  Total Relationships: {}", style(rels_count).yellow());
    println!("  Total Events:        {}", style(events_count).yellow());
    println!("  Total Rules:         {}", style(rules_count).yellow());
    println!(
        "  Database Size:       {}",
        style(format_size(db_size)).green()
    );
    println!(
        "  Search Index Size:   {}",
        style(format_size(search_size)).green()
    );
    println!(
        "  CLI Version:         {}",
        style(env!("CARGO_PKG_VERSION")).dim()
    );

    Ok(())
}

fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.2} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.2} MB", bytes as f64 / 1024.0 / 1024.0)
    }
}
