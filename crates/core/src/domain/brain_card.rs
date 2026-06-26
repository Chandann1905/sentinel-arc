//! Brain Card — the rich summary card for important nodes.
//!
//! Every major node exposes a Brain Card containing its purpose,
//! dependencies, history, roadmap, known issues, and risk assessment.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::ids::NodeId;
use crate::types::node_type::NodeType;
use crate::types::source::Confidence;

/// A Brain Card summarizing a node's full context.
///
/// Brain Cards are computed/generated from the Knowledge Engine —
/// they aggregate data from the node itself, its relationships,
/// events, and associated rules.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BrainCard {
    /// Title of the entity.
    pub title: String,

    /// Type of the entity.
    #[serde(rename = "type")]
    pub node_type: NodeType,

    /// What this entity does / why it exists.
    pub purpose: String,

    /// Current status string.
    pub status: String,

    /// IDs of nodes this entity depends on.
    pub dependencies: Vec<NodeId>,

    /// IDs of related nodes (non-dependency relationships).
    pub related_nodes: Vec<NodeId>,

    /// File paths associated with this entity.
    pub files: Vec<String>,

    /// API endpoints associated with this entity.
    pub apis: Vec<String>,

    /// Database tables associated with this entity.
    pub database_tables: Vec<String>,

    /// Summary of recent history entries.
    pub history: Vec<HistoryEntry>,

    /// Roadmap items related to this entity.
    pub roadmap: Vec<RoadmapEntry>,

    /// Known issues affecting this entity.
    pub known_issues: Vec<String>,

    /// Computed risk score (0–100, higher = riskier).
    pub risk_score: u8,

    /// Confidence in the accuracy of this card's data.
    pub confidence: Confidence,

    /// When this card was last generated/refreshed.
    pub last_updated: DateTime<Utc>,
}

/// A single history entry for the Brain Card timeline.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HistoryEntry {
    /// When this event happened.
    pub timestamp: DateTime<Utc>,

    /// Human-readable description of the change.
    pub description: String,

    /// Who or what made the change.
    pub author: String,
}

/// A roadmap entry for the Brain Card.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoadmapEntry {
    /// Title of the roadmap item.
    pub title: String,

    /// Status: idea, planned, active, completed, cancelled.
    pub status: String,
}

impl BrainCard {
    /// Create an empty Brain Card shell for a given node.
    pub fn empty(title: impl Into<String>, node_type: NodeType) -> Self {
        Self {
            title: title.into(),
            node_type,
            purpose: String::new(),
            status: "Active".into(),
            dependencies: Vec::new(),
            related_nodes: Vec::new(),
            files: Vec::new(),
            apis: Vec::new(),
            database_tables: Vec::new(),
            history: Vec::new(),
            roadmap: Vec::new(),
            known_issues: Vec::new(),
            risk_score: 0,
            confidence: Confidence::default(),
            last_updated: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_brain_card() {
        let card = BrainCard::empty("Wallet", NodeType::Feature);
        assert_eq!(card.title, "Wallet");
        assert_eq!(card.node_type, NodeType::Feature);
        assert!(card.dependencies.is_empty());
        assert!(card.known_issues.is_empty());
        assert_eq!(card.risk_score, 0);
    }

    #[test]
    fn brain_card_serde_roundtrip() {
        let card = BrainCard::empty("Auth Module", NodeType::Module);
        let json = serde_json::to_string(&card).unwrap();
        let deserialized: BrainCard = serde_json::from_str(&json).unwrap();
        assert_eq!(card.title, deserialized.title);
        assert_eq!(card.node_type, deserialized.node_type);
    }
}
