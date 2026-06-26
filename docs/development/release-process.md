# Release Process

This document outlines the standard operating procedure for cutting a new release of Sentinel Arc.

## 1. Version Bump
We strictly adhere to Semantic Versioning (SemVer).
- Modify the `version` field in the workspace `Cargo.toml`.
- Because crates inherit `version.workspace = true`, you only need to change it in the root `Cargo.toml`.

## 2. Generate Changelog
Review the commit history since the last tag. Update the `CHANGELOG.md` with:
- **Added**: New features.
- **Changed**: Modifications to existing functionality.
- **Deprecated**: Features to be removed.
- **Removed**: Features that have been removed.
- **Fixed**: Bug fixes.
- **Security**: Security vulnerabilities resolved.

## 3. Verify Quality Gates
Run the full verification suite locally before pushing:
```bash
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
```

## 4. Tag and Push
Commit the version bump and changelog. Then, create an annotated git tag.

```bash
git add Cargo.toml CHANGELOG.md
git commit -m "chore: release vX.Y.Z"
git tag -a vX.Y.Z -m "Release vX.Y.Z"
git push origin main
git push origin vX.Y.Z
```

## 5. GitHub Release
Once the tag is pushed, the `.github/workflows/release.yml` GitHub Action will automatically trigger (if configured). Otherwise, navigate to the GitHub Releases page, select the new tag, and paste the changelog notes to publish the release publicly.
