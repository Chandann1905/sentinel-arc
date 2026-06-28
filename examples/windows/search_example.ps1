# search_example.ps1
# Demonstrates querying the Tantivy search index via Sentinel Arc on Windows.

$ErrorActionPreference = "Stop"

Write-Host "=> Initializing workspace..."
cargo run --manifest-path crates/cli/Cargo.toml -- init

Write-Host "=> Searching for a known component..."
cargo run --manifest-path crates/cli/Cargo.toml -- search "engine"
