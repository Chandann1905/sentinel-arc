# Sentinel Arc — MCP Server

This crate provides the Model Context Protocol (MCP) server for Sentinel Arc.
It exposes the internal capabilities of Sentinel Arc (Knowledge, Context, Validation, Timeline) as standard MCP tools, resources, and prompts, enabling AI agents and assistants to deeply understand and interact with your workspace.

## Features

- **Strictly Read-Only**: The MCP server never modifies your database. It relies entirely on existing read operations from the engines.
- **Tools**:
  - `search_nodes`: Search the graph via Tantivy full-text search.
  - `get_node`: Retrieve detailed attributes of any node.
  - `generate_context`: Produce an AI-optimized context package via graph topology traversal.
  - `generate_timeline`: Generate event timelines for the workspace or specific nodes.
- **Resources**:
  - `arc://architecture`: Fetch a complete Graph Projection of your project.
  - `arc://rules`: Read active workspace rules.
  - `arc://validation/report`: Get real-time validation and drift reports.
  - `arc://timeline/global`: Get the latest 50 global events.
- **Prompts**:
  - `Architecture Review`
  - `Code Review`
  - `Bug Triage`

## Client Setup

### Claude Desktop

Add this to your `claude_desktop_config.json`:
```json
{
  "mcpServers": {
    "sentinel-arc": {
      "command": "sentinel",
      "args": ["mcp"],
      "env": {}
    }
  }
}
```

### Cursor

Add this to your workspace `.cursor/mcp.json`:
```json
{
  "mcpServers": {
    "sentinel-arc": {
      "command": "sentinel",
      "args": ["mcp"]
    }
  }
}
```

### VS Code (Cline / RooCode)

In your VS Code global or workspace settings (`settings.json`), configure your preferred MCP extension:
```json
{
  "mcp.servers": {
    "sentinel-arc": {
      "command": "sentinel",
      "args": ["mcp"]
    }
  }
}
```

## Troubleshooting

- **Server hangs/freezes:** The server incorporates default 60-second timeouts. If a graph query takes too long, it will return an RPC error `-32000` (Request timed out) instead of blocking the transport layer. Ensure your graph database is healthy if timeouts happen frequently.
- **Tools not appearing:** Make sure the `sentinel` binary is globally available in your system's `PATH`. If not, replace `"command": "sentinel"` with the absolute path to your binary (e.g. `"/Users/name/.cargo/bin/sentinel"`).
- **Concurrency issues:** As of this version, request multiplexing handles overlapping JSON-RPC calls safely via `tokio` multi-threading. You can abort operations inside your client, which triggers a `notifications/cancelled` to abort the underlying spawned task cleanly.

## Starting the Server Manually

You can start the server manually for debugging:

```bash
sentinel mcp
```

The server uses the `stdio` transport by default, reading JSON-RPC 2.0 messages from standard input and writing responses to standard output.
