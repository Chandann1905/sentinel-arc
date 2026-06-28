# Sentinel Arc — v1.0 Release Candidate Checklist

## 1. Build Verification
- [x] **`cargo fmt --check`**: PASSED. No formatting deviations detected.
- [x] **`cargo clippy --workspace --all-targets --all-features -- -D warnings`**: PASSED. Zero linting warnings across the entire workspace.
- [x] **Fresh Installation**: PASSED. `cargo install --path crates/cli` successfully compiles the CLI from scratch and places the `sentinel-cli` executable globally without dependency resolution errors (outside of transient crates.io network timeouts).

## 2. Test Verification
- [x] **`cargo test --workspace --all-features`**: PASSED. All unit and integration tests executed successfully across the `core`, `knowledge`, `timeline`, `validation`, `context`, `scanner`, `mcp`, and `cli` crates.
- [x] **`cargo doc --workspace --no-deps`**: PASSED. Zero missing documentation warnings in public APIs.

## 3. Documentation Verification
- [x] All commands listed in `docs/cli.md` map 1:1 with `sentinel-cli --help`.
- [x] All troubleshooting guides accurately reflect executable behavior and environment requirements.
- [x] Marketing claims have been purged; the architecture documentation reflects the factual behavior of the Repository and Engine layers.

## 4. MCP Verification
- [x] `sentinel-cli mcp` boots correctly and listens on Stdio.
- [x] MCP Tools (`search_nodes`, `get_node`, `generate_context`, `generate_timeline`) match the documentation and internal API implementations.
- [x] MCP Resources (`arc://architecture`, `arc://rules`, `arc://validation/report`, `arc://timeline/global`) map correctly to their underlying engine handlers.

## 5. CLI Verification
- [x] `sentinel-cli init` safely initializes SQLite and Tantivy in `.sentinel/`.
- [x] `sentinel-cli scan` parses standard code into the graph.
- [x] `sentinel-cli search`, `graph`, `validate`, and `stats` all execute successfully against a populated database.

## 6. CI/CD Verification
- [x] `.github/workflows/ci.yml` correctly targets macOS, Windows, and Linux on stable Rust.
- [x] `.github/workflows/release.yml` compresses binary artifacts securely and checks version constraints against `GITHUB_REF_NAME` vs CLI binary output.

## Known Limitations
- The MCP server currently only supports `stdio`. External processes seeking to use HTTP or SSE cannot integrate natively without a wrapper.
- Graph multi-node P2P memory sync is not implemented (slated for v2.0).

## Remaining Risks
- The Tree-sitter scanners strictly parse Rust natively; dynamic injection of custom language grammars without recompilation is unsupported in this RC.
- Heavy repositories with >100,000 nodes may cause graph memory projections to consume high RAM, requiring `limit` constraints.

## Release Recommendation
The repository meets all strict standards for reliability, memory safety, architectural integrity, and documentation consistency. There are no dangling features or failing test suites.

**STATUS:** READY FOR v1.0 RC
