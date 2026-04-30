# Changelog

All notable changes to this project are documented in this file.

## Unreleased

### Changed

- Synced Torn OpenAPI schema to `5.8.0`.
- Updated `company` routing for the API v2 refactor: `profile`, `employees`, `applications`, `stock`, `lookup`, and `timestamp` now use v2 endpoints, while `companies`, `news`, and `search` remain v1 fallbacks.
- Expanded company typed models for v2 response envelopes and preserved compatibility with legacy v1-shaped company payloads.

## [1.0.0] - 2026-03-01

### Added

- Production-ready wrapper coverage across core Torn resources, with typed methods for high-value selections and raw fallback methods for long-tail selections.
- Planner/executor resilience features:
  - v2 batching and direct endpoint routing,
  - controlled v1 fallback behavior,
  - retry and rate-limit handling.
- Security/quality release gates in CI:
  - formatting,
  - clippy with `-D warnings`,
  - tests,
  - rustdoc build with strict rustdoc lints,
  - `cargo audit`,
  - `cargo deny check`.
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

### Fixed

- Wrapper validation behavior around required IDs/path args and invalid range combinations now consistently returns actionable user-facing errors.
- Internal and surfaced error/debug outputs avoid exposing raw API key values.
