# Building & Compiling

Sentinel Arc is organized as a **Cargo Workspace**. This means multiple crates (`sentinel-arc-core` and `sentinel-arc-knowledge`) are managed under a single unified compilation tree.

## Standard Build

To build the entire workspace in debug mode:
```bash
cargo build --workspace
```
*Note: Because we use the `sqlx` bundled SQLite feature, the very first compilation will take a little longer to compile the C bindings. Subsequent builds are heavily cached.*

## Release Build

If you are benchmarking or preparing a production binary, use the release profile:
```bash
cargo build --workspace --release
```

## IDE Support
We highly recommend using **VS Code** with the **rust-analyzer** extension, or **IntelliJ Rust**. Because this is a standard Cargo workspace, `rust-analyzer` will automatically detect `crates/core` and `crates/knowledge` without any special configuration.

## Formatting and Linting
Before submitting a Pull Request, you must ensure your code complies with our formatting and linting rules.

**Format your code:**
```bash
cargo fmt --all
```

**Run the Linter (Clippy):**
```bash
cargo clippy --workspace --all-targets --all-features -- -D warnings
```
We run with `-D warnings` in CI, meaning any warning will fail the build. Please fix all clippy warnings locally.
