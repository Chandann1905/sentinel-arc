# Repository Pattern
All state mutation and retrieval passes through the `KnowledgeRepository`. It is the sole owner of SQLite transactions, ensuring ACID compliance and eliminating partial state updates.
