# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2026-06-28
### Added
- **Sentinel CLI** (`sentinel-cli`): Production developer CLI providing the primary entry point for workspace management.
  - `sentinel-cli init`: Initialize a new Sentinel Arc workspace (`.sentinel/`, SQLite database, search index).
  - `sentinel-cli doctor`: Verify environment health, workspace structure, and database integrity.
  - `sentinel-cli scan [PATH]`: Incrementally scan source code with Tree-sitter and populate the knowledge graph.
  - `sentinel-cli search <QUERY>`: Full-text search across the knowledge graph with node-type filtering and JSON output.
  - `sentinel-cli graph <QUERY>`: Visualize dependency trees and impact graphs as ASCII trees.
  - `sentinel-cli context <INTENT>`: Generate LLM-ready context packages from the knowledge graph.
  - `sentinel-cli validate`: Run all architectural integrity validators and drift detection.
  - `sentinel-cli stats`: Display workspace statistics (nodes, relationships, events, rules, database size).
  - `sentinel-cli rebuild-index`: Force a full rebuild of the Tantivy search index from SQLite.
  - `sentinel-cli version`: Display version and build information.
  - `sentinel-cli completion <SHELL>`: Generate shell completion scripts (bash, zsh, fish, PowerShell, elvish).
- **CLI Integration Tests**: End-to-end tests using `assert_cmd` covering `init`, `doctor`, and `version` commands.
- **CLI Documentation**: Comprehensive reference guide at `docs/cli/README.md`.
- **CLI Examples**: Usage examples at `examples/cli/`.

### Changed
- Updated `README.md` with CLI installation instructions, Quick Start guide, and full command reference table.
- Updated architecture diagram to include CLI and all engine layers.

## [0.2.0] - 2026-06-26
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
