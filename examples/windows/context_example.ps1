# context_example.ps1
# Demonstrates packing an LLM-optimized context prompt on Windows.

$ErrorActionPreference = "Stop"

Write-Host "=> Initializing workspace..."
cargo run --manifest-path crates/cli/Cargo.toml -- init

Write-Host "=> Gathering context for an architecture question..."
cargo run --manifest-path crates/cli/Cargo.toml -- context "What is the entrypoint to the application?"
