#!/usr/bin/env bash
# validation_example.sh
# Demonstrates running architectural drift detection via the Validation Engine.

set -e

echo "=> Initializing workspace..."
cargo run --manifest-path crates/cli/Cargo.toml -- init || true

echo "=> Validating workspace architecture against physical file system..."
cargo run --manifest-path crates/cli/Cargo.toml -- validate
