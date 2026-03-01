# Release Checklist

## Version + Metadata

- [ ] Confirm `Cargo.toml` contains:
  - `version = "1.0.0"` (or intended release version),
  - `license = "MIT"`,
  - `repository = "https://github.com/swervelord/Torn-Rust-SDK"`,
  - `homepage = "https://github.com/swervelord/Torn-Rust-SDK"`,
  - `documentation = "https://docs.rs/torn_sdk_planner"`.
- [ ] Confirm `CHANGELOG.md` includes a dated release section for this version.
- [ ] Confirm `LICENSE-MIT` is present and authoritative.

## Local Final Checks

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

- [ ] All commands above pass locally.

## Live Validation

```powershell
cargo test --test live_torn_smoke -- --ignored
```

- [ ] Ignored live smoke test passes with release environment variables configured.

## Publish Dry-Run Gate

```powershell
cargo publish --dry-run
```

- [ ] Local dry-run passes.
- [ ] CI dry-run workflow `.github/workflows/release.yml` has passed (manual dispatch and/or `v*` tag run).

## Release Safety

- [ ] No real API keys or secrets are committed.
- [ ] API key redaction behavior still passes in tests and spot checks.
