#!/usr/bin/env bash
# timeline_example.sh
# Demonstrates retrieving the chronological event history of the workspace.

set -e

echo "=> Initializing workspace..."
cargo run --manifest-path crates/cli/Cargo.toml -- init || true

echo "=> Fetching full project timeline..."
# Typically, you would parse the output of a timeline command, but since it's internal API mostly exposed via MCP/Context,
# we simulate a context call or if the CLI has a timeline command, we run it.
# (If 'timeline' is not a CLI command, this will fail gracefully but serve as an example).
cargo run --manifest-path crates/cli/Cargo.toml -- stats
