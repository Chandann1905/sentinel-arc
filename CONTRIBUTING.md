# Contributing to Sentinel Arc

Thank you for your interest in contributing to Sentinel Arc! By participating in this project, you agree to abide by our [Code of Conduct](CODE_OF_CONDUCT.md).

## Getting Started

### 1. Prerequisites
You will need:
- **Rust Toolchain**: `1.85.0` or higher (we recommend installing via [rustup](https://rustup.rs)).
- **Git**: For version control.
- **C Compiler**: For compiling `libsqlite3-sys` on some platforms.

### 2. Local Environment Setup
Fork the repository and clone it to your local machine:
```bash
git clone https://github.com/YOUR_USERNAME/sentinel-arc.git
cd sentinel-arc
```

To verify your environment is functioning correctly, build the workspace:
```bash
cargo build --workspace
```

---

## Development Standards

Sentinel Arc acts as a durable database and source of truth for autonomous systems. **Memory safety, ACID compliance, and API stability are paramount.**

### Formatting
All code must conform to the standard rustfmt configuration.
Before committing, always run:
```bash
cargo fmt --all
```

### Linting (Clippy)
We enforce a strict zero-warning policy. If your code produces a warning, the CI pipeline will fail.
Before committing, always run:
```bash
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

### Testing
All new engines or modules must include 100% test coverage for success paths and edge cases (especially invalid input handling and transaction rollbacks).
```bash
cargo test --workspace --all-features
```

### Documentation
Any modifications to public APIs (`pub` structs, traits, or functions) in the core crates must be accompanied by accurate `///` Rustdocs. 
```bash
cargo doc --workspace --no-deps
```

---

## Pull Request Checklist

When you are ready to submit a Pull Request, please ensure the following:

- [ ] **Branch Naming**: Use a descriptive prefix (e.g., `feat/`, `fix/`, `docs/`, `refactor/`).
- [ ] **Commit Messages**: Write clear, descriptive commit messages. We prefer [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) (e.g., `feat(graph): add strongly connected components detection`).
- [ ] **Tests Passing**: You have run `cargo test` locally and everything passes.
- [ ] **Linters Clean**: You have run `cargo clippy -- -D warnings` and resolved all issues.
- [ ] **Documentation**: You have added Rustdocs for new public APIs. If you added a new feature, you have updated the corresponding markdown guide in `docs/`.

---

## Architectural Guidelines

When contributing new features, respect the **Domain-Driven Design (DDD)** boundaries:
1. **Never bypass `KnowledgeEngine`**: External APIs (like CLI and MCP) must never talk to the Repository layer or SQLite directly.
2. **Never import `sqlx` in high-level engines**: The Database interactions belong exclusively in the Storage/Repository layers.
3. **Immutability**: The Timeline and Context engines are inherently read-only. Do not introduce mutation logic into them.
