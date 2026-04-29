# SDK Examples

These examples focus on the current typed wrapper surface. They are intentionally small and mirror the shapes covered by the contract tests.

## Setup

```rust
use torn_sdk_planner::{TornClient, TornSdk};

let sdk = TornSdk::new(TornClient::from_env("spec/capabilities.json")?);
```

## User: Casino, Inventory, Trades

```rust
use torn_sdk_planner::{BaseOptions, SortOrder, UserOptions};

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

if let Some(trade_id) = trades.trades.first().and_then(|trade| trade.id) {
    let trade = sdk
        .user()
        .trade(UserOptions::default().with_trade_id(trade_id.to_string()))
        .await?;

    println!(
        "tokens={:?} inventory_items={} trades={} first_trade_items={}",
        casino.casino.tokens,
        inventory.inventory.items.len(),
        trades.trades.len(),
        trade.trade.items.len()
    );
}
```

Notes:

- `user.inventory` currently needs a `cat` in practice.
- `user.trades` preserves `_metadata` for paging.
- `user.trade` is a typed detail call driven by `UserOptions::with_trade_id(...)`.

## Forum: Threads And Posts

```rust
use torn_sdk_planner::{BaseOptions, ForumOptions};

let threads = sdk
    .forum()
    .threads(ForumOptions::default().with_base(BaseOptions::default().with_limit(25)))
    .await?;

let posts = sdk
    .forum()
    .posts(ForumOptions::default().with_thread_id("16559714"))
    .await?;

println!(
    "threads={} next_page={:?} posts={}",
    threads.threads.len(),
    threads
        ._metadata
        .as_ref()
        .and_then(|metadata| metadata.links.as_ref())
        .and_then(|links| links.next.as_deref()),
    posts.posts.len()
);
```

## Key: Access Log

```rust
use torn_sdk_planner::{BaseOptions, KeyOptions};

let log = sdk
    .key()
    .log(KeyOptions::default().with_base(BaseOptions::default().with_limit(25)))
    .await?;

for entry in &log.log {
    println!("{:?} {:?} {:?}", entry.timestamp, entry.log_type, entry.selections);
}
```

## Market And Property

```rust
use torn_sdk_planner::{BaseOptions, MarketOptions, PropertyOptions};

let points = sdk.market().pointsmarket(MarketOptions::default()).await?;

let auctionhouse = sdk
    .market()
    .auctionhouse(MarketOptions::default().with_base(BaseOptions::default().with_limit(10)))
    .await?;

let property_lookup = sdk.property().lookup(PropertyOptions::default()).await?;

println!(
    "points_listings={} auction_rows={} property_selections={}",
    points.pointsmarket.len(),
    auctionhouse.auctionhouse.len(),
    property_lookup.selections.len()
);
```

## Racing

```rust
use torn_sdk_planner::{BaseOptions, RacingOptions};

let tracks = sdk.racing().tracks(RacingOptions::default()).await?;

let records = sdk
    .racing()
    .records(
        RacingOptions::default()
            .with_track_id("6")
            .with_base(BaseOptions::default().with_cat("D")),
    )
    .await?;

println!(
    "tracks={} records={}",
    tracks.tracks.len(),
    records.records.len()
);
```

## Torn Catalogs And Reference Data

```rust
use torn_sdk_planner::TornOptions;

let logtypes = sdk.torn().logtypes(TornOptions::default()).await?;
let stocks = sdk.torn().stocks(TornOptions::default()).await?;

println!(
    "logtypes={} stocks={}",
    logtypes.logtypes.len(),
    stocks.stocks.len()
);
```

## Raw Escape Hatch

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
