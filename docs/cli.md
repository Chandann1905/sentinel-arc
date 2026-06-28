# Command Line Interface (CLI) Reference

The `sentinel-cli` acts as the primary developer interface to the Sentinel Arc engines.

## Global Flags
- `--verbose` (`-v`): Enable debug logging.
- `--help` (`-h`): Print detailed help information.

---

## Workspace Management

### `init`
Initializes a new Sentinel Arc workspace in the current directory.
```bash
sentinel-cli init
```

### `doctor`
Validates the health of the workspace, checking database integrity, schema versions, and search index availability.
```bash
sentinel-cli doctor
```

### `stats`
Displays aggregated metrics including total Node count, Relationship connections, and Event logs.
```bash
sentinel-cli stats
```

---

## Data Ingestion & Validation

### `scan <PATH>`
Parses source files at the given path using Tree-sitter and synchronizes them into the Knowledge Graph.
```bash
sentinel-cli scan ./src
```

### `validate`
Runs the Validation Engine to compare the in-memory graph against the physical file system, detecting architectural drift or orphaned nodes.
```bash
sentinel-cli validate
```

---

## Querying

### `search <QUERY>`
Executes a full-text search against the Tantivy index.
```bash
sentinel-cli search "authentication middleware"
```

### `graph <NODE_ID_OR_QUERY>`
Renders an ASCII dependency tree and impact graph for a specific node, pulling data from the `petgraph` projection.
```bash
sentinel-cli graph "MyStruct"
```

### `context <INTENT>`
Generates a token-optimized context package designed to be fed into LLMs.
```bash
sentinel-cli context "Refactor the database connection logic"
```

---

## Utilities

### `mcp`
Starts the JSON-RPC 2.0 Model Context Protocol server over standard input/output.
```bash
sentinel-cli mcp
```

### `completion <SHELL>`
Generates shell autocompletion scripts.
```bash
sentinel-cli completion bash > ~/.local/share/bash-completion/completions/sentinel-cli
```

---

## Technical Details

### Exit Codes
- `0`: Success. The command completed without error.
- `1`: General Error (e.g., IO error, bad configuration).
- `2`: Validation Failure (e.g., drift detected during `validate`).

### Environment Variables
Sentinel Arc currently exposes no environment-variable configuration.

### Configuration
All workspace-level configuration and database files live exclusively within the `.sentinel/` directory created at the root of your project during `init`.

### Logging
You can enable verbose/debug output across all commands by passing the global `--verbose` (or `-v`) flag. This will print detailed engine traces and transaction states to `stderr`.

### JSON Output
Currently, the CLI does not natively support structured JSON output for subcommands. For programmatic interaction and structured JSON data, use the Model Context Protocol (`mcp`) server.
