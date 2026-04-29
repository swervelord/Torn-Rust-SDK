# Typed Model Strategy

The SDK uses a hybrid model strategy:

1. Generated baseline schemas from `spec/torn-openapi-current.json` where the OpenAPI snapshot provides usable coverage.
2. Manual facade models in `src/models/manual/*` for stable public shapes, conservative unions, and v1-backed resource surfaces such as `company`.

This keeps broad schema alignment without forcing every public API call to return noisy generated types.

## Current Coverage Shape

The typed SDK surface is complete at product scope, but intentionally mixed in how the public facades are modeled.

- Completion-state resource coverage: `company`, `faction`, `forum`, `key`, `market`, `property`, `racing`, `torn`, `user`
- Completion-state wrapper coverage snapshot: `189 / 189` capability selections
- The merged tree may temporarily lag this accounting while the company wrapper and the last faction helpers land alongside the docs update.

The exact selection list is tracked in [../TYPED_COVERAGE_STATUS.md](../TYPED_COVERAGE_STATUS.md).

## Modeling Rules

1. Generated files are treated as artifacts and are not hand-edited.
2. Public wrappers deserialize into manual facades when the response shape is stable enough to own.
3. Manual facades default optional fields aggressively with `#[serde(default)]`.
4. Unknown fields are preserved with `#[serde(flatten)] extra: BTreeMap<String, serde_json::Value>` when the payload is open-ended.
5. Shared envelopes such as pagination metadata are modeled once and reused.
6. Runtime drift beats schema purity. If the live API diverges from the snapshot, the facade should model the stable runtime contract first.

## Shared Pagination And Metadata

Typed paginated responses should preserve Torn's `_metadata` envelope instead of hiding it.

Current shared patterns:

- `PaginatedMetadata`
- `PaginatedMetadataWithTotal`
- `PaginatedLinks`

This is the preferred shape for:

- `user.inventory`
- `user.trades`
- `forum.threads`
- `forum.posts`
- `market.properties`
- `market.rentals`
- `market.auctionhouse`
- `racing.races`
- `torn.bounties`
- other paginated list endpoints as they are typed

## Raw Escape Hatches

Typed coverage is not meant to block access to the full API.

Three escape hatches remain first-class:

1. Per-resource raw helpers such as `events_raw`, `attacklog_raw`, or `raw_selection`.
2. Multi-selection raw helpers such as `raw_selections`.
3. Direct low-level client access through `sdk.client()`.

This keeps full typed coverage from forcing brittle public models for unstable or underspecified responses.
The same escape hatch story still matters for future Torn additions and for callers that prefer raw multi-selection workflows.

## Union And Drift Strategy

Some Torn surfaces need stricter handling than simple `serde` structs.

- Prefer strongly typed stable variants for known runtime shapes.
- Preserve unrecognized variants instead of failing deserialization outright.
- Use conservative facades for explicit v1 fallback selections such as `torn.stats`, `torn.searchforcash`, or `torn.shoplifting`.
- Use conservative facades for v1-backed surfaces such as `company` and legacy Torn fallbacks.
- Keep runtime-specific quirks documented in tests and docs when the spec is known to lag live behavior.

Examples already following this pattern include:

- `user.trade` item unions
- `user.competition`
- `user.job`
- the completion-state `company` wrapper surface
- mixed pagination envelopes reused across resources
- Torn surfaces that keep typed public helpers even when planner routing falls back to v1

## Wrapper Decision Boundary

A selection should get a first-class typed helper when most of these are true:

- the live shape is stable,
- the endpoint is high value,
- the public facade can be maintained without weekly churn,
- the ambiguity can be isolated to a small fallback area instead of infecting the whole model.

If those conditions are not met, keep the raw method and defer the facade until the shape is clearer.
