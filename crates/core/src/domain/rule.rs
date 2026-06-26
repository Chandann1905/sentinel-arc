//! The Rule entity — data-driven constraints.
//!
//! Rules are data, not code. They define project constraints like
//! "backend=supabase" or "architecture=clean_architecture" and are
//! stored in the database, not hardcoded.

use serde::{Deserialize, Serialize};

use crate::types::ids::RuleId;

/// Severity level of a rule violation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum RuleSeverity {
    /// Informational only.
    Info,
    /// Warning — should be addressed.
    #[default]
    Warning,
    /// Error — must be fixed.
    Error,
    /// Critical — blocks progress.
    Critical,
}

impl std::fmt::Display for RuleSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Info => write!(f, "Info"),
            Self::Warning => write!(f, "Warning"),
            Self::Error => write!(f, "Error"),
            Self::Critical => write!(f, "Critical"),
        }
    }
}

/// Category of a rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuleCategory {
    /// Architecture-level constraint.
    Architecture,
    /// Technology choice constraint.
    Technology,
    /// Naming convention constraint.
    Naming,
    /// Dependency constraint.
    Dependency,
    /// Validation constraint.
    Validation,
}

impl std::fmt::Display for RuleCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Architecture => write!(f, "Architecture"),
            Self::Technology => write!(f, "Technology"),
            Self::Naming => write!(f, "Naming"),
            Self::Dependency => write!(f, "Dependency"),
            Self::Validation => write!(f, "Validation"),
        }
    }
}

/// A data-driven rule that defines a project constraint.
///
/// Rules are never hardcoded in logic. They are loaded from the database
/// and evaluated by the Rule Engine. Schema defined in TRD.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rule {
    /// Unique identifier.
    pub id: RuleId,

    /// Human-readable name (e.g., "backend_technology").
    pub name: String,

    /// Category of this rule.
    pub category: RuleCategory,

    /// The rule value (e.g., "supabase", "clean_architecture").
    pub value: String,

    /// Severity if this rule is violated.
    pub severity: RuleSeverity,

    /// Whether this rule is currently active.
    pub enabled: bool,
}

impl Rule {
    /// Create a new enabled rule.
    pub fn new(name: impl Into<String>, category: RuleCategory, value: impl Into<String>) -> Self {
        Self {
            id: RuleId::new(),
            name: name.into(),
            category,
            value: value.into(),
            severity: RuleSeverity::default(),
            enabled: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_rule_is_enabled_by_default() {
        let rule = Rule::new("backend", RuleCategory::Technology, "supabase");
        assert!(rule.enabled);
        assert_eq!(rule.severity, RuleSeverity::Warning);
    }

    #[test]
    fn rule_serde_roundtrip() {
        let rule = Rule::new(
            "architecture",
            RuleCategory::Architecture,
            "clean_architecture",
        );
        let json = serde_json::to_string(&rule).unwrap();
        let deserialized: Rule = serde_json::from_str(&json).unwrap();
        assert_eq!(rule.id, deserialized.id);
        assert_eq!(rule.name, deserialized.name);
        assert_eq!(rule.category, deserialized.category);
        assert_eq!(rule.value, deserialized.value);
    }

    #[test]
    fn severity_serde_roundtrip() {
        let severities = [
            RuleSeverity::Info,
            RuleSeverity::Warning,
            RuleSeverity::Error,
            RuleSeverity::Critical,
        ];
        for s in &severities {
            let json = serde_json::to_string(s).unwrap();
            let deserialized: RuleSeverity = serde_json::from_str(&json).unwrap();
            assert_eq!(*s, deserialized);
        }
    }
}
