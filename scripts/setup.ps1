<#
.SYNOPSIS
Sentinel Arc Bootstrap Setup Script

.DESCRIPTION
This script verifies the Rust toolchain, compiles the project, and runs the test suite.
#>

Write-Host "Sentinel Arc Setup Script" -ForegroundColor Cyan
Write-Host "===========================" -ForegroundColor Cyan

# 1. Check for Rustup/Cargo
if (!(Get-Command "cargo" -ErrorAction SilentlyContinue)) {
    Write-Host "Error: Cargo not found. Please install Rust from https://rustup.rs" -ForegroundColor Red
    exit 1
}

$rustcVersion = (rustc --version)
Write-Host "Detected Toolchain: $rustcVersion" -ForegroundColor Green

# 2. Check for C Compiler (Required for sqlx sqlite bundled)
# This is a soft check, it might fail to detect MSVC but Cargo will figure it out
if (!(Get-Command "cl" -ErrorAction SilentlyContinue) -and !(Get-Command "gcc" -ErrorAction SilentlyContinue)) {
    Write-Host "Warning: No C compiler detected. If the build fails, install Visual Studio C++ Build Tools." -ForegroundColor Yellow
}

# 3. Format Check
Write-Host "`nRunning cargo fmt..." -ForegroundColor Cyan
cargo fmt --all
if ($LASTEXITCODE -ne 0) {
    Write-Host "Failed to format code." -ForegroundColor Red
    exit $LASTEXITCODE
}

# 4. Clippy
Write-Host "`nRunning clippy..." -ForegroundColor Cyan
cargo clippy --workspace --all-targets --all-features -- -D warnings
if ($LASTEXITCODE -ne 0) {
    Write-Host "Clippy found warnings or errors. Please fix them." -ForegroundColor Red
    exit $LASTEXITCODE
}

# 5. Build
Write-Host "`nBuilding Workspace..." -ForegroundColor Cyan
cargo build --workspace
if ($LASTEXITCODE -ne 0) {
    Write-Host "Failed to build workspace." -ForegroundColor Red
    exit $LASTEXITCODE
}

# 6. Test
Write-Host "`nRunning Test Suite..." -ForegroundColor Cyan
cargo test --workspace
if ($LASTEXITCODE -ne 0) {
    Write-Host "Tests failed!" -ForegroundColor Red
    exit $LASTEXITCODE
}

Write-Host "`nSuccess! All tests passed! You are ready to contribute to Sentinel Arc." -ForegroundColor Green
exit 0
