mod support;

use std::path::PathBuf;
use std::time::Duration;

use support::MockTransport;
use torn_sdk_planner::{
    BaseOptions, ExecutorConfig, MarketOptions, RateLimitConfig, RequestExecutor, RequestPlanner,
    SdkError, TornClient, TornSdk, TransportError, TransportResponse,
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
        user_agent: "market-contract-test".to_string(),
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
) -> (TornSdk<MockTransport>, MockTransport) {
    let planner = RequestPlanner::from_capabilities_file(production_capabilities_path())
        .expect("capabilities should load");
    let transport = MockTransport::with_responses(responses);
    let executor = RequestExecutor::new(transport.clone(), vec!["k1".to_string()], test_config())
        .expect("executor should initialize");
    (TornSdk::new(TornClient::new(planner, executor)), transport)
}

#[tokio::test]
async fn typed_market_helpers_deserialize_remaining_stable_shapes() {
    let (sdk, _) = make_sdk_with_responses(vec![
        Ok(TransportResponse {
            status: 200,
            body: r#"{"timestamp":1710000000}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"selections":["properties","rentals","lookup","timestamp","pointsmarket","auctionhouse","auctionhouselisting"]}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"pointsmarket":{"20085593":{"cost":33395,"quantity":500,"total_cost":16697500}}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"properties":{"listings":[{"happy":165,"cost":5999,"market_price":7500,"upkeep":10,"modifications":["Superior Interior Modification"]}],"property":{"id":1,"name":"Trailer"}},"properties_timestamp":1710000100,"properties_delay":30,"_metadata":{"links":{"next":null,"prev":null},"total":1}}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"rentals":{"listings":[{"happy":165,"cost":1,"cost_per_day":0,"rental_period":3,"market_price":7500,"upkeep":10,"modifications":["Superior Interior Modification"]}],"property":{"id":1,"name":"Trailer"}},"rentals_timestamp":1710000200,"rentals_delay":30,"_metadata":{"links":{"next":null,"prev":null},"total":1}}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"auctionhouse":[{"id":514309,"seller":{"id":2466470,"name":"Hwa"},"buyer":{"id":3166898,"name":"Melchizedek7"},"timestamp":1777419066,"price":64000001,"bids":7,"item":{"id":219,"uid":10772039092,"name":"Enfield SA-80","type":"Weapon","sub_type":"Rifle","stats":{"damage":69.65,"accuracy":59.99,"armor":null,"quality":116.38},"bonuses":[{"id":86,"title":"Disarm","description":"Disables an opponent's weapon upon a hand or arm hit for 4 turns","value":4}],"rarity":"yellow"}}],"_metadata":{"links":{"next":null,"prev":null}}}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"auctionhouselisting":{"id":514309,"seller":{"id":2466470,"name":"Hwa"},"buyer":{"id":3166898,"name":"Melchizedek7"},"timestamp":1777419066,"price":64000001,"bids":7,"item":{"id":219,"uid":10772039092,"name":"Enfield SA-80","type":"Weapon","sub_type":"Rifle","stats":{"damage":69.65,"accuracy":59.99,"armor":null,"quality":116.38},"bonuses":[{"id":86,"title":"Disarm","description":"Disables an opponent's weapon upon a hand or arm hit for 4 turns","value":4}],"rarity":"yellow"}}}"#
                .to_string(),
        }),
    ]);

    let timestamp = sdk
        .market()
        .timestamp(MarketOptions::default())
        .await
        .expect("timestamp should deserialize");
    assert_eq!(timestamp.timestamp, Some(1_710_000_000));

    let lookup = sdk
        .market()
        .lookup(MarketOptions::default())
        .await
        .expect("lookup should deserialize");
    assert!(
        lookup
            .selections
            .iter()
            .any(|selection| selection == "pointsmarket")
    );

    let pointsmarket = sdk
        .market()
        .pointsmarket(MarketOptions::default())
        .await
        .expect("pointsmarket should deserialize");
    let first_listing = pointsmarket
        .pointsmarket
        .get("20085593")
        .expect("listing should exist");
    assert_eq!(first_listing.cost, Some(33_395));
    assert_eq!(first_listing.quantity, Some(500));

    let properties = sdk
        .market()
        .properties(
            MarketOptions::default()
                .with_property_type_id("1")
                .with_base(BaseOptions::default().with_limit(1)),
        )
        .await
        .expect("properties should deserialize");
    assert_eq!(
        properties
            .properties
            .property
            .as_ref()
            .and_then(|item| item.id),
        Some(1)
    );
    assert_eq!(properties.properties_delay, Some(30));
    assert_eq!(properties.properties.listings[0].cost, Some(5_999));

    let rentals = sdk
        .market()
        .rentals(
            MarketOptions::default()
                .with_property_type_id("1")
                .with_base(BaseOptions::default().with_limit(1)),
        )
        .await
        .expect("rentals should deserialize");
    assert_eq!(rentals.rentals_delay, Some(30));
    assert_eq!(rentals.rentals.listings[0].rental_period, Some(3));

    let auctionhouse = sdk
        .market()
        .auctionhouse(MarketOptions::default().with_base(BaseOptions::default().with_limit(1)))
        .await
        .expect("auctionhouse should deserialize");
    assert_eq!(auctionhouse.auctionhouse.len(), 1);
    assert_eq!(auctionhouse.auctionhouse[0].id, Some(514_309));
    assert_eq!(
        auctionhouse.auctionhouse[0]
            .item
            .as_ref()
            .and_then(|item| item.sub_type.as_deref()),
        Some("Rifle")
    );

    let auctionhouselisting = sdk
        .market()
        .auctionhouselisting(MarketOptions::default().with_id("514309"))
        .await
        .expect("auctionhouselisting should deserialize");
    assert_eq!(auctionhouselisting.auctionhouselisting.id, Some(514_309));
    assert_eq!(
        auctionhouselisting
            .auctionhouselisting
            .seller
            .as_ref()
            .and_then(|seller| seller.name.as_deref()),
        Some("Hwa")
    );
}

#[tokio::test]
async fn market_auctionhouse_supports_collection_requests_without_id() {
    let (sdk, transport) = make_sdk_with_responses(vec![Ok(TransportResponse {
        status: 200,
        body: r#"{"auctionhouse":[],"_metadata":{"links":{"next":null,"prev":null}}}"#.to_string(),
    })]);

    sdk.market()
        .auctionhouse(MarketOptions::default().with_base(BaseOptions::default().with_limit(1)))
        .await
        .expect("auctionhouse should allow collection requests without an id");

    let requests = transport.requests();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].path, "/market");
    assert_eq!(
        requests[0].query.get("selections"),
        Some(&"auctionhouse".to_string())
    );
    assert_eq!(requests[0].query.get("limit"), Some(&"1".to_string()));
    assert!(!requests[0].query.contains_key("id"));
}

#[tokio::test]
async fn market_typed_helpers_preserve_validation_boundaries() {
    let (sdk, _) = make_sdk_with_responses(vec![]);

    let properties_error = sdk
        .market()
        .properties(MarketOptions::default())
        .await
        .expect_err("properties should require propertyTypeId");
    match properties_error {
        SdkError::Validation(message) => assert!(message.contains("propertyTypeId")),
        other => panic!("unexpected error: {other:?}"),
    }

    let listing_error = sdk
        .market()
        .auctionhouselisting(MarketOptions::default())
        .await
        .expect_err("auctionhouselisting should require id");
    match listing_error {
        SdkError::Validation(message) => assert!(message.contains("requires 'id'")),
        other => panic!("unexpected error: {other:?}"),
    }
}
