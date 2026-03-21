# Client API

`TornClient` is the core intent API.

It composes:

- `RequestPlanner` (selection batching and endpoint/version planning)
- `RequestExecutor` (HTTP execution, retries, rate limiting, runtime fallback)

Use `TornSdk` for wrapper ergonomics/typed models. Use `TornClient` directly when you want full request control.

## Bootstrap

```rust
use torn_sdk_planner::{ExecutorConfig, TornClient};

let client = TornClient::from_capabilities_file(
    "spec/capabilities.json",
    vec!["your_api_key".to_string()],
    ExecutorConfig::default(),
)?;
```

Or load from `.env`:

```rust
use torn_sdk_planner::TornClient;

let client = TornClient::from_env("spec/capabilities.json")?;
```

## Generic Request

```rust
use torn_sdk_planner::DataRequestOptions;

let report = client
    .get_resource_data(
        "user",
        vec!["profile", "discord"],
        DataRequestOptions::default()
            .with_id("3637232")
            .with_filter("striptags", "true"),
    )
    .await?;
```

## Typed Decode

```rust
#[derive(serde::Deserialize)]
struct UserBundle {
    profile: serde_json::Value,
    discord: serde_json::Value,
}

let data: UserBundle = client
    .get_resource_data_typed(
        "user",
        vec!["profile", "discord"],
        DataRequestOptions::default().with_id("3637232"),
    )
    .await?;
```

## Execution Overrides

```rust
use std::time::Duration;
use torn_sdk_planner::DataRequestOptions;

let report = client
    .get_user_data(
        vec!["profile"],
        DataRequestOptions::default()
            .with_id("3637232")
            .with_max_attempts(2)
            .with_request_timeout(Duration::from_secs(10)),
    )
    .await?;
```

## Runtime Fallback Behavior

Executor behavior preserves planner output and only falls back when needed:

- v2 combined request may split into singles when Torn reports incompatible selection combinations.
- v2 requests may be retried against v1 when Torn returns migration-style API errors.
- v1 fallback scrubs unsupported filters and rewrites path format where needed.

## Production Guidance

- Prefer `TornSdk` typed methods for stable high-value payloads.
- Keep `TornClient` + raw JSON for rapidly changing or niche payloads.
- Set bounded windows (`limit`, `from`, `to`, `offset`) for historical/high-volume selections.
