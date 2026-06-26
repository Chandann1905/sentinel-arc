#!/usr/bin/env bash
set -e

echo -e "\033[0;36mSentinel Arc Setup Script\033[0m"
echo -e "\033[0;36m===========================\033[0m"

# 1. Check for Cargo
if ! command -v cargo &> /dev/null; then
    echo -e "\033[0;31mError: Cargo not found. Please install Rust via:\033[0m"
    echo "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

echo -e "\033[0;32mDetected Toolchain: $(rustc --version)\033[0m"

# 2. Check for C Compiler (Required for sqlx sqlite bundled)
if ! command -v cc &> /dev/null && ! command -v gcc &> /dev/null && ! command -v clang &> /dev/null; then
    echo -e "\033[0;33mWarning: No C compiler detected. If the build fails, install build-essential or xcode-select.\033[0m"
fi

# 3. Format
echo -e "\n\033[0;36mRunning cargo fmt...\033[0m"
cargo fmt --all

# 4. Clippy
echo -e "\n\033[0;36mRunning clippy...\033[0m"
cargo clippy --workspace --all-targets --all-features -- -D warnings

# 5. Build
echo -e "\n\033[0;36mBuilding Workspace...\033[0m"
cargo build --workspace

# 6. Test
echo -e "\n\033[0;36mRunning Test Suite...\033[0m"
cargo test --workspace

echo -e "\n\033[0;32mSuccess! All tests passed! You are ready to contribute to Sentinel Arc.\033[0m"
exit 0
