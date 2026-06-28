# Frequently Asked Questions (FAQ)

### Why SQLite instead of a dedicated Graph Database (like Neo4j)?
Sentinel Arc is designed to be an *embedded, zero-configuration* tool. We want developers and autonomous agents to maintain state inside a repository without requiring Docker containers, separate network processes, or heavy Java dependencies. SQLite provides ACID-compliant transactions inside a single `.db` file, which is designed for embedded usage and maps perfectly to the per-workspace requirements of code editors.

### Why Tantivy instead of SQLite FTS?
While SQLite offers Full-Text Search (FTS5), [Tantivy](https://github.com/quickwit-oss/tantivy) (a Rust port of Lucene) provides richer query capabilities, better memory efficiency, and complex query syntax (fuzzy matching, phrase queries, boosting). We leverage Tantivy specifically to optimize LLM context retrieval.

### Does Sentinel Arc modify my source code?
**No.** Sentinel Arc is strictly a read-only observer of your file system. It reads the AST (Abstract Syntax Tree) using Tree-sitter and stores a representation of your code inside the `.sentinel/` database. It will never overwrite, format, or mutate your actual source code.

### What languages does the Scanner Engine support?
Currently, the Scanner Engine is optimized for **Rust**. However, the engine is modular by design, and support for Python, TypeScript, and Go is mapped for future `v1.x` releases (see [ROADMAP.md](../ROADMAP.md)).

### Can I run the MCP Server over HTTP instead of Stdio?
Currently, the `sentinel-arc-mcp` crate only implements the `Stdio` transport layer, as this is the primary mechanism utilized by local IDEs (Cursor, VS Code) and Claude Desktop. HTTP/SSE transport support is under consideration for `v2.0` distributed environments.
