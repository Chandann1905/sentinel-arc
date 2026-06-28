# Model Context Protocol (MCP) Guide

Sentinel Arc exposes its powerful Knowledge Graph to AI assistants via the [Model Context Protocol](https://modelcontextprotocol.io/). By running the MCP server, Claude, Cursor, and other compatible clients can natively search your codebase, explore dependency graphs, and generate context.

## Protocol & Compliance
- **Supported MCP Revision:** Supports the latest stable MCP Protocol specification.
- **JSON-RPC Compliance:** Fully complies with the JSON-RPC 2.0 specification, strictly enforcing `id` matching and robust error handling (`-32600`, `-32601`).
- **Transport:** The server exclusively operates over `stdio` (Standard Input/Output) for direct subprocess integration with local AI IDEs. HTTP/SSE transports are not currently supported.
- **Read-Only Guarantee:** The server strictly operates in a read-only context. Tools and resources will never execute structural changes, file modifications, or SQLite write operations.

## Capabilities

### Tools
- `search_nodes`: Perform full-text search against the Tantivy index.
- `get_graph`: Retrieve dependency and impact graphs for a specific node.
- `generate_context`: Ask the AI to build an LLM-optimized context payload for a given intent.
- `validate_architecture`: Run drift detection and health checks on the workspace.

### Resources
- `arc://architecture`: Fetch a complete Graph Projection of your project.
- `arc://rules`: Read active workspace rules.
- `arc://validation/report`: Get real-time validation and drift reports.
- `arc://timeline/global`: Get the latest 50 global events.

### Prompt Templates
- `Architecture Review`: Analyzes the global repository structure.
- `Code Review`: Inspects isolated nodes for logical flaws.
- `Bug Triage`: Generates historical timeline context for debugging.

## Concurrency, Timeouts & Cancellation
The MCP server is fully asynchronous, multiplexing multiple concurrent requests. 
- **Timeouts:** All operations possess a strict **60-second timeout**. If a request exceeds this duration, a JSON-RPC error `-32000 (Request timed out)` is safely returned.
- **Cancellation:** Client-side UI cancellations map perfectly to `notifications/cancelled`. Sentinel Arc intercepts these and gracefully drops the background Tokio tasks without hanging.

## Setup Instructions

### Claude Desktop Integration
Add the server configuration to your Claude Desktop config file.
- **macOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`
- **Windows**: `%APPDATA%\Claude\claude_desktop_config.json`

```json
{
  "mcpServers": {
    "sentinel-arc": {
      "command": "sentinel-cli",
      "args": ["mcp"],
      "env": {}
    }
  }
}
```

### Cursor Integration
Cursor supports per-workspace MCP servers. In the root of your project, add or edit `.cursor/mcp.json`:

```json
{
  "mcpServers": {
    "sentinel-arc": {
      "command": "sentinel-cli",
      "args": ["mcp"]
    }
  }
}
```
Reload your Cursor window. The tools will become available inside Composer.
