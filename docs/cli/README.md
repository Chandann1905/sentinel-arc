# Sentinel Arc CLI

The `sentinel-cli` is the official Developer Experience (DX) interface for interacting with a Sentinel Arc workspace.

## Installation

```bash
cargo install --path crates/cli
```

## Commands

### `sentinel init`
Initializes a new Sentinel Arc workspace in the current directory. Creates the `.sentinel/` directory, initializes the SQLite database, and creates the Search Index.

### `sentinel doctor`
Analyzes the workspace and environment to verify all prerequisites are met and the workspace is healthy.

### `sentinel scan [PATH]`
Runs the Scanner Engine to incrementally scan the project files and extract structural knowledge (functions, modules, types).

### `sentinel search <QUERY>`
Full-text search across the Knowledge Engine. Use `--node-type <TYPE>` to filter or `--json` for machine-readable output.

### `sentinel graph <NODE_ID|QUERY>`
Visualizes the relationship graph (upstream impact and downstream dependencies) for a given node using an ASCII tree.

### `sentinel context <INTENT>`
Generates a context package containing relevant nodes, rules, and events tailored to a specific user intent.

### `sentinel validate`
Runs all architectural and integrity validators against the workspace and prints a validation report. Exits with code 1 if errors or warnings are found.

### `sentinel stats`
Prints summary statistics for the workspace, including total nodes, relationships, events, rules, and database size.

### `sentinel rebuild-index`
Forces a full rebuild of the Tantivy search index using the primary SQLite source of truth.

## Architecture

The CLI is a thin wrapper that *only* consumes the public Rust APIs of the Sentinel Arc engines (`KnowledgeEngine`, `ScannerEngine`, `ContextEngine`, etc.). It does not contain direct database access, bypassed logic, or manual data manipulation.
