use anyhow::{Context, Result};
use console::{Emoji, style};
use indicatif::{ProgressBar, ProgressStyle};
use sentinel_arc_knowledge::database::Database;
use sentinel_arc_knowledge::engine::knowledge_engine::KnowledgeEngine;
use std::path::Path;
use std::time::Instant;

static CHECK: Emoji<'_, '_> = Emoji("✅ ", "");

pub async fn handle() -> Result<()> {
    let start_time = Instant::now();

    let db_path = Path::new(".sentinel/knowledge.db");
    let db = Database::init(db_path)
        .await
        .context("Failed to initialize database")?;
    let knowledge = KnowledgeEngine::new(&db);

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
            .template("{spinner:.blue} [{elapsed_precise}] {msg}")?,
    );
    pb.set_message("Rebuilding search index...");
    pb.enable_steady_tick(std::time::Duration::from_millis(100));

    knowledge.rebuild_search_index().await?;

    pb.finish_and_clear();

    println!(
        "{} {}",
        CHECK,
        style("Search Index Rebuilt Successfully!").green().bold()
    );
    println!("  Elapsed Time: {:.2?}", start_time.elapsed());

    Ok(())
}
