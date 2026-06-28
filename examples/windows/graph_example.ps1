# graph_example.ps1
# Demonstrates generating a dependency/impact graph for a node on Windows.

$ErrorActionPreference = "Stop"

Write-Host "=> Initializing workspace..."
cargo run --manifest-path crates/cli/Cargo.toml -- init

Write-Host "=> Fetching graph for arbitrary query..."
cargo run --manifest-path crates/cli/Cargo.toml -- graph "main"
