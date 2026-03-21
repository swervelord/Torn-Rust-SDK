# Publishing Guide

This document defines the exact release flow for `torn_sdk_planner`.

## 1. Version + Metadata

Verify `Cargo.toml` contains:

- `version = "1.0.0"` (or intended release version),
- `license = "MIT"`,
- `repository = "https://github.com/swervelord/Torn-Rust-SDK"`,
- `homepage = "https://github.com/swervelord/Torn-Rust-SDK"`,
- `documentation = "https://docs.rs/torn_sdk_planner"`.

Also verify:

- `CHANGELOG.md` has a dated release section for the release version.
- `LICENSE-MIT` is present and authoritative.

## 2. Local Final Checks

Run from repository root:

```powershell
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
$env:RUSTDOCFLAGS="-D rustdoc::broken_intra_doc_links -D rustdoc::bare_urls"; cargo doc --no-deps --all-features
powershell -ExecutionPolicy Bypass -File .\scripts\check-capabilities-drift.ps1
cargo audit
cargo deny check
```

## 3. Live Validation

```powershell
cargo test --test live_torn_smoke -- --ignored
```

## 4. Publish Dry Run

```powershell
cargo publish --dry-run
```

## 5. Release Workflow Gate

Run `.github/workflows/release.yml` via `workflow_dispatch` (or by pushing a `v*` tag) and confirm the `cargo publish --dry-run` job succeeds.

No commit/tag/push automation is performed by this guide.
