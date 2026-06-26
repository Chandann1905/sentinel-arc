# Sentinel Arc — Project Brain

Sentinel Arc is a local-first AI knowledge graph and project memory platform that gives coding agents architectural context, deterministic memory, and impact analysis for large codebases.

Built in Rust, it serves as the foundation for autonomous agents, providing robust state management, transactional integrity, and rule-based governance.

## Architecture

Sentinel Arc employs a strictly layered architecture using **Domain Driven Design (DDD)** and the **Repository Pattern**:

- **Core**: Contains pure domain models (`Node`, `Relationship`, `Event`, `Rule`) and shared trait definitions.
- **Knowledge Engine**: The primary facade coordinating operations across:
  - `NodeEngine`: Node lifecycle management.
  - `RelationshipEngine`: Graph traversal and connection management.
  - `EventEngine`: Audit trail and temporal query execution.
  - `RuleEngine`: Business rule governance and evaluation.

## Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (1.70.0 or later)

### Build & Run
```bash
# Clone the repository
git clone https://github.com/Chandann1905/sentinel-arc.git
cd sentinel-arc/project-brain

# Build the project
cargo build

# Run all tests
cargo test
```

## Documentation
- **Architecture**: See the `docs/` directory.
- **API Documentation**: Run `cargo doc --open --no-deps` to view the comprehensive API docs.

## Contributing
We welcome contributions from the community! Please read our [Contributing Guide](CONTRIBUTING.md) and [Code of Conduct](CODE_OF_CONDUCT.md) before submitting pull requests.

## License
This project is licensed under the [MIT License](LICENSE).
