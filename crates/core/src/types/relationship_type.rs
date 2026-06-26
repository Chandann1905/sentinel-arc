//! All relationship types supported by Sentinel Arc.
//!
//! Derived from DOMAIN_MODEL.md and PRD. This enum is frozen — new variants
//! require an Architecture Change Request.

use serde::{Deserialize, Serialize};
use std::fmt;

/// The type of connection between two nodes.
///
/// Relationships are first-class entities — never embedded inside business
/// logic or hidden inside metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RelationshipType {
    /// A depends on B (e.g., Wallet DEPENDS_ON Transactions).
    DependsOn,
    /// A uses B (e.g., Wallet USES wallet_transactions table).
    Uses,
    /// A calls B (e.g., OrderService CALLS WalletService).
    Calls,
    /// A implements B (e.g., wallet_service.dart IMPLEMENTS Wallet feature).
    Implements,
    /// A reads from B (e.g., WalletService READS wallets table).
    Reads,
    /// A writes to B (e.g., WalletService WRITES wallet_transactions).
    Writes,
    /// A modifies B (e.g., Cashback MODIFIES Wallet).
    Modifies,
    /// General relationship (e.g., two loosely related features).
    RelatesTo,
    /// A blocks B (e.g., Bug BLOCKS Feature).
    Blocks,
    /// A is part of B (e.g., File PART_OF Module).
    PartOf,
    /// A was created from B (e.g., Roadmap Item CREATED_FROM Decision).
    CreatedFrom,
    /// A follows B in sequence (e.g., task ordering).
    Follows,
}

impl RelationshipType {
    /// Return all known relationship types.
    pub fn all() -> &'static [RelationshipType] {
        &[
            Self::DependsOn,
            Self::Uses,
            Self::Calls,
            Self::Implements,
            Self::Reads,
            Self::Writes,
            Self::Modifies,
            Self::RelatesTo,
            Self::Blocks,
            Self::PartOf,
            Self::CreatedFrom,
            Self::Follows,
        ]
    }

    /// Whether this relationship implies a dependency direction for impact analysis.
    pub fn is_dependency(&self) -> bool {
        matches!(
            self,
            Self::DependsOn | Self::Uses | Self::Calls | Self::Reads | Self::Writes
        )
    }
}

impl fmt::Display for RelationshipType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DependsOn => write!(f, "DEPENDS_ON"),
            Self::Uses => write!(f, "USES"),
            Self::Calls => write!(f, "CALLS"),
            Self::Implements => write!(f, "IMPLEMENTS"),
            Self::Reads => write!(f, "READS"),
            Self::Writes => write!(f, "WRITES"),
            Self::Modifies => write!(f, "MODIFIES"),
            Self::RelatesTo => write!(f, "RELATES_TO"),
            Self::Blocks => write!(f, "BLOCKS"),
            Self::PartOf => write!(f, "PART_OF"),
            Self::CreatedFrom => write!(f, "CREATED_FROM"),
            Self::Follows => write!(f, "FOLLOWS"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serde_roundtrip_all_variants() {
        for rt in RelationshipType::all() {
            let json = serde_json::to_string(rt).unwrap();
            let deserialized: RelationshipType = serde_json::from_str(&json).unwrap();
            assert_eq!(*rt, deserialized, "Failed roundtrip for {rt:?}");
        }
    }

    #[test]
    fn serialization_uses_screaming_snake_case() {
        assert_eq!(
            serde_json::to_string(&RelationshipType::DependsOn).unwrap(),
            "\"DEPENDS_ON\""
        );
        assert_eq!(
            serde_json::to_string(&RelationshipType::PartOf).unwrap(),
            "\"PART_OF\""
        );
        assert_eq!(
            serde_json::to_string(&RelationshipType::CreatedFrom).unwrap(),
            "\"CREATED_FROM\""
        );
    }

    #[test]
    fn dependency_classification() {
        assert!(RelationshipType::DependsOn.is_dependency());
        assert!(RelationshipType::Uses.is_dependency());
        assert!(RelationshipType::Calls.is_dependency());
        assert!(RelationshipType::Reads.is_dependency());
        assert!(RelationshipType::Writes.is_dependency());
        assert!(!RelationshipType::RelatesTo.is_dependency());
        assert!(!RelationshipType::Blocks.is_dependency());
        assert!(!RelationshipType::Follows.is_dependency());
    }

    #[test]
    fn all_returns_12_types() {
        assert_eq!(RelationshipType::all().len(), 12);
    }
}
