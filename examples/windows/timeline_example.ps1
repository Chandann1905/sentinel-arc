# timeline_example.ps1
# Demonstrates retrieving the chronological event history of the workspace on Windows.

$ErrorActionPreference = "Stop"

Write-Host "=> Initializing workspace..."
cargo run --manifest-path crates/cli/Cargo.toml -- init

Write-Host "=> Fetching full project timeline..."
cargo run --manifest-path crates/cli/Cargo.toml -- stats
