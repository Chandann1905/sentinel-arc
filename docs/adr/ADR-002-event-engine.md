# ADR-002: Event Engine
**Context**: We require an audit trail of all knowledge base changes.
**Decision**: Implement an append-only `EventEngine` that is called automatically within every mutation transaction.
**Status**: Accepted.
