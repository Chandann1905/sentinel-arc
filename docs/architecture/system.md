# System Architecture

Sentinel Arc operates on a strict, layered architecture emphasizing immutability through event-sourcing and separation of concerns through Domain Driven Design (DDD).

## Architecture Layers

```mermaid
graph TD
    subgraph "Presentation Layer (Future)"
        API[HTTP API]
        CLI[Command Line Interface]
    end

    subgraph "Application Layer"
        KE[KnowledgeEngine Facade]
    end

    subgraph "Domain Layer"
        NE[NodeEngine]
        RE[RelationshipEngine]
        EE[EventEngine]
        RUE[RuleEngine]
        MODELS[Core Models: Node, Event, Rule, Relationship]
    end

    subgraph "Infrastructure Layer"
        KR[KnowledgeRepository]
        STORES[NodeStore, EventStore, RelationshipStore, RuleStore]
    end

    subgraph "Persistence"
        SQLITE[(SQLite Database)]
    end

    API --> KE
    CLI --> KE
    KE --> NE
    KE --> RE
    KE --> EE
    KE --> RUE

    NE --> KR
    RE --> KR
    EE --> KR
    RUE --> KR

    KR --> STORES
    STORES --> SQLITE
```

### 1. Presentation Layer
Currently unimplemented. Will house the HTTP API, WebSockets, and CLI. This layer will never interact directly with the stores; it must interface exclusively with the `KnowledgeEngine`.

### 2. Application Layer
The `KnowledgeEngine` acts as the single point of entry. It abstracts away the internal component orchestration, providing a clean, unified API for higher-level consumers.

### 3. Domain Layer
Contains the isolated engines responsible for business logic:
- `NodeEngine`: Enforces node validation, creation, and state transitions.
- `RelationshipEngine`: Validates source-target existence and enforces cyclic constraints.
- `EventEngine`: Provides absolute temporal event auditing.
- `RuleEngine`: Enforces dynamic business constraints.

Core Models are defined in `sentinel-arc-core` and possess no persistence awareness.

### 4. Infrastructure Layer
The `KnowledgeRepository` handles database connection pooling and asynchronous transaction boundaries via `sqlx`. It initializes the specialized Data Access Objects (Stores) required for persistence.
