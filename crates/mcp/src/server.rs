use std::sync::Arc;

use crate::config::McpConfig;
use crate::handlers::handle_request;
use crate::transport;
use sentinel_arc_knowledge::engine::knowledge_engine::KnowledgeEngine;
use sentinel_arc_timeline::engine::TimelineEngine;

pub struct McpServer {
    pub config: McpConfig,
    pub knowledge_engine: Arc<KnowledgeEngine>,
    pub timeline_engine: Arc<TimelineEngine>,
}

impl McpServer {
    pub fn new(
        config: McpConfig,
        knowledge_engine: Arc<KnowledgeEngine>,
        timeline_engine: Arc<TimelineEngine>,
    ) -> Self {
        Self {
            config,
            knowledge_engine,
            timeline_engine,
        }
    }

    pub async fn run_stdio(&self) -> std::io::Result<()> {
        let ke = self.knowledge_engine.clone();
        let te = self.timeline_engine.clone();
        let default_timeout = self.config.default_timeout_seconds;

        transport::read_requests(move |req| {
            let ke = ke.clone();
            let te = te.clone();
            Box::pin(async move { handle_request(req, ke, te, default_timeout).await })
        })
        .await
    }
}
