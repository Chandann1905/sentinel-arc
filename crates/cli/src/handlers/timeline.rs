use anyhow::{Context, Result};
use comfy_table::{Cell, Table, modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL};
use console::style;
use std::path::Path;
use std::sync::Arc;

use sentinel_arc_core::NodeId;
use sentinel_arc_knowledge::database::Database;
use sentinel_arc_knowledge::engine::knowledge_engine::KnowledgeEngine;
use sentinel_arc_timeline::TimelineEngine;

pub async fn handle(node_id_arg: Option<String>, decisions: bool) -> Result<()> {
    let db_path = Path::new(".sentinel/knowledge.db");
    let db = Database::init(db_path)
        .await
        .context("Failed to initialize database. Run `sentinel init`.")?;
    let ke = Arc::new(KnowledgeEngine::new(&db));

    let timeline_engine = TimelineEngine::new(ke);

    let timeline_result = if decisions {
        timeline_engine.generate_decision_history().await
    } else if let Some(id_str) = node_id_arg {
        let node_id = NodeId::from(id_str.as_str());
        timeline_engine.generate_node_timeline(&node_id).await
    } else {
        timeline_engine.generate_project_timeline(50).await
    };

    match timeline_result {
        Ok(timeline) => {
            if timeline.events.is_empty() {
                println!("No events found for this timeline.");
                return Ok(());
            }

            println!(
                "\n{} Timeline: {:?}",
                style("=>").green().bold(),
                timeline.timeline_type
            );
            println!(
                "{}\n",
                style(format!(
                    "Generated at {}",
                    timeline.generated_at.format("%Y-%m-%d %H:%M:%S")
                ))
                .dim()
            );

            let mut table = Table::new();
            table
                .load_preset(UTF8_FULL)
                .apply_modifier(UTF8_ROUND_CORNERS)
                .set_header(vec!["Timestamp", "Author", "Type", "Description"]);

            for event in timeline.events {
                let timestamp_str = event.timestamp.format("%Y-%m-%d %H:%M:%S").to_string();
                table.add_row(vec![
                    Cell::new(timestamp_str),
                    Cell::new(event.author),
                    Cell::new(event.event_type.to_string()),
                    Cell::new(event.description),
                ]);
            }
            println!("{table}");
            Ok(())
        }
        Err(e) => anyhow::bail!("Error generating timeline: {}", e),
    }
}
