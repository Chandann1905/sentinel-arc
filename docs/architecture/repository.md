# Repository Pattern

Sentinel Arc strictly employs the **Repository Pattern** to decouple business logic from storage implementation details.

## The `KnowledgeRepository`

The `KnowledgeRepository` struct holds the `sqlx::SqlitePool` and is passed by reference to all Engines. It serves as the sole gateway to the underlying SQLite database.

- **Encapsulation**: Raw SQL queries are entirely restricted to the `store/` modules. No Engine is permitted to execute SQL.
- **Transactions**: The repository exposes wrapper methods that execute domain-level operations across multiple stores within a single atomic database transaction.

## Stores

The Repository initializes specialized "Stores" internally:

1. `NodeStore`: Executes `INSERT`, `UPDATE`, and `SELECT` for nodes. Handles JSON serialization of `tags` and `metadata`.
2. `RelationshipStore`: Executes SQL for graph edges.
3. `EventStore`: Executes append-only SQL inserts for the temporal ledger.
4. `RuleStore`: Executes CRUD for business constraints.

These stores are completely stateless, accepting the active `sqlx::Transaction` or `sqlx::Pool` reference dynamically per query.
