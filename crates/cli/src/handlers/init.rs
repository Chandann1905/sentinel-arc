use anyhow::Result;
use console::{Emoji, style};
use sentinel_arc_knowledge::database::Database;
use sentinel_arc_knowledge::engine::knowledge_engine::KnowledgeEngine;
use std::fs;
use std::path::Path;

static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", "");
static CHECK: Emoji<'_, '_> = Emoji("✅ ", "");

pub async fn handle() -> Result<()> {
    println!(
        "{} {}Initialize Sentinel Arc Workspace",
        SPARKLE,
        style("Starting").cyan().bold()
    );

    let dir = Path::new(".sentinel");
    if !dir.exists() {
        fs::create_dir(dir)?;
        println!("  {} Created .sentinel/ directory", CHECK);
    } else {
        println!("  {} .sentinel/ directory already exists", CHECK);
    }

    let db_path = dir.join("knowledge.db");
    let db = Database::init(&db_path).await?;
    println!("  {} Initialized SQLite Database", CHECK);

    let _engine = KnowledgeEngine::new(&db);
    println!("  {} Verified Search Index", CHECK);

    println!(
        "\n{} {}",
        SPARKLE,
        style("Workspace successfully initialized!").green().bold()
    );

    Ok(())
}
