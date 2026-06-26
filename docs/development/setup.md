# Environment Setup

Welcome to Sentinel Arc! This guide covers everything you need to start contributing to the repository.

## 1. Prerequisites

We assume you have never worked on the Sentinel Arc codebase before, but you should have a basic understanding of Rust.

- **Rust Toolchain**: You need Rust version `1.70.0` or higher.
- **Git**: For version control.

### Installing Rust
If you don't have Rust installed, use `rustup`:

**Linux/macOS:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Windows:**
Download and run the installer from [rustup.rs](https://rustup.rs).

### C Compiler (Optional but recommended)
Since `sqlx` compiles a bundled SQLite implementation (`libsqlite3-sys`), you might need a C compiler on your system to build the bindings.
- **macOS**: `xcode-select --install`
- **Ubuntu/Debian**: `sudo apt install build-essential`
- **Windows**: The Visual Studio C++ Build Tools (installed optionally during `rustup-init.exe`) are sufficient.

## 2. Cloning the Repository

```bash
git clone https://github.com/Chandann1905/sentinel-arc.git
cd sentinel-arc
```

## 3. The Setup Script
For maximum convenience, run the setup script provided in the repository root. This script verifies your toolchain, checks formatting, and runs the test suite.

**Windows:**
```powershell
.\scripts\setup.ps1
```

**macOS / Linux:**
```bash
./scripts/setup.sh
```

If the script completes successfully with "All tests passed!", your environment is perfectly configured and you are ready to write code.

## Next Steps
Read [Building & Compiling](building.md) to understand how to interact with Cargo in this workspace.
