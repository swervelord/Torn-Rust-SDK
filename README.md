# Torn Rust SDK

`torn_sdk_planner` is a planner-driven Torn API SDK for Rust with:

- selection-aware routing across generic v2 requests, direct v2 endpoints, and controlled v1 fallback,
- resilient execution with retries, rate limiting, and fallback handling,
- typed wrapper helpers across the complete product selection inventory,
- raw escape hatches when a selection is volatile, underspecified, or not typed yet.

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

## Coverage Snapshot

Completion-state typed wrapper coverage is `189 / 189` capability selections.

- `company`: `9 / 9`
- `user`: `68 / 68`
- `faction`: `44 / 44`
- `forum`: `6 / 6`
- `key`: `2 / 2`
- `market`: `9 / 9`
- `property`: `3 / 3`
- `racing`: `8 / 8`
- `torn`: `40 / 40`

Notes:

- This is product-scope accounting for the finished wrapper surface, not just the subset already merged in the current tree.
- The completion delta from the older `168 / 180` snapshot is the new `company` surface (`9 / 9`) plus the remaining `faction` tranche (`armor`, `boosters`, `caches`, `cesium`, `crime`, `crimeexp`, `crimes`, `drugs`, `medical`, `temporary`, `utilities`, `weapons`).
- Counts are based on one first-class typed wrapper helper per capability selection.
- Helper variants such as `torn.stock(...)` are not counted as extra selections on top of `torn.stocks`.
- Raw helpers remain available across every resource even when a typed helper already exists.

The detailed selection-by-selection view lives in [docs/TYPED_COVERAGE_STATUS.md](docs/TYPED_COVERAGE_STATUS.md).

## Typed Surface

At product completion the SDK exposes a first-class typed helper for every known resource selection, while still using conservative facades where Torn payloads are volatile or v1-backed. Representative typed helpers include:

- `user.casino`, `user.inventory`, `user.trades`, `user.trade`, `user.attacks`, `user.revives`, `user.ammo`, `user.equipment`, `user.log`, `user.networth`, `user.personalstats`
- `company.profile`, `company.employees`, `company.applications`, `company.detailed`, `company.stock`, `company.news`
- `faction.attacks`, `faction.attacksfull`, `faction.chain`, `faction.chains`, `faction.chainreport`, `faction.crime`, `faction.crimes`, `faction.crimeexp`, `faction.raids`, `faction.raidreport`, `faction.rankedwarreport`, `faction.reports`, `faction.revives`, `faction.revivesfull`, `faction.territory`, `faction.territoryownership`, `faction.territorywars`, `faction.territorywarreport`, `faction.warfare`, `faction.news`
- `forum.categories`, `forum.threads`, `forum.thread`, `forum.posts`
- `key.info`, `key.log`
- `market.pointsmarket`, `market.properties`, `market.rentals`, `market.itemmarket`, `market.auctionhouse`, `market.auctionhouselisting`
- `property.property`, `property.lookup`, `property.timestamp`
- `racing.tracks`, `racing.cars`, `racing.carupgrades`, `racing.races`, `racing.race`, `racing.records`
- `torn` catalog and reference helpers such as `calendar`, `items`, `honors`, `medals`, `logcategories`, `logtypes`, `merits`, `itemammo`, `itemmods`, `properties`, `stocks`, `stock`, `bounties`, `attacklog`, `itemstats`, `itemdetails`, `eliminationteam`, plus conservative typed facades for v1-backed selections

## Typed vs Raw

Use typed helpers when you want the maintained public facade. Drop to raw when you need batching across selections, future API additions, unstable fields, or a payload the SDK has not modeled yet.

```rust
use torn_sdk_planner::{
    BaseOptions, ForumOptions, KeyOptions, MarketOptions, PropertyOptions, RacingOptions,
    SortOrder, UserOptions,
};

let casino = sdk.user().casino().await?;

let inventory = sdk
    .user()
    .inventory(
        UserOptions::default().with_base(
            BaseOptions::default()
                .with_cat("Other")
                .with_limit(25)
                .with_offset(0),
        ),
    )
    .await?;

let trades = sdk
    .user()
    .trades(
        UserOptions::default().with_base(
            BaseOptions::default()
                .with_cat("completed")
                .with_sort(SortOrder::Desc)
                .with_limit(10),
        ),
    )
    .await?;

let posts = sdk
    .forum()
    .posts(ForumOptions::default().with_thread_id("16559714"))
    .await?;

let key_log = sdk
    .key()
    .log(KeyOptions::default().with_base(BaseOptions::default().with_limit(25)))
    .await?;

let property_lookup = sdk.property().lookup(PropertyOptions::default()).await?;

let records = sdk
    .racing()
    .records(
        RacingOptions::default()
            .with_track_id("6")
            .with_base(BaseOptions::default().with_cat("D")),
    )
    .await?;

let points = sdk.market().pointsmarket(MarketOptions::default()).await?;

println!(
    "casino_tokens={:?} inventory={} trades={} posts={} key_log={} property_lookup={} racing_records={} points_listings={}",
    casino.casino.tokens,
    inventory.inventory.items.len(),
    trades.trades.len(),
    posts.posts.len(),
    key_log.log.len(),
    property_lookup.selections.len(),
    records.records.len(),
    points.pointsmarket.len()
);
```

Paged typed responses preserve Torn's `_metadata` envelope so callers can keep paging via `prev` and `next` links without dropping to raw JSON.

For raw access:

```rust
use torn_sdk_planner::{BaseOptions, DataRequestOptions, UserOptions};

let raw_events = sdk
    .user()
    .events_raw(
        UserOptions::default().with_base(
            BaseOptions::default()
                .with_from(1_700_000_000)
                .with_to(1_700_000_300)
                .with_limit(10),
        ),
    )
    .await?;

let direct_report = sdk
    .client()
    .get_user_data(
        vec!["networth"],
        DataRequestOptions::default().with_id("3637232"),
    )
    .await?;

assert!(raw_events.get("events").is_some());
assert!(direct_report.merged_json.get("networth").is_some());
```

## Docs

- [Docs Index](docs/README.md)
- [SDK Examples](docs/guides/SDK_EXAMPLES.md)
- [Typed Coverage Status](docs/TYPED_COVERAGE_STATUS.md)
- [Typed Model Strategy](docs/architecture/TYPED_MODEL_STRATEGY.md)
- [Client API](docs/guides/CLIENT_API.md)
- [Errors](docs/guides/ERRORS.md)
- [Environment](docs/guides/ENVIRONMENT.md)
- [Wrapper Architecture](docs/architecture/WRAPPER_ARCHITECTURE.md)

## MSRV

Minimum supported Rust version: `1.85`.
