//! # Sentinel Arc Context Engine
//!
//! Generates highly compressed AI context packages and impact reports from the knowledge graph.

pub mod discovery;
pub mod engine;
pub mod enrichment;
pub mod expansion;
pub mod impact;
pub mod types;

pub use engine::ContextEngine;
pub use types::{ContextPackage, ContextRequest, ImpactReport};
