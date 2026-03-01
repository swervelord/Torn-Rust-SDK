#[allow(dead_code)]
mod support;

use std::path::PathBuf;
use std::time::Duration;

use support::MockTransport;
use torn_sdk_planner::{
    BaseOptions, DataRequestOptions, ExecutorConfig, MarketOptions, RateLimitConfig,
    RequestExecutor, RequestPlanner, TornClient, TornOptions, TornSdk, TransportError,
    TransportResponse, UserOptions,
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
            body: r#"{"itemmarket":{"item":{"id":2,"name":"Item","type":"Primary"},"listings":[{"amount":1,"price":100}]}}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"calendar":{"events":[],"competitions":[]}}"#.to_string(),
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
