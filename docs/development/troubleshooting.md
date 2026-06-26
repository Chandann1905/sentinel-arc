# Troubleshooting Guide

Here are common errors you might encounter while developing Sentinel Arc, and how to resolve them.

## 1. "error: linker `cc` not found"
If you see this during `cargo build`, your system lacks a C compiler. `sqlx` requires a C compiler to build the bundled SQLite C source code.
- **Windows**: Install Visual Studio C++ Build Tools.
- **macOS**: Run `xcode-select --install`.
- **Ubuntu/Debian**: Run `sudo apt-get install build-essential`.

## 2. "Database is locked" (SQLite)
If a test panics or hangs with a `Database is locked` error, it means an asynchronous SQLite transaction was left open while another thread attempted to write.
- Ensure all `sqlx::Transaction` objects are explicitly committed (`tx.commit().await?`) or dropped.
- In tests, ensure you are using the in-memory database configuration which properly serializes connections.

## 3. "missing field `xyz`" during Serde Deserialization
If you added a new field to a domain model (e.g., `Node`), old JSON payloads stored in the SQLite `events` or `metadata` columns will fail to deserialize.
- Always use `#[serde(default)]` when adding new fields to existing models to maintain backward compatibility with historical database entries.

## 4. Unused Code Warnings (`dead_code`)
If you add a new query to `NodeStore` but haven't wired it up to the `KnowledgeEngine` yet, Clippy will flag it as `dead_code` and fail the CI build.
- Temporarily add `#[allow(dead_code)]` above the method if it is foundational, or simply wire it up to the public API immediately.
