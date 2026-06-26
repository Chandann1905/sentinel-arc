# High-Level Architecture Guide

This guide provides a pointer to our deep-dive architectural documents located in the `docs/architecture` folder.

If you are aiming to understand how the system's components interact, please read them in the following order:

1. **[System Overview](../architecture/system.md)**: A high-level Mermaid diagram of how the Knowledge Engine isolates the Database.
2. **[Engine Boundaries](../architecture/engines.md)**: Details on the Node, Event, Relationship, and Rule engines, and their strict `pub(crate)` encapsulation.
3. **[Repository Pattern](../architecture/repository.md)**: How we decouple SQLx logic from domain logic.
4. **[Transaction Flow](../architecture/transactions.md)**: A sequence diagram illustrating how atomic rollbacks prevent partial state mutations.
5. **[Event Sourcing](../architecture/events.md)**: How the append-only ledger guarantees temporal accuracy.

## Domain Driven Design (DDD)
We enforce a strict separation of concerns:
- **`sentinel-arc-core`**: The pure data models. These structs have no concept of SQLite, SQLx, or async runtimes.
- **`sentinel-arc-knowledge`**: The implementation layer. It converts domain models into SQL queries, but never bleeds SQL types into the public API.
