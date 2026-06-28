# validation_example.ps1
# Demonstrates running architectural drift detection on Windows.

$ErrorActionPreference = "Stop"

Write-Host "=> Initializing workspace..."
cargo run --manifest-path crates/cli/Cargo.toml -- init

Write-Host "=> Validating workspace architecture against physical file system..."
cargo run --manifest-path crates/cli/Cargo.toml -- validate
