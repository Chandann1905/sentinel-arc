#!/usr/bin/env bash
# graph_example.sh
# Demonstrates generating a dependency/impact graph for a node.

set -e

echo "=> Initializing workspace..."
cargo run --manifest-path crates/cli/Cargo.toml -- init || true

echo "=> Fetching graph for arbitrary query..."
# In a real environment, you'd pass an exact Node ID or a clear query.
cargo run --manifest-path crates/cli/Cargo.toml -- graph "main"
