#!/usr/bin/env bash
# search_example.sh
# Demonstrates querying the Tantivy search index via Sentinel Arc.

set -e

echo "=> Initializing workspace..."
cargo run --manifest-path crates/cli/Cargo.toml -- init || true

echo "=> Searching for a known component..."
cargo run --manifest-path crates/cli/Cargo.toml -- search "engine"
