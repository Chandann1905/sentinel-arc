# ADR-003: Knowledge Engine Facade
**Context**: Upstream clients (e.g., UI, MCPs) need a simple API to interact with the system.
**Decision**: Create a `KnowledgeEngine` facade that wraps all sub-engines and enforces the public API boundary.
**Status**: Accepted.
