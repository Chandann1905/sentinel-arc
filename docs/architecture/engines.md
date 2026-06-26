# Internal Engines Architecture

The Application layer is driven by discrete, decoupled "Engines". Each Engine governs a distinct slice of the domain logic.

## 1. NodeEngine
Responsible for the lifecycle of Nodes.

- **Responsibilities**: Creation, validation, updates, version bumping, and archival.
- **Rules**: A Node cannot be modified without bumping its version. All state changes emit a corresponding `NodeCreated`, `NodeUpdated`, or `NodeDeleted` event within the same database transaction.

## 2. RelationshipEngine
Responsible for managing connections between Nodes.

- **Responsibilities**: Creation and deletion of directed edges, cycle detection (future), and dependency resolution.
- **Rules**: Ensures that both the source and target nodes exist in the `NodeStore` before creating a relationship. It prevents orphaned relationships by emitting `RelationshipCreated` and `RelationshipDeleted` events inside a transaction.

## 3. EventEngine
Responsible for read-only access to the system's temporal ledger.

- **Responsibilities**: Querying historical states, fetching chronological events by Node or Relationship.
- **Rules**: The `EventEngine` itself never creates events. Events are appended strictly by the `NodeEngine` and `RelationshipEngine` via the `KnowledgeRepository`. It is purely a query execution layer.

## 4. RuleEngine
Responsible for the dynamic business constraints of the system.

- **Responsibilities**: Registering constraints, evaluating predicates against node metadata, and categorizing rules.
- **Rules**: Future extensibility point for WASM-based or declarative JSON constraints.

## KnowledgeEngine Facade
The `KnowledgeEngine` exposes public methods and aggregates the internal `pub(crate)` engines. It is the only public interface for the Application layer, completely isolating the HTTP/CLI frontends from internal component dependencies.
