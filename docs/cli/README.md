# Sentinel Arc CLI

The `sentinel-cli` is the official Developer Experience (DX) interface for interacting with a Sentinel Arc workspace. It is a thin wrapper that *only* consumes the public Rust APIs of the underlying engines. It never accesses the SQLite database or Tantivy index directly.

## Installation

### From Source (Recommended)
```bash
cargo install --path crates/cli
```

### Development Build
```bash
cargo build -p sentinel-cli
# Binary available at target/debug/sentinel-cli
```

### Release Build
```bash
cargo build -p sentinel-cli --release
# Binary available at target/release/sentinel-cli
```

---

## Commands

### `sentinel-cli init`
Initializes a new Sentinel Arc workspace in the current directory.

Creates:
- `.sentinel/` directory
- SQLite database (`.sentinel/knowledge.db`)
- Tantivy search index

```bash
sentinel-cli init
```

---

### `sentinel-cli doctor`
Analyzes the workspace and environment to verify all prerequisites are met and the workspace is healthy.

Checks:
- Rust toolchain version
- `.sentinel/` directory existence
- Database file presence
- Search index directory

```bash
sentinel-cli doctor
```

---

### `sentinel-cli scan [PATH]`
Runs the Scanner Engine to incrementally scan the project files and extract structural knowledge.

Extracts:
- Functions, structs, enums, traits, type aliases, constants
- Module hierarchy and file relationships
- Uses Tree-sitter for language-aware parsing

```bash
# Scan current directory
sentinel-cli scan

# Scan a specific path
sentinel-cli scan ./src
```

---

### `sentinel-cli search <QUERY>`
Full-text search across the Knowledge Engine using Tantivy.

Options:
- `--node-type <TYPE>`: Filter results by node type (e.g., `function`, `struct`)
- `--limit <N>`: Maximum number of results (default: 10)
- `--json`: Output results as JSON for machine consumption

```bash
# Basic search
sentinel-cli search "KnowledgeEngine"

# Filter by type
sentinel-cli search "create" --node-type function

# JSON output with limit
sentinel-cli search "Node" --limit 5 --json
```

---

### `sentinel-cli graph <NODE_ID|QUERY>`
Visualizes the relationship graph (upstream impact and downstream dependencies) for a given node using an ASCII tree.

- Accepts a Node ID (UUID) or a search query (title match)
- Displays downstream dependencies and upstream impact
- Detects and reports circular references

```bash
# By title
sentinel-cli graph "KnowledgeEngine"

# By Node ID
sentinel-cli graph "550e8400-e29b-41d4-a716-446655440000"
```

---

### `sentinel-cli context <INTENT>`
Generates a context package containing relevant nodes, rules, and events tailored to a specific user intent. Designed for LLM integration.

Options:
- `--json`: Output the context package as JSON

```bash
# Generate context for a refactoring intent
sentinel-cli context "refactor the authentication module"

# JSON output for programmatic consumption
sentinel-cli context "add error handling to the parser" --json
```

---

### `sentinel-cli validate`
Runs all architectural and integrity validators against the workspace. Prints a validation report.

Checks:
- Broken relationships (referencing non-existent nodes)
- Circular dependencies
- Orphaned nodes
- Schema drift

Exit codes:
- `0`: No issues found
- `1`: Warnings or errors detected

```bash
sentinel-cli validate
```

---

### `sentinel-cli stats`
Prints summary statistics for the workspace.

Displays:
- Total nodes, relationships, events, and rules
- Database file size
- Search index size

```bash
sentinel-cli stats
```

---

### `sentinel-cli rebuild-index`
Forces a full rebuild of the Tantivy search index using the primary SQLite source of truth.

Use when:
- The search index appears out of sync
- After manual database modifications
- After recovering from a crash

```bash
sentinel-cli rebuild-index
```

---

### `sentinel-cli version`
Displays the Sentinel Arc version and Rust compiler version used to build the binary.

```bash
sentinel-cli version
```

---

### `sentinel-cli completion <SHELL>`
Generates shell completion scripts for the specified shell.

Supported shells: `bash`, `zsh`, `fish`, `powershell`, `elvish`

```bash
# Bash
sentinel-cli completion bash > ~/.local/share/bash-completion/completions/sentinel-cli

# Zsh
sentinel-cli completion zsh > ~/.zfunc/_sentinel-cli

# Fish
sentinel-cli completion fish > ~/.config/fish/completions/sentinel-cli.fish

# PowerShell
sentinel-cli completion powershell >> $PROFILE
```

---

## Architecture

```
Consumer / CLI
      │
      ▼
KnowledgeEngine (facade)
      │
      ├── NodeEngine
      ├── EventEngine
      ├── RelationshipEngine
      ├── RuleEngine
      ├── SearchEngine → Tantivy Index
      ├── GraphEngine → Petgraph Projection
      ├── ScannerEngine → Tree-sitter
      ├── ContextEngine
      └── ValidationEngine
      │
      ▼
KnowledgeRepository
      │
      ▼
SQLite Database
```

The CLI is a **presentation-only** layer. It:
- Never accesses SQLite directly
- Never accesses Tantivy directly
- Never contains business logic
- Never duplicates engine logic
- Only calls public engine APIs
