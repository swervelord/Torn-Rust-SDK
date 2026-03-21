# SDK Examples

## 1. Auth + Bootstrap

```rust
use torn_sdk_planner::{TornClient, TornSdk};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdk = TornSdk::new(TornClient::from_env("spec/capabilities.json")?);
    let profile = sdk.user().profile("3637232").await?;
    println!("name={:?}", profile.profile.name);
    Ok(())
}
```

## 2. Typed Methods (High-Value Endpoints)

```rust
use torn_sdk_planner::{FactionOptions, MarketOptions, TornOptions, UserOptions};

let user_basic = sdk.user().basic("3637232").await?;
let user_money = sdk.user().money(UserOptions::default()).await?;

let faction_basic = sdk
    .faction()
    .basic(FactionOptions::default().with_id("12345"))
    .await?;

let itemmarket = sdk
    .market()
    .itemmarket(MarketOptions::default().with_id("2"))
    .await?;

let calendar = sdk.torn().calendar(TornOptions::default()).await?;
println!(
    "{:?} {:?} {:?} {:?}",
    user_basic.profile.id,
    user_money.money.wallet,
    faction_basic.basic.name,
    calendar.calendar.events.len()
);
```

## 3. Raw Methods (Volatile/Long-Tail Endpoints)

```rust
use torn_sdk_planner::{BaseOptions, UserOptions};

let raw = sdk
    .user()
    .events_raw(
        UserOptions::default().with_base(
            BaseOptions::default()
                .with_from(1_700_000_000)
                .with_to(1_700_003_600)
                .with_limit(100)
                .with_striptags(true),
        ),
    )
    .await?;

println!("{:?}", raw.get("events"));
```

## 4. Validation Errors (Client-Side)

```rust
use torn_sdk_planner::{MarketOptions, SdkError};

match sdk.market().bazaar(MarketOptions::default()).await {
    Err(SdkError::Validation(message)) => {
        // "resource 'market' selection 'bazaar' requires 'id' to be provided"
        eprintln!("{message}");
    }
    other => println!("{other:?}"),
}
```

## 5. Escape Hatch (`TornClient`)

```rust
use torn_sdk_planner::DataRequestOptions;

let report = sdk
    .client()
    .get_user_data(
        vec!["networth"],
        DataRequestOptions::default()
            .with_id("3637232")
            .with_filter("limit", "1"),
    )
    .await?;

println!("calls={}", report.calls.len());
```
