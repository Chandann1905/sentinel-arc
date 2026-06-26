use crate::types::ValidationIssue;
use async_trait::async_trait;
use sentinel_arc_core::domain::node::Node;
use sentinel_arc_core::domain::relationship::Relationship;
use sentinel_arc_core::error::BrainResult;
use sentinel_arc_core::types::ids::NodeId;
use sentinel_arc_graph::projection::GraphProjection;
use sentinel_arc_knowledge::engine::knowledge_engine::KnowledgeEngine;
use sentinel_arc_scanner::engine::ScannerEngine;
use std::collections::HashMap;

/// A context object injected into all Validators to provide necessary data and references.
pub struct ValidationContext<'a> {
    pub knowledge: &'a KnowledgeEngine,
    pub projection: &'a GraphProjection,
    pub scanner: &'a ScannerEngine,

    // Cached lists so we don't query SQLite repeatedly for every single validator
    pub all_nodes: Vec<Node>,
    pub node_map: HashMap<NodeId, Node>,
    pub all_relationships: Vec<Relationship>,
}

impl<'a> ValidationContext<'a> {
    pub async fn new(
        knowledge: &'a KnowledgeEngine,
        projection: &'a GraphProjection,
        scanner: &'a ScannerEngine,
    ) -> BrainResult<Self> {
        let all_nodes = knowledge.list_nodes().await?;
        let mut node_map = HashMap::new();
        for n in &all_nodes {
            node_map.insert(n.id.clone(), n.clone());
        }

        let all_relationships = knowledge.list_all_relationships().await?;

        Ok(Self {
            knowledge,
            projection,
            scanner,
            all_nodes,
            node_map,
            all_relationships,
        })
    }
}

/// A pluggable validation rule that inspects the context and returns any discovered issues.
#[async_trait]
pub trait Validator: Send + Sync {
    fn name(&self) -> &'static str;
    async fn validate(&self, ctx: &ValidationContext<'_>) -> BrainResult<Vec<ValidationIssue>>;
}
