# ADR-001: Repository Pattern
**Context**: We need to manage SQLite transactions across multiple stores (Node, Relationship, Event).
**Decision**: Implement a central `KnowledgeRepository` that instantiates all stores within a shared database transaction.
**Status**: Accepted.
