# Sentinel Arc — MCP Server Setup & Administration

This guide explains how to integrate Sentinel Arc's Model Context Protocol (MCP) server into your favorite AI development environments, manage timeouts, and troubleshoot common issues.

## 1. Claude Desktop Integration

Claude Desktop fully supports custom MCP servers.

**Configuration Path (macOS):**
`~/Library/Application Support/Claude/claude_desktop_config.json`

**Configuration Path (Windows):**
`%APPDATA%\Claude\claude_desktop_config.json`

**Configuration Payload:**
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

*Note:* If Claude fails to find the `sentinel` command, replace `"command": "sentinel"` with the absolute path to your cargo binary (e.g., `/Users/yourname/.cargo/bin/sentinel`).

---

## 2. Cursor Integration

Cursor natively supports MCP via the workspace settings.

**Configuration Path:**
`<workspace_root>/.cursor/mcp.json`

**Configuration Payload:**
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
After saving, reload the Cursor window. The `sentinel-arc` tools will appear in the Composer context menu.

---

## 3. VS Code Integration (Cline / RooCode)

VS Code extensions like Cline or RooCode interact with MCP servers through standard configurations.

**Configuration Path:**
Add to your VS Code `settings.json` (Workspace or Global):

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

---

## 4. Timeout Configuration

Sentinel Arc MCP commands are protected by rigorous timeouts to ensure that a slow graph query or heavy validation task does not permanently lock the client.

By default, operations timeout after **60 seconds**.
If a timeout occurs, you will receive an RPC error with code `-32000` (`Request timed out`).

Currently, timeouts are fixed at 60s per instance, but they will be exposed via `.sentinel/config.toml` in the upcoming v1.0 release.

---

## 5. Concurrency and Cancellation

The MCP server handles requests concurrently by multiplexing JSON-RPC invocations over Stdio using `tokio` multi-threading. 

If you issue a cancellation request from your client (such as closing a context window in Cursor while it is loading), the client emits a `notifications/cancelled` MCP notification. Sentinel Arc intercepts this message, correlates it by `requestId`, and cleanly aborts the underlying spawned operation.

---

## 6. Troubleshooting Common Issues

### Issue: "Server Hangs / Freezes"
**Cause:** The graph database may be locked, or an infinite loop occurred during context generation.
**Solution:** The 60-second timeout will automatically abort the operation. Try rebuilding the graph using `sentinel doctor`.

### Issue: "Method not found"
**Cause:** The client attempted an unsupported tool or resource.
**Solution:** Ensure your client is using protocol version `2024-11-05` and correctly parsing `tools/list`.

### Issue: "Invalid Request: missing id"
**Cause:** The client issued a functional request (e.g. `tools/call`) but formatted it as a Notification (omitting the `id`).
**Solution:** Adhere to the JSON-RPC 2.0 specification; only operations lacking an `id` that expect no response (like `notifications/initialized`) are processed without an ID.
