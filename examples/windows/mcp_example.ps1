# mcp_example.ps1
# Demonstrates booting the JSON-RPC Model Context Protocol server on Windows.

$ErrorActionPreference = "Stop"

Write-Host "=> Initializing workspace..."
cargo run --manifest-path crates/cli/Cargo.toml -- init

Write-Host "=> Booting MCP Server (listening on Stdio)..."
Write-Host "Press Ctrl+C to exit."
cargo run --manifest-path crates/cli/Cargo.toml -- mcp
