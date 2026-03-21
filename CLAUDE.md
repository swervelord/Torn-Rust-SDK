# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Workspace Layout

This is a development workspace with two top-level folders:

- **`SDK/`** — The `torn_sdk_planner` Rust crate (edition 2024, MSRV 1.85). This is the Torn API SDK.
- **`Projects/`** — Staging area for new projects that depend on the SDK. When production-ready, Discord bots move to `/home/<name>`, websites to `/var/www/<name>`.

SDK internals stay in `SDK/`. Project code stays in `Projects/`. Do not mix them.

## SDK Commands

All SDK commands run from the `SDK/` directory:

```bash
cd /home/Torn-Rust-SDK/SDK

# Build / check
cargo check
cargo build

# Lint (must pass clean)
cargo clippy --all-targets --all-features -- -D warnings

# Run all tests (unit + integration, uses mock transport)
cargo test

# Run a single test
cargo test test_name
cargo test -- --exact module::test_name

# Live smoke tests (skipped unless env vars are set)
TORN_LIVE_TEST_API_KEY=xxx cargo test --test live_torn_smoke
```

SDK credentials go in `SDK/.env` (see `SDK/.env.example`). Keys: `TORN_API_KEYS` (comma-separated) or `TORN_API_KEY` (single).

## Project Dependency Pattern

Projects under `Projects/` use a local path dependency during incubation:

```toml
[dependencies]
torn_sdk_planner = { path = "../../SDK" }
```

After promotion to `/home/<name>`, the relative path breaks. Either use an absolute path or keep the capabilities file accessible via an env var (e.g., `CAPABILITIES_PATH=/home/Torn-Rust-SDK/SDK/spec/capabilities.json`).

## SDK Architecture

The SDK follows a **plan-then-execute** pipeline:

1. **Capabilities** (`capabilities.rs`, `spec/capabilities.json`) — Generated metadata describing which selections each Torn API resource supports, including v2 endpoints and parameters. Loaded at startup, drives all routing decisions.

2. **Planner** (`planner.rs`) — Takes a `PlanRequest` (resource + selections) and produces a `RequestPlan` containing one or more `PlannedRequest`s. Handles v2 batching (grouping selections into a single generic call), direct endpoint routing, and v1 fallback assignment. Uses `v1_catalog.rs` for static v1 resource/selection mappings.

3. **Executor** (`executor.rs`) — Executes `PlannedRequest`s with retries, rate limiting (per-key and per-IP via `rate_limit.rs`), concurrency control, and automatic v2-to-v1 fallback on failure.

4. **Transport** (`transport.rs`) — HTTP abstraction trait (`HttpTransport`) with a default `ReqwestTransport` implementation. Tests use a `MockTransport` (in `tests/support/`).

5. **Client** (`client.rs`) — `TornClient` ties planner + executor together. Provides `request_data<T>()` for typed deserialization and `request_raw()` for `serde_json::Value`.

6. **Wrappers** (`wrapper/`) — `TornSdk` wraps `TornClient` with ergonomic per-resource APIs (`sdk.user()`, `sdk.faction()`, etc.). Each resource module (e.g., `wrapper/user.rs`) has typed methods (returning manual model structs) and raw methods (returning `RawSelectionBundle`/`serde_json::Value`).

7. **Models** (`models/`) — `manual/` has hand-written typed structs for stable selections. `generated/` has `RawSelectionBundle` for dynamic/untyped access.

### Key design points

- `spec/capabilities.json` is the single source of truth for routing. It's generated from the Torn OpenAPI spec via PowerShell scripts in `scripts/`.
- The planner never makes HTTP calls — it only produces plans. The executor handles all I/O.
- Wrappers validate inputs (selection names, required path args) before calling the client, surfacing `SdkError` early.
- `#![deny(missing_docs)]` is enforced — all public items need doc comments.

## Test Structure

- `tests/core_contract.rs` — Primary integration tests using `MockTransport`. Tests planner routing, executor behavior, wrapper APIs, and error paths.
- `tests/documentation_contract.rs` — Validates doc examples and public API surface.
- `tests/live_torn_smoke.rs` — Real API smoke tests, gated behind `TORN_LIVE_TEST_API_KEY` env var.
- `tests/fixtures/` — JSON response fixtures for mock-based tests.

## Spec / Capabilities Workflow

The `spec/` directory contains the Torn OpenAPI spec and derived capabilities:

- `spec/torn-openapi-current.json` — Current OpenAPI spec snapshot
- `spec/capabilities.json` — Derived routing metadata consumed by the planner
- `spec/metadata.json` — Tracks spec version and fetch timestamp

PowerShell scripts in `scripts/` handle syncing the OpenAPI spec and regenerating capabilities. The capabilities drift check (`scripts/check-capabilities-drift.ps1`) runs in CI.

## Runtime Configuration

All runtime config is via environment variables (see `SDK/.env.example`). Key ones beyond API keys:

- `TORN_HTTP_TIMEOUT_SECS`, `TORN_MAX_ATTEMPTS`, `TORN_NETWORK_RETRY_BACKOFF_MS` — transport/retry tuning
- `TORN_RATE_LIMIT_PER_KEY_PER_MIN`, `TORN_RATE_LIMIT_PER_IP_PER_MIN` — rate limit overrides
- `TORN_MAX_IN_FLIGHT` — concurrency cap
