# Debugging Guide

When dealing with asynchronous Rust and SQL transactions, debugging can sometimes be tricky.

## 1. Rust Backtraces
If a test panics or you encounter an `unwrap()` failure, run the process with the environment variable `RUST_BACKTRACE=1` to see exactly which line of code failed.

**Linux / macOS:**
```bash
RUST_BACKTRACE=1 cargo test
```

**Windows (PowerShell):**
```powershell
$env:RUST_BACKTRACE=1; cargo test
```

## 2. SQLx Logging
If you need to see the exact SQL queries being executed against the SQLite database, you can enable logging. However, since the current iteration uses in-memory SQLite and suppresses most logs to keep test output clean, you may need to add `tracing` or `env_logger` to your specific test if you are developing a new SQL query.

## 3. Investigating "Database Locked" Errors
SQLite only supports a single writer at a time. If you ever encounter a "Database is locked" error in the test suite:
- Ensure that you are not holding a `sqlx::Transaction` open indefinitely.
- Ensure that an `await` point hasn't deadlocked.
- Remember that `sqlx::SqlitePool` manages concurrent reads fine, but concurrent writes will queue.

## 4. Where to start reading code?
If you are trying to understand the system, start at:
- `crates/core/src/domain/node.rs` (To see the pure data models)
- `crates/knowledge/src/engine/knowledge_engine.rs` (To see the public API surface)
- `crates/knowledge/src/repository.rs` (To see the database layer)
