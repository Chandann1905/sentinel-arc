//! All node types supported by Project Brain.
//!
//! Derived from DOMAIN_MODEL.md. This enum is frozen — new variants
//! require an Architecture Change Request.

use serde::{Deserialize, Serialize};
use std::fmt;

/// The type of a knowledge node.
///
/// Every important entity in the system is represented as a `Node` with one
/// of these types. See the Domain Model for definitions of each type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeType {
    /// Top-level container (e.g., "Yum Point").
    Project,
    /// Connected Git repository (e.g., "frontend", "backend").
    Repository,
    /// Business functionality (e.g., "Wallet", "Orders").
    Feature,
    /// Technical grouping (e.g., "Authentication Module").
    Module,
    /// Source code file (e.g., "wallet_service.dart").
    File,
    /// Code-level behavior (e.g., "createTransaction()").
    Function,
    /// External or internal interface (e.g., "POST /wallet").
    Api,
    /// Storage structure (e.g., "wallet_transactions").
    DatabaseTable,
    /// Architecture Decision Record (e.g., "ADR-004").
    Decision,
    /// Known issue (e.g., "Wallet race condition").
    Bug,
    /// Work item (e.g., "Implement cashback").
    Task,
    /// Future work (e.g., "Referral rewards").
    RoadmapItem,
    /// Knowledge note.
    Note,
    /// Formal documentation (e.g., PRD, TRD).
    Document,
    /// A person associated with the project.
    Person,
}

impl NodeType {
    /// Return the node level (L1–L5) for hierarchy-aware types.
    ///
    /// Not all node types map to a fixed level — only the hierarchical ones
    /// defined in the Domain Model.
    pub fn level(&self) -> Option<u8> {
        match self {
            Self::Project => Some(1),
            Self::Feature => Some(2),
            Self::Module => Some(3),
            Self::File => Some(4),
            Self::Function => Some(5),
            _ => None,
        }
    }

    /// Return all known node types.
    pub fn all() -> &'static [NodeType] {
        &[
            Self::Project,
            Self::Repository,
            Self::Feature,
            Self::Module,
            Self::File,
            Self::Function,
            Self::Api,
            Self::DatabaseTable,
            Self::Decision,
            Self::Bug,
            Self::Task,
            Self::RoadmapItem,
            Self::Note,
            Self::Document,
            Self::Person,
        ]
    }
}

impl fmt::Display for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Project => write!(f, "Project"),
            Self::Repository => write!(f, "Repository"),
            Self::Feature => write!(f, "Feature"),
            Self::Module => write!(f, "Module"),
            Self::File => write!(f, "File"),
            Self::Function => write!(f, "Function"),
            Self::Api => write!(f, "API"),
            Self::DatabaseTable => write!(f, "Database Table"),
            Self::Decision => write!(f, "Decision"),
            Self::Bug => write!(f, "Bug"),
            Self::Task => write!(f, "Task"),
            Self::RoadmapItem => write!(f, "Roadmap Item"),
            Self::Note => write!(f, "Note"),
            Self::Document => write!(f, "Document"),
            Self::Person => write!(f, "Person"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serde_roundtrip_all_variants() {
        for nt in NodeType::all() {
            let json = serde_json::to_string(nt).unwrap();
            let deserialized: NodeType = serde_json::from_str(&json).unwrap();
            assert_eq!(*nt, deserialized, "Failed roundtrip for {nt:?}");
        }
    }

    #[test]
    fn serialization_uses_snake_case() {
        assert_eq!(
            serde_json::to_string(&NodeType::DatabaseTable).unwrap(),
            "\"database_table\""
        );
        assert_eq!(
            serde_json::to_string(&NodeType::RoadmapItem).unwrap(),
            "\"roadmap_item\""
        );
    }

    #[test]
    fn node_levels() {
        assert_eq!(NodeType::Project.level(), Some(1));
        assert_eq!(NodeType::Feature.level(), Some(2));
        assert_eq!(NodeType::Module.level(), Some(3));
        assert_eq!(NodeType::File.level(), Some(4));
        assert_eq!(NodeType::Function.level(), Some(5));
        assert_eq!(NodeType::Bug.level(), None);
        assert_eq!(NodeType::Note.level(), None);
    }

    #[test]
    fn all_returns_15_types() {
        assert_eq!(NodeType::all().len(), 15);
    }

    #[test]
    fn display_formatting() {
        assert_eq!(NodeType::Api.to_string(), "API");
        assert_eq!(NodeType::DatabaseTable.to_string(), "Database Table");
        assert_eq!(NodeType::RoadmapItem.to_string(), "Roadmap Item");
    }
}
