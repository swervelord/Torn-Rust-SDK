# Torn Rust SDK

`torn_sdk_planner` is a production-focused Torn API SDK for Rust with:

- selection-aware request planning (v2 generic batching + direct endpoint routing),
- runtime resilience (retries, rate limiting, v2->v1 fallback),
- typed wrappers for high-value endpoints,
- raw fallback for volatile/long-tail selections.

## Quickstart

```rust
use torn_sdk_planner::{TornClient, TornSdk};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdk = TornSdk::new(TornClient::from_env("spec/capabilities.json")?);
    let profile = sdk.user().profile("3637232").await?;
    println!("{:?}", profile.profile.name);
    Ok(())
}
```

## Typed vs Raw

- Use typed methods for stable, high-value flows (`user.profile`, `faction.basic`, `torn.calendar`).
- Use `*_raw`, `raw_selection`, or `raw_selections` for long-tail or fast-changing payloads.
- Use `sdk.client()` when you need full control over selections/options.

## Docs

- [Docs Index](docs/README.md)
- [Client API](docs/guides/CLIENT_API.md)
- [Wrapper Architecture](docs/architecture/WRAPPER_ARCHITECTURE.md)
- [SDK Examples](docs/guides/SDK_EXAMPLES.md)
- [Errors](docs/guides/ERRORS.md)
- [Environment](docs/guides/ENVIRONMENT.md)
- [Publishing](docs/release/PUBLISHING.md)
- [Release Checklist](docs/release/RELEASE_CHECKLIST.md)

## MSRV

- Minimum supported Rust version (MSRV): `1.85`.
