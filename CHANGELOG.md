# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-06-26
### Added
- **Core Framework**: Foundation types (`NodeId`, `RelationshipId`), domain primitives, and system boundaries.
- **Storage Layer**: SQLite integration for Node, Event, Rule, and Relationship data.
- **Engines**: 
  - `NodeEngine` (node lifecycle)
  - `EventEngine` (event streams)
  - `RelationshipEngine` (dependency graphs)
  - `RuleEngine` (architectural constraints)
  - `KnowledgeEngine` (unified DDD facade)
  - `SearchEngine` (Tantivy full-text index)
  - `GraphEngine` (Petgraph in-memory topological projection and paths)
  - `ScannerEngine` (Tree-sitter source code ingestion)
  - `ContextEngine` (Graph traversal and intent resolution for LLMs)
  - `ValidationEngine` (Integrity checks and drift detection)

### Security
- Stateless architecture guarantees with strictly bounded memory allocations during graph traversals.
