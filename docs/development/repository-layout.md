# Repository Layout

Sentinel Arc is a standard Cargo workspace. Below is a map to help you navigate.

```text
sentinel-arc/
├── .github/                  # GitHub Actions CI/CD and Issue Templates
├── Cargo.toml                # Workspace definition
├── README.md                 # Project landing page
├── crates/
│   ├── core/                 # `sentinel-arc-core`
│   │   ├── src/
│   │   │   ├── domain/       # Pure models: Node, Event, Rule, Relationship
│   │   │   ├── types/        # Enums, Ids, Source types
│   │   │   └── error.rs      # Global domain errors
│   └── knowledge/            # `sentinel-arc-knowledge`
│       ├── migrations/       # SQLite schemas (001_..., 002_...)
│       ├── src/
│       │   ├── engine/       # Logic processors (KnowledgeEngine, NodeEngine)
│       │   ├── store/        # SQLx executors (NodeStore, EventStore)
│       │   └── repository.rs # Transaction coordinator
├── docs/                     # Documentation
│   ├── adr/                  # Architecture Decision Records
│   ├── architecture/         # Technical architecture specifications
│   └── development/          # Onboarding guides (you are here)
├── examples/                 # Minimal runnable code snippets
└── scripts/                  # Developer setup scripts
```

## Where should I put my code?
- **Adding a new data field to a Node?** Modify `crates/core/src/domain/node.rs` and then add an SQL migration to `crates/knowledge/migrations/`.
- **Adding a new query method?** Add it to the specific store (e.g., `NodeStore`), then expose it via the Engine (`NodeEngine`), and finally expose it publicly via the `KnowledgeEngine`.
