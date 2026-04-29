#[allow(dead_code)]
mod support;

use std::path::PathBuf;
use std::time::Duration;

use support::MockTransport;
use torn_sdk_planner::{
    BaseOptions, DataRequestOptions, ExecutorConfig, ForumOptions, KeyOptions, MarketOptions,
    PropertyOptions, RacingOptions, RateLimitConfig, RequestExecutor, RequestPlanner, TornClient,
    TornOptions, TornSdk, TransportError, TransportResponse, UserOptions,
};

fn production_capabilities_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("spec")
        .join("capabilities.json")
}

fn test_config() -> ExecutorConfig {
    ExecutorConfig {
        base_url_v2: "https://api.torn.com/v2".to_string(),
        base_url_v1: "https://api.torn.com".to_string(),
        timeout: Duration::from_secs(5),
        user_agent: "docs-contract-test".to_string(),
        max_attempts: 1,
        network_retry_backoff: Duration::from_millis(1),
        rate_limits: RateLimitConfig {
            per_key_per_minute: 1000,
            per_ip_per_minute: 1000,
        },
        max_in_flight: 4,
    }
}

fn make_sdk_with_responses(
    responses: Vec<Result<TransportResponse, TransportError>>,
) -> TornSdk<MockTransport> {
    let planner = RequestPlanner::from_capabilities_file(production_capabilities_path())
        .expect("capabilities should load");
    let transport = MockTransport::with_responses(responses);
    let executor = RequestExecutor::new(transport, vec!["k1".to_string()], test_config())
        .expect("executor should initialize");
    TornSdk::new(TornClient::new(planner, executor))
}

#[tokio::test]
async fn docs_examples_compile_and_execute() {
    let sdk = make_sdk_with_responses(vec![
        Ok(TransportResponse {
            status: 200,
            body: r#"{"profile":{"id":3637232,"name":"Example User"}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"money":{"wallet":1200,"points":50}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"casino":{"tokens":5,"streak":-1}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"inventory":{"items":[{"id":788,"amount":1,"equipped":false,"name":"Certificate of Awesome","uid":null,"faction_owned":false}],"timestamp":1710000000},"_metadata":{"links":{"next":null,"prev":null},"total":1}}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"trades":[{"id":12036957,"timestamp":false,"description":"g","user":{"id":3637232,"name":"Swervelord"},"trader":{"id":3054822,"name":"Miya"}}],"_metadata":{"links":{"next":null,"prev":null}}}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"trade":{"id":42,"timestamp":1710000100,"description":"sample","type":"private","user":{"id":3637232,"name":"Swervelord"},"trader":{"id":3054822,"name":"Miya"},"items":[{"user_id":3054822,"type":"Money","details":{"amount":500000}}]}}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"itemmarket":{"item":{"id":2,"name":"Item","type":"Primary"},"listings":[{"amount":1,"price":100}]}}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"calendar":{"events":[],"competitions":[]}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"posts":[{"id":987,"thread_id":16559714,"created_time":1710001000,"content":"Hello","author":{"id":1,"username":"Poster"}}],"_metadata":{"links":{"next":null,"prev":null}}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"log":[{"timestamp":1710000000,"type":"user","selections":"profile,bars","id":"3637232","comment":"typed smoke"}]}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"selections":["lookup","property","timestamp"]}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"records":[{"driver_id":1,"driver_name":"Racer","car_item_id":10,"car_item_name":"Cosworth","lap_time":62.41}]}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"pointsmarket":{"20085593":{"cost":33395,"quantity":500,"total_cost":16697500}}}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"events":[{"id":1}]}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"networth":{"total":123456}}"#.to_string(),
        }),
    ]);

    let profile = sdk
        .user()
        .profile("3637232")
        .await
        .expect("profile should deserialize");
    assert_eq!(profile.profile.id, Some(3637232));

    let money = sdk
        .user()
        .money(UserOptions::default())
        .await
        .expect("money should deserialize");
    assert_eq!(money.money.wallet, Some(1200));

    let casino = sdk
        .user()
        .casino()
        .await
        .expect("casino should deserialize");
    assert_eq!(casino.casino.tokens, Some(5));

    let inventory = sdk
        .user()
        .inventory(
            UserOptions::default()
                .with_base(BaseOptions::default().with_cat("Other").with_limit(1)),
        )
        .await
        .expect("inventory should deserialize");
    assert_eq!(inventory.inventory.items.len(), 1);

    let trades = sdk
        .user()
        .trades(
            UserOptions::default()
                .with_base(BaseOptions::default().with_cat("ongoing").with_limit(1)),
        )
        .await
        .expect("trades should deserialize");
    assert_eq!(trades.trades.len(), 1);

    let trade = sdk
        .user()
        .trade(UserOptions::default().with_trade_id("42"))
        .await
        .expect("trade should deserialize");
    assert_eq!(trade.trade.id, Some(42));

    let itemmarket = sdk
        .market()
        .itemmarket(MarketOptions::default().with_id("2"))
        .await
        .expect("itemmarket should deserialize");
    assert_eq!(
        itemmarket.itemmarket.item.as_ref().and_then(|item| item.id),
        Some(2)
    );

    let calendar = sdk
        .torn()
        .calendar(TornOptions::default())
        .await
        .expect("calendar should deserialize");
    assert!(calendar.calendar.events.is_empty());

    let posts = sdk
        .forum()
        .posts(ForumOptions::default().with_thread_id("16559714"))
        .await
        .expect("posts should deserialize");
    assert_eq!(posts.posts.len(), 1);

    let key_log = sdk
        .key()
        .log(KeyOptions::default().with_base(BaseOptions::default().with_limit(25)))
        .await
        .expect("key log should deserialize");
    assert_eq!(key_log.log.len(), 1);

    let property_lookup = sdk
        .property()
        .lookup(PropertyOptions::default())
        .await
        .expect("property lookup should deserialize");
    assert_eq!(property_lookup.selections.len(), 3);

    let racing_records = sdk
        .racing()
        .records(
            RacingOptions::default()
                .with_track_id("6")
                .with_base(BaseOptions::default().with_cat("D")),
        )
        .await
        .expect("racing records should deserialize");
    assert_eq!(racing_records.records.len(), 1);

    let points = sdk
        .market()
        .pointsmarket(MarketOptions::default())
        .await
        .expect("points market should deserialize");
    assert_eq!(points.pointsmarket.len(), 1);

    let raw = sdk
        .user()
        .events_raw(
            UserOptions::default().with_base(
                BaseOptions::default()
                    .with_from(1_700_000_000)
                    .with_to(1_700_000_300)
                    .with_limit(10),
            ),
        )
        .await
        .expect("events raw should succeed");
    assert!(raw.get("events").is_some());

    let report = sdk
        .client()
        .get_user_data(
            vec!["networth"],
            DataRequestOptions::default().with_id("3637232"),
        )
        .await
        .expect("escape hatch call should succeed");
    assert!(report.merged_json.get("networth").is_some());
}
