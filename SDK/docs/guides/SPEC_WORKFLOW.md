# Torn OpenAPI spec workflow

Use these scripts to always work from the latest Torn OpenAPI spec.

## Scripts

- `scripts/sync-torn-openapi.ps1`
  - Downloads `https://www.torn.com/swagger/openapi.json`
  - Reads `info.version`
  - Saves a versioned snapshot at `spec/versions/torn-openapi-<version>.json`
  - Updates the stable file at `spec/torn-openapi-current.json`
  - Updates lock metadata at `spec/metadata.json`

- `scripts/start-project.ps1`
  - Runs the sync script
  - Regenerates `spec/capabilities.json` from the synced spec
  - Prints the current version and canonical spec path for generators/tooling

- `scripts/generate-capabilities.ps1`
  - Parses the canonical spec and emits a machine-readable capability map
  - Captures:
    - Generic selection endpoint/query model per resource (`/user`, `/faction`, etc.)
    - Available selections (including fallback/unavailable flags from schema descriptions)
    - Endpoint-level query/path parameters and response top-level fields
    - Nested response field paths (for documenting subfields and typed SDK modeling)
    - Filter capabilities (`limit`, `from`, `to`, `sort`, `cat`, `stat`, `filters`, `striptags`, `offset`, `timestamp`)
  - Supports manual overrides via `spec/capability-overrides.json`

- `scripts/check-capabilities-drift.ps1`
  - Regenerates capabilities to a temp file
  - Compares against committed `spec/capabilities.json`
  - Fails if committed capabilities are stale

## Run at project start

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\start-project.ps1
```

## Run manually

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\sync-torn-openapi.ps1
```

## Regenerate capabilities only

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\generate-capabilities.ps1
```

## Check for capabilities drift

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\check-capabilities-drift.ps1
```

## Optional force refresh

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\sync-torn-openapi.ps1 -Force
```

## Canonical file for SDK work

Always point codegen/parsers at:

- `spec/torn-openapi-current.json`
- `spec/capabilities.json` (for smart request planning/selection batching)

The pinned version and hash are stored in:

- `spec/metadata.json`

Manual overrides for capability edge cases are stored in:

- `spec/capability-overrides.json`

Rust planner usage is documented in:

- `../architecture/REQUEST_PLANNER.md`
- `../architecture/EXECUTOR.md`
- `CLIENT_API.md`
- `ENVIRONMENT.md`
