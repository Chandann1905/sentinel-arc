use anyhow::Result;
use console::{style, Emoji};
use std::path::Path;
use std::process::Command;

static CROSS: Emoji<'_, '_> = Emoji("❌ ", "");
static CHECK: Emoji<'_, '_> = Emoji("✅ ", "");
static WARN: Emoji<'_, '_> = Emoji("⚠️ ", "");

pub async fn handle() -> Result<()> {
    println!("{}", style("Sentinel Arc Doctor").bold());
    println!("-------------------");

    // Rust version
    match Command::new("rustc").arg("--version").output() {
        Ok(output) => {
            let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
            println!("{} Rust Version: {}", CHECK, style(version).green());
        }
        Err(_) => println!(
            "{} Rust Version: {}",
            CROSS,
            style("Not found in PATH").red()
        ),
    }

    // Workspace structure
    let dir = Path::new(".sentinel");
    if dir.exists() {
        println!("{} Workspace initialized (.sentinel/ exists)", CHECK);

        let db = dir.join("knowledge.db");
        if db.exists() {
            println!("{} Database found", CHECK);
        } else {
            println!(
                "{} Database missing. Run `sentinel init`.",
                style(CROSS).red()
            );
        }
    } else {
        println!(
            "{} Workspace not initialized. Run `sentinel init`.",
            style(WARN).yellow()
        );
    }

    let search_dir = dir.join("search_index");
    if search_dir.exists() {
        println!("{} Search index directory found", CHECK);
    } else {
        println!("{} Search index directory missing", style(WARN).yellow());
    }

    Ok(())
}
