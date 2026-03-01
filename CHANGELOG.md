# Changelog

All notable changes to this project are documented in this file.

## [Unreleased]

## [1.0.0] - 2026-03-01

### Added

- Production-ready wrapper coverage across core Torn resources, with typed methods for high-value selections and raw fallback methods for long-tail selections.
- Planner/executor resilience features:
  - v2 batching and direct endpoint routing,
  - controlled v1 fallback behavior,
  - retry and rate-limit handling.
- Deterministic capabilities generation and CI drift guard:
  - `scripts/generate-capabilities.ps1`
  - `scripts/check-capabilities-drift.ps1`
- Security/quality release gates in CI:
  - formatting,
  - clippy with `-D warnings`,
  - tests,
  - rustdoc build with strict rustdoc lints,
  - `cargo audit`,
  - `cargo deny check`.
- Release-only publish dry-run workflow:
  - `.github/workflows/release.yml` (manual dispatch and `v*` tags).
- API key redaction hardening in transport/executor debug/error surfaces with regression coverage.

### Changed

- Crate metadata finalized for public release:
  - version `1.0.0`,
  - repository/homepage set to `https://github.com/swervelord/Torn-Rust-SDK`,
  - crate license expression set to `MIT`.
- Docs policy finalized:
  - crate-level `#![deny(missing_docs)]`,
  - crate-level deny for rustdoc link/URL lints,
  - scoped `missing_docs` allowance on model-heavy modules (`models::generated`, `models::manual`) to avoid blocking on exhaustive field-level docs.
- `cargo deny` CI behavior now uses default advisory fetch strategy (no `--disable-fetch`).

### Fixed

- Wrapper validation behavior around required IDs/path args and invalid range combinations now consistently returns actionable user-facing errors.
- Internal and surfaced error/debug outputs avoid exposing raw API key values.
