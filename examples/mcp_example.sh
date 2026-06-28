#!/usr/bin/env bash
# mcp_example.sh
# Demonstrates booting the JSON-RPC Model Context Protocol server.
# Because it runs indefinitely waiting for stdio, we just print the instructions.

set -e

echo "=> Initializing workspace..."
cargo run --manifest-path crates/cli/Cargo.toml -- init || true

echo "=> Booting MCP Server (listening on Stdio)..."
echo "Press Ctrl+C to exit."
cargo run --manifest-path crates/cli/Cargo.toml -- mcp
