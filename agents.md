# Torn-Rust-SDK Workspace Guide

## Purpose
This directory is a development workspace with two top-level folders:

- `SDK/`: the Torn Rust SDK source, specs, tests, and docs.
- `Projects/`: incubation area for in-progress apps that use the SDK.

Use `Projects/` for active builds, then move finished projects out to their production location.

## Required Structure

- `/home/Torn-Rust-SDK/SDK`
- `/home/Torn-Rust-SDK/Projects`

Keep SDK internals in `SDK/`. Do not place app-specific project code in the SDK folder.

## SDK Overview
The SDK crate is `torn_sdk_planner` and lives in `SDK/Cargo.toml`.

Core behavior:

- Loads Torn API credentials from environment variables (`TORN_API_KEYS` or `TORN_API_KEY`).
- Supports v2 routing with runtime fallback to v1 where needed.
- Exposes typed wrappers plus raw selection escape hatches.

Useful SDK commands:

- `cd /home/Torn-Rust-SDK/SDK && cargo check`
- `cd /home/Torn-Rust-SDK/SDK && cargo test`
- `cd /home/Torn-Rust-SDK/SDK && cargo clippy --all-targets --all-features -- -D warnings`

## Project Workflow (Inside `Projects/`)
Each project should be its own folder under `Projects/`, for example:

- `/home/Torn-Rust-SDK/Projects/discord-tracker-bot`
- `/home/Torn-Rust-SDK/Projects/faction-site`

For Rust projects in `Projects/<project-name>`, use a local path dependency during incubation:

```toml
[dependencies]
torn_sdk_planner = { path = "../../SDK" }
```

Recommended per-project layout:

- `src/`
- `Cargo.toml`
- `.env` (local secrets, not committed)
- `.env.example` (safe template)
- `README.md` (setup/run notes)

## Promotion to Production Paths
When a project is production-ready:

- Discord bots move to `/home/<project-name>`
- Websites move to `/var/www/<project-name>`

Before or during move, update SDK dependency if needed:

- Keep using local path if `/home/Torn-Rust-SDK/SDK` remains accessible, or
- Switch to a git/version dependency for portability.

Example git dependency:

```toml
[dependencies]
torn_sdk_planner = { git = "https://github.com/swervelord/Torn-Rust-SDK", package = "torn_sdk_planner" }
```

## Operational Rules

- Keep SDK credentials in `SDK/.env`.
- Keep project credentials in each project's own `.env`.
- Do not commit real API keys.
- Validate SDK changes in `SDK/` before building dependent projects.
- Allow multiple concurrent project directories under `Projects/`.
