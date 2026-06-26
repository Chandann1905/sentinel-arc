# Engines
Engines encapsulate business logic for specific domains (Nodes, Relationships, Events, Rules). They orchestrate operations but **never** access the database directly; they delegate to Stores via the Repository.
