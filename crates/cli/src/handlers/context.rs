use anyhow::{Context, Result};
use console::style;

use sentinel_arc_knowledge::database::Database;
use sentinel_arc_knowledge::engine::knowledge_engine::KnowledgeEngine;
use std::path::Path;

pub async fn handle(intent: &str, json: bool) -> Result<()> {
    let db_path = Path::new(".sentinel/knowledge.db");
    let db = Database::init(db_path)
        .await
        .context("Failed to initialize database. Run `sentinel init`.")?;
    let knowledge = KnowledgeEngine::new(&db);
    let context_engine = sentinel_arc_context::engine::ContextEngine::new();
    let proj = sentinel_arc_graph::engine::GraphEngine::build_projection(&knowledge).await?;
    let req = sentinel_arc_context::types::ContextRequest::new(intent);
    let package = context_engine
        .generate_context(&knowledge, &proj, req)
        .await?;

    if json {
        let json_output = serde_json::to_string_pretty(&package)?;
        println!("{}", json_output);
        return Ok(());
    }

    println!("{}", style("Sentinel Arc Context Package").bold().cyan());
    println!("Intent: {}\n", style(intent).italic());

    println!("{}", style("Root Nodes:").yellow().bold());
    for node in &package.root_nodes {
        println!("  - {} ({:?})", style(&node.title).green(), node.node_type);
    }

    if !package.related_architecture.is_empty() {
        println!("\n{}", style("Related Architecture:").magenta().bold());
        for node in &package.related_architecture {
            println!("  - {} ({:?})", style(&node.title).cyan(), node.node_type);
        }
    }

    if !package.relevant_rules.is_empty() {
        println!("\n{}", style("Relevant Rules:").blue().bold());
        for rule in &package.relevant_rules {
            println!("  - {} ({:?})", style(&rule.name).cyan(), rule.category);
        }
    }

    println!("\n{}", style("Impact Report:").red().bold());
    println!("  Risk Score: {}", package.impact_report.risk_score);
    println!(
        "  Complexity Score: {}",
        package.impact_report.complexity_score
    );

    Ok(())
}
