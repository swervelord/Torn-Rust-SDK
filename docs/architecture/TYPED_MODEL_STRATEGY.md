# Typed Model Strategy

Typed models will follow a hybrid approach:

1. Generated baseline models from `spec/torn-openapi-current.json`.
2. Manual facade models for high-traffic responses with stable ergonomic names.

## Why Hybrid

- Generated models preserve broad schema coverage and reduce drift.
- Manual facades keep wrapper API readable and avoid leaking noisy spec details.

## Rules

1. Generated files are treated as build artifacts and not hand-edited.
2. Facades can flatten and rename fields for ergonomics, but must keep raw access where needed.
3. Unknown/unmodeled fields should remain available via `serde_json::Value` extension fields.
4. Wrapper endpoints should deserialize into facades, not raw generated structs, unless explicitly "raw".

## First Typed Targets

- `user.profile`
- `user.discord`
- `user.hof`
- `faction.members`
- common pagination metadata
