# Wrapper Architecture

`TornSdk` is a thin additive layer over `TornClient`.

Core planning/execution behavior is unchanged:

- planner handles selection batching/splitting and v1/v2 routing,
- executor handles retries, rate limits, and runtime fallback behavior.

The wrapper focuses on ergonomics, typed models, and input validation.

## API Shape

- Resource-first accessors:
  - `sdk.user()`
  - `sdk.faction()`
  - `sdk.market()`
  - `sdk.torn()`
- Escape hatch:
  - `sdk.client()` for fully generic calls.

## Typed Coverage

Typed methods target stable, high-value payloads:

- `user`: `profile`, `basic`, `discord`, `faction`, `money`, `bars`, `cooldowns`, `travel`
- `faction`: `basic`, `members`, `wars`, `rankedwars`
- `market`: `bazaar`, `itemmarket`
- `torn`: `calendar`, `items`, `honors`, `medals`
- `racing`: `race`
- `forum`: `thread`
- `property`: `property`
- `key`: `info`

Manual typed models are resilient:

- optional fields for unstable payload sections,
- `#[serde(default)]` on list/object fields,
- flattened maps for unknown extra fields.

## Raw Fallback

All wrapper resources support:

- per-selection raw methods (`*_raw`)
- `raw_selection`
- `raw_selections`
- generic `typed_selection<T>` for custom typed decoding

This keeps long-tail and volatile endpoints available without waiting for typed model updates.

## Validation Strategy

Wrapper validation returns `SdkError::Validation` before network calls:

- unsupported selection name,
- missing required identifier/path option (`id`, `threadId`, `raceId`, etc.),
- invalid shared range options (`from > to`),
- endpoint-specific requirements (for example `faction.search` requires `name`).

Validation is additive and wrapper-local. Planner/executor internals are unchanged.

## Consistency Guarantees

- `SUPPORTED_SELECTIONS` in each wrapper resource is test-checked against `spec/capabilities.json`.
- Documentation examples are exercised by integration tests (`tests/documentation_contract.rs`).

## Migration Notes

- Typed wrapper return types now expose structured fields for high-value endpoints instead of plain `serde_json::Value`.
- Raw access remains unchanged (`*_raw`, `raw_selection`, `raw_selections`) for payloads that are still volatile or not yet typed.
- If you previously accessed typed bundles as raw JSON indexes, update code to use typed fields and `extra` maps for unknown/forward fields.
