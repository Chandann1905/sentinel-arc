use sentinel_arc_core::types::ids::{NodeId, RelationshipId};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationIssue {
    pub rule_name: String,
    pub severity: Severity,
    pub message: String,
    pub node_id: Option<NodeId>,
    pub relationship_id: Option<RelationshipId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationReport {
    pub timestamp: u64,
    pub total_issues: usize,
    pub error_count: usize,
    pub warning_count: usize,
    pub info_count: usize,
    pub issues: Vec<ValidationIssue>,
}

impl Default for ValidationReport {
    fn default() -> Self {
        Self::empty()
    }
}

impl ValidationReport {
    pub fn empty() -> Self {
        Self {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            total_issues: 0,
            error_count: 0,
            warning_count: 0,
            info_count: 0,
            issues: Vec::new(),
        }
    }

    pub fn build(issues: Vec<ValidationIssue>) -> Self {
        let mut error_count = 0;
        let mut warning_count = 0;
        let mut info_count = 0;

        for issue in &issues {
            match issue.severity {
                Severity::Error => error_count += 1,
                Severity::Warning => warning_count += 1,
                Severity::Info => info_count += 1,
            }
        }

        Self {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            total_issues: issues.len(),
            error_count,
            warning_count,
            info_count,
            issues,
        }
    }
}
