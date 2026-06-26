use anyhow::{Context, Result};
use console::{style, Emoji};
use indicatif::{ProgressBar, ProgressStyle};
use sentinel_arc_knowledge::database::Database;
use sentinel_arc_knowledge::engine::knowledge_engine::KnowledgeEngine;
use sentinel_arc_scanner::engine::ScannerEngine;
use std::path::Path;
use std::time::Instant;

static ROCKET: Emoji<'_, '_> = Emoji("🚀 ", "");

pub async fn handle(path: &str) -> Result<()> {
    let start_time = Instant::now();

    let db_path = Path::new(".sentinel/knowledge.db");
    let db = Database::init(db_path)
        .await
        .context("Failed to initialize database. Run `sentinel init`.")?;
    let knowledge = KnowledgeEngine::new(&db);
    let scanner = ScannerEngine::new();

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
            .template("{spinner:.green} [{elapsed_precise}] {msg}")?,
    );
    pb.set_message(format!("Scanning directory: {}", path));
    pb.enable_steady_tick(std::time::Duration::from_millis(100));

    let config = sentinel_arc_scanner::config::ScanConfig::new(Path::new(path).to_path_buf());
    scanner
        .scan_workspace(&knowledge, config)
        .await
        .context("Scanner engine failed")?;

    pb.finish_and_clear();

    println!("{} {}", ROCKET, style("Scan Complete!").green().bold());
    println!("  Elapsed Time: {:.2?}", start_time.elapsed());

    Ok(())
}
