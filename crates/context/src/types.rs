use sentinel_arc_core::domain::node::Node;
use sentinel_arc_core::domain::rule::Rule;
use serde::{Deserialize, Serialize};

/// A request to generate an AI context package.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextRequest {
    /// The user's natural language intent or query.
    pub intent: String,
    /// Optional limit on how deep to traverse the dependency graph. Default 2.
    pub max_depth: u32,
    /// Maximum number of nodes to return in total. Default 50.
    pub max_nodes: usize,
}

impl Default for ContextRequest {
    fn default() -> Self {
        Self {
            intent: String::new(),
            max_depth: 2,
            max_nodes: 50,
        }
    }
}

impl ContextRequest {
    pub fn new(intent: impl Into<String>) -> Self {
        Self {
            intent: intent.into(),
            ..Default::default()
        }
    }
}

/// The final compressed AI context package.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContextPackage {
    pub intent: String,
    pub root_nodes: Vec<Node>,
    pub related_architecture: Vec<Node>,
    pub relevant_rules: Vec<Rule>,
    pub known_decisions: Vec<Node>,
    pub impact_report: ImpactReport,
}

/// Analysis of the consequences of changing the requested context.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ImpactReport {
    pub affected_features: Vec<Node>,
    pub affected_files: Vec<Node>,
    pub risk_score: u8,
    pub complexity_score: u8,
}
