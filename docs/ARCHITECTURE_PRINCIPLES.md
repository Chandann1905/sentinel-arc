# Immutable Architecture Principles
1. Engines **never** access SQLite directly.
2. Stores **never** contain business logic.
3. Repository owns transactions.
4. Events are immutable.
5. Every mutation emits an Event.
6. Graph is a projection, never the source of truth.
7. KnowledgeEngine is the only public knowledge entry point.
8. AI must derive knowledge from stored entities only.
9. No duplicated business rules.
10. No circular dependencies.
11. No hidden side effects.
