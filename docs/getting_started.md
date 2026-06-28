# Getting Started with Sentinel Arc

Welcome to Sentinel Arc! This guide will help you understand the core concepts and get your first workspace initialized.

## What is Sentinel Arc?
Sentinel Arc acts as a durable, queryable "memory" for autonomous agents and complex software repositories. Instead of an AI reading your codebase and "forgetting" its structure between sessions, Sentinel Arc scans the AST (Abstract Syntax Tree) of your code and builds a persistent graph in SQLite.

## Core Concepts
1. **Node**: The fundamental unit of knowledge. A Node can represent a Rust Function, a Python Class, a feature request, or an architectural decision.
2. **Relationship**: A directional link between two Nodes (e.g., `Function A` -> `CALLS` -> `Function B`).
3. **Event**: Every time a Node or Relationship is created, modified, or deleted, an immutable Event is recorded. This allows you to "rewind" your project's history.

## Initializing Your First Workspace

### Installation
```bash
cargo install --path crates/cli
```

### Initialize workspace
```bash
sentinel-cli init
```

### Scan project
```bash
sentinel-cli scan .
```

### Search
```bash
sentinel-cli search "KnowledgeEngine"
```

### Generate Context
```bash
sentinel-cli context "Refactor authentication"
```

### Launch MCP Server
```bash
sentinel-cli mcp
```

## Next Steps
- Learn how to query the graph using the [CLI Reference](cli.md).
- Hook up your AI assistant using the [MCP Guide](mcp.md).
