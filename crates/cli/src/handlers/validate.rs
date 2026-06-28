use anyhow::{Context, Result};
use console::{Emoji, style};
use sentinel_arc_knowledge::database::Database;
use sentinel_arc_knowledge::engine::knowledge_engine::KnowledgeEngine;
use sentinel_arc_validation::engine::ValidationEngine;
use sentinel_arc_validation::types::Severity;
use std::path::Path;

static CROSS: Emoji<'_, '_> = Emoji("❌ ", "");
static WARN: Emoji<'_, '_> = Emoji("⚠️ ", "");
static INFO: Emoji<'_, '_> = Emoji("ℹ️ ", "");
static CHECK: Emoji<'_, '_> = Emoji("✅ ", "");

pub async fn handle() -> Result<i32> {
    let db_path = Path::new(".sentinel/knowledge.db");
    let db = Database::init(db_path)
        .await
        .context("Failed to initialize database")?;
    let knowledge = KnowledgeEngine::new(&db);
    let validation = ValidationEngine::new();

    let proj = sentinel_arc_graph::engine::GraphEngine::build_projection(&knowledge).await?;
    let scanner = sentinel_arc_scanner::engine::ScannerEngine::new();
    let report = validation
        .run_full_validation(&knowledge, &proj, &scanner)
        .await?;

    if report.issues.is_empty() {
        println!(
            "{} {}",
            CHECK,
            style("Validation Passed! No drift detected.")
                .green()
                .bold()
        );
        return Ok(0);
    }

    let mut exit_code = 0;

    for issue in report.issues {
        match issue.severity {
            Severity::Error => {
                println!("{} {}", CROSS, style(&issue.message).red());
                if exit_code < 2 {
                    exit_code = 2;
                }
            }
            Severity::Warning => {
                println!("{} {}", WARN, style(&issue.message).yellow());
                if exit_code < 1 {
                    exit_code = 1;
                }
            }
            Severity::Info => {
                println!("{} {}", INFO, style(&issue.message).blue());
            }
        }
        if let Some(id) = &issue.node_id {
            println!("    Node ID: {}", style(id).dim());
        }
        if let Some(id) = &issue.relationship_id {
            println!("    Relationship ID: {}", style(id).dim());
        }
    }

    Ok(exit_code)
}
