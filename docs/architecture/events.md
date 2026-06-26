# Event Sourcing and Temporal Auditing

Every mutation in Sentinel Arc is permanently recorded. The system uses an **Append-Only Event Ledger** pattern.

## Event Immutability
Events are strictly append-only. There are no API endpoints or internal mechanisms to update or delete an `Event` once it is committed to the database.

## Dual-Write Guarantees
Whenever a state mutation occurs (e.g., `NodeUpdated`), the target entity (Node) is updated in the database, and the corresponding temporal record (`Event`) is inserted into the `events` table. 

This happens explicitly within a single atomic database transaction inside the `KnowledgeRepository`. If the Event insertion fails, the Node update rolls back instantly.

## The `Event` Model
Defined in `sentinel-arc-core`, the `Event` encapsulates:
- `id`: A deterministic UUID.
- `entity_id`: The ID of the Node or Relationship affected.
- `event_type`: An enum defining the operation (`NodeCreated`, `RelationshipDeleted`, etc.).
- `payload`: A generic `serde_json::Value` capturing the specific delta or state snapshot.
- `timestamp`: UTC DateTime of the occurrence.
