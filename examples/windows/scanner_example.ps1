# scanner_example.ps1
# Demonstrates reading the file system into the SQLite repository on Windows.

$ErrorActionPreference = "Stop"

Write-Host "=> Initializing workspace..."
cargo run --manifest-path crates/cli/Cargo.toml -- init

Write-Host "=> Scanning the root directory..."
cargo run --manifest-path crates/cli/Cargo.toml -- scan .
