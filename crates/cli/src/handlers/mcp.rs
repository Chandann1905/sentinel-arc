use anyhow::Result;
use sentinel_arc_knowledge::database::Database;
use sentinel_arc_knowledge::engine::knowledge_engine::KnowledgeEngine;
use sentinel_arc_mcp::{McpConfig, McpServer};
use sentinel_arc_timeline::engine::TimelineEngine;
use std::path::PathBuf;
use std::sync::Arc;

pub async fn handle() -> Result<()> {
    // Determine the .brain directory
    let mut db_path = PathBuf::from(".sentinel");
    if !db_path.exists() {
        // Fall back to home directory global brain or just let initialization fail gracefully
        anyhow::bail!("Workspace not initialized. Please run 'sentinel init' first.");
    }
    db_path.push("knowledge.db");

    let db = Database::init(&db_path).await?;
    let ke = Arc::new(KnowledgeEngine::new(&db));
    let te = Arc::new(TimelineEngine::new(ke.clone()));

    let config = McpConfig::default();
    let mcp_server = McpServer::new(config, ke, te);

    mcp_server.run_stdio().await?;

    Ok(())
}
