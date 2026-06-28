#!/usr/bin/env bash
# context_example.sh
# Demonstrates packing an LLM-optimized context prompt via Context Engine.

set -e

echo "=> Initializing workspace..."
cargo run --manifest-path crates/cli/Cargo.toml -- init || true

echo "=> Gathering context for an architecture question..."
cargo run --manifest-path crates/cli/Cargo.toml -- context "What is the entrypoint to the application?"
