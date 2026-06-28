# Sentinel Arc v1.0.0 Release Notes

We are thrilled to announce the General Availability (GA) release of **Sentinel Arc v1.0.0**. This milestone marks the official promotion of Sentinel Arc to a production-ready, highly reliable state-management and event-sourcing engine.

## Major Features

- **Knowledge Engine**: Local, fast, SQLite-based knowledge graph with complete CRUD capabilities.
- **Relationship Engine**: Expressive multi-directional relationships between nodes.
- **Event Sourcing**: Immutable append-only audit trail guaranteeing a perfect historical record of all state transitions.
- **Rules Engine**: Embedded rule evaluator enforcing architectural constraints.

## Architecture Overview
Sentinel Arc is built using a strict Domain-Driven Design (DDD) architecture in Rust. It utilizes a public facade (`KnowledgeEngine`) that encapsulates a highly concurrent SQLx SQLite storage backend and a Tantivy full-text search engine. 

## Command Line Interface (CLI)
The `sentinel-cli` executable is now the definitive standard interface for all Sentinel Arc operations:
- **Initialization**: Effortlessly set up a `.sentinel/` workspace (`sentinel-cli init`).
- **Diagnostics**: Health check and environment verification (`sentinel-cli doctor`).

## Core Subsystems

### Model Context Protocol (MCP) Server
`sentinel-cli mcp` seamlessly exposes Sentinel Arc's capabilities to AI assistants (Claude Desktop, Cursor, VS Code).
- Supports JSON-RPC 2.0 over standard I/O.
- Exposes workspace metrics via `tools/list` and `resources/list`.

### Search
The integrated Tantivy full-text search engine allows millisecond-latency fuzzy searching across the entire workspace graph via `sentinel-cli search`.

### Scanner
Automated ingest pipeline to scan Rust source code using Tree-sitter, parsing structural representations directly into the graph (`sentinel-cli scan .`).

### Validation
Drift detection engine capable of evaluating the current graph state against the enforced rule definitions (`sentinel-cli validate`).

### Timeline
Chronological reconstruction of events across the workspace, offering insight into historical development patterns (`sentinel-cli timeline`).

### Graph
Visual network mapping to identify hotspots, coupling, and complex node relationships (`sentinel-cli graph`).

### Context
Intelligent intent-based compilation of LLM prompts and context packages dynamically sourced from the active graph (`sentinel-cli context`).

## Breaking Changes
- Replaced the legacy `./.brain/` directory with a standardized `./.sentinel/` hidden directory to encapsulate the database and search indices.
- Standardized the binary name across all matrices to `sentinel-cli`.

## Known Limitations
- The `sentinel-cli scan` command currently supports only the Rust programming language via Tree-sitter.

## Upgrade Notes
If upgrading from a `v0.x.x` pre-release:
1. Ensure Sentinel Arc processes are stopped.
2. Delete the deprecated `.brain/` directory in your workspace.
3. Run `sentinel-cli init` and `sentinel-cli scan .` to regenerate your workspace state under the new `.sentinel/` standard.
