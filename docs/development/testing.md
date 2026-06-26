# Testing Strategy

Sentinel Arc has a rigorous test suite designed to be executed instantly without requiring any external dependencies or docker-compose setups.

## Running Tests

To run the entire test suite across the workspace:
```bash
cargo test --workspace
```

To run tests for a specific crate (e.g., knowledge):
```bash
cargo test -p sentinel-arc-knowledge
```

## In-Memory Database Architecture

We use `sqlx` with the SQLite driver configured to run entirely in memory for tests.
In `crates/knowledge/src/test_utils.rs`, we define a helper function `setup_engines()` that:
1. Opens an in-memory SQLite pool (`sqlite::memory:`).
2. Runs all database migrations (`migrations/*.sql`) automatically.
3. Initializes the `KnowledgeRepository` and `KnowledgeEngine`.

This guarantees that:
- Tests run extremely fast.
- Tests are isolated from one another (no shared state collisions).
- Developers do not need to install or run a local SQL server.

## Writing New Tests

When adding a feature, locate the appropriate test module (usually at the bottom of the source file, enclosed in `#[cfg(test)] mod tests`).

Use the `setup_engines().await` utility for integration tests:
```rust
#[tokio::test]
async fn test_my_new_feature() {
    let engine = setup_engines().await;
    // ... write assertions ...
}
```
