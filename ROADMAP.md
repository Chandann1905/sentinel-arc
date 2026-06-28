# Roadmap

Sentinel Arc is approaching its first stable `v1.0` release. The project milestones are organized by engines and capabilities, representing the steady progression of the platform from a foundational graph database to a highly scalable AI memory orchestration layer.

---

## 🟢 Completed Milestones

The following capabilities are fully implemented, tested, and actively used:

- **Foundation (v0.1.0)**: Established Domain-Driven Design (DDD) architecture, pure `core` models, and workspace abstractions.
- **Storage Layer**: SQLite schema definitions, indexing, and connection pooling.
- **Repository Layer**: CRUD operations mapped safely to domain entities.
- **Node Engine**: Ingestion and versioning of knowledge `Node` entities.
- **Relationship Engine**: Directional connection mapping between entities.
- **Event Engine**: Synchronous, append-only temporal tracking (Event Sourcing).
- **Rule Engine**: Business logic rule enforcement for node and relationship mutations.
- **Knowledge Engine**: The primary transactional orchestrator and public facade.
- **Search Engine**: Full-text ingestion and retrieval utilizing `tantivy`.
- **Graph Engine**: In-memory topology projection using `petgraph` for BFS/DFS traversals.
- **Scanner Engine**: Tree-sitter file system parsers to automatically map ASTs to nodes.
- **Validation Engine**: Project health scanning and drift detection mechanisms.
- **Developer CLI**: The `sentinel-cli` interface bridging CLI flags to engine commands.
- **MCP Server**: Natively embedded JSON-RPC 2.0 Model Context Protocol capabilities.

---

## 🟡 Current State (Preparing for v1.0)

We are finalizing the developer experience and hardening the runtime.

- [x] Concurrency limits and request multiplexing in the MCP Server.
- [x] Automated CI/CD (cross-compiling Windows, macOS, Linux).
- [x] Security workflow implementations (`cargo-audit`, `CodeQL`).
- [x] Comprehensive Developer Documentation and Guides.
- [ ] Stabilizing the public API to guarantee zero breaking changes across `v1.x`.

---

## 🔵 Future (v1.x)

Post-v1.0 features designed to elevate agentic capabilities without compromising backward compatibility.

- **Cloud Synchronization**: Allowing `.sentinel` SQLite stores to be synced with S3 or remote storage securely.
- **Vector Search Support**: Augmenting the Tantivy engine with embedding-based semantic search.
- **Streaming Context**: Upgrading the Context Engine to stream large packages asynchronously.
- **Custom Scanners**: Exposing an API for users to write their own Tree-sitter configurations and parser hooks.

---

## 🟣 Future (v2.0)

Long-term, experimental architectures being mapped for the distant future.

- **Distributed Graphs**: Transitioning from a purely local embedded database to a multi-node peer-to-peer memory sync.
- **Real-Time Reactive Rules**: Upgrading the Rule Engine from synchronous rejection to reactive pub-sub event triggers.
