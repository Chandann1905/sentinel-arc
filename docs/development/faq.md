# Frequently Asked Questions (FAQ)

### What is Sentinel Arc?
Sentinel Arc is an embedded state management and event-sourcing engine built in Rust. It serves as the memory layer for AI agents or complex workflow systems.

### Does it require a standalone Database Server?
No. Sentinel Arc uses SQLite via the `sqlx` crate with the bundled feature. The database is compiled directly into your application, meaning it runs entirely locally.

### Why not use an ORM like Diesel?
Sentinel Arc uses `sqlx` because it allows us to write raw SQL while validating queries against the schema at compile time. This provides maximum performance and transparency without the abstraction tax of a heavy ORM.

### Can I access the database directly without the Knowledge Engine?
Technically yes, if you open the SQLite file with a CLI tool. However, programmatically within Rust, the `KnowledgeEngine` is the only `pub` struct. All underlying stores are sealed as `pub(crate)` to prevent developers from accidentally bypassing the event-sourcing audit trail.

### Will there be HTTP endpoints?
Yes. The current architecture strictly implements the domain and data layer. An API layer (`sentinel-arc-api`) is planned for the roadmap, which will wrap the `KnowledgeEngine` in an asynchronous REST or GraphQL server.
