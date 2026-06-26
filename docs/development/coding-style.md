# Coding Style
- Follow idiomatic Rust formatting (`cargo fmt`).
- No warnings are allowed (`cargo clippy -- -D warnings`).
- Internal engine logic MUST remain `pub(crate)` to prevent API leakage.
