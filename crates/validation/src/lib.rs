//! # Sentinel Arc Validation Engine
//!
//! Evaluates the current project structure against architectural constraints and rules.

pub mod context;
pub mod engine;
pub mod types;
pub mod validators;

pub use engine::ValidationEngine;
pub use types::{Severity, ValidationIssue, ValidationReport};
