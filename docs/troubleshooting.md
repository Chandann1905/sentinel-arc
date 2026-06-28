# Troubleshooting Guide

This guide addresses common issues encountered when initializing, scanning, or interacting with Sentinel Arc.

## 1. Permission Denied
**Symptom:** `Permission denied (os error 13)` when running `sentinel-cli init` or `scan`.
**Cause:** The user does not have write access to the current directory to create `.sentinel/`.
**Resolution:** Ensure you own the directory or run the command with elevated privileges.

## 2. Unsupported Rust Version
**Symptom:** `error: package requires rustc 1.85.0 or newer` during `cargo install`.
**Cause:** Your local Rust toolchain is outdated.
**Resolution:** Run `rustup update stable` to fetch the latest Rust compiler.

## 3. Corrupted Tantivy Index
**Symptom:** Empty search results or index panic errors.
**Cause:** The Tantivy search index is out of sync with the SQLite database, or was modified externally.
**Resolution:** Run `sentinel-cli rebuild-index` to forcefully sync Tantivy with SQLite from scratch.

## 4. Missing Workspace
**Symptom:** `Error: Workspace not initialized`.
**Cause:** You are running a command like `sentinel-cli scan` in a directory without a `.sentinel/` folder.
**Resolution:** Run `sentinel-cli init` first.

## 5. Invalid Database Schema
**Symptom:** SQLite throws `no such table: nodes` or schema mismatch errors.
**Cause:** You are using a newer version of `sentinel-cli` against an older database.
**Resolution:** Delete the `.sentinel/` directory and re-initialize it via `sentinel-cli init`.

## 6. MCP Timeout
**Symptom:** The AI client reports `Error -32000: Request timed out`.
**Cause:** Graph traversal or search exceeded the strict 60-second execution window.
**Resolution:** Refine your query or limit the scope of the context request.

## 7. MCP Connection Failure
**Symptom:** The AI client sits indefinitely waiting for a response, or says "Server closed connection".
**Cause:** The `sentinel-cli` binary is not in your system `PATH`, or the path specified in your MCP config is incorrect.
**Resolution:** Provide the absolute path to `sentinel-cli` in your `claude_desktop_config.json` or `.cursor/mcp.json`.

## 8. JSON-RPC Errors
**Symptom:** Your AI client reports `-32600 Invalid Request` or `-32601 Method not found`.
**Cause:** The AI client is utilizing an outdated MCP protocol version or malformed JSON payload.
**Resolution:** Ensure the client explicitly parses the `tools/list` response before attempting to call tools. Sentinel Arc strictly enforces JSON-RPC 2.0.

## 9. Build Failures
**Symptom:** Compilation fails during `cargo build` relating to `sqlite3`.
**Cause:** Missing C compiler for building `libsqlite3-sys` from source.
**Resolution:** Install `build-essential` (Linux) or Visual Studio Build Tools (Windows).
