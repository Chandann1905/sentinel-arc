#!/usr/bin/env bash
# scanner_example.sh
# Demonstrates reading the file system into the SQLite repository using Tree-sitter.

set -e

echo "=> Initializing workspace..."
cargo run --manifest-path crates/cli/Cargo.toml -- init || true

echo "=> Scanning the root directory..."
cargo run --manifest-path crates/cli/Cargo.toml -- scan .
