mod support;

use std::path::PathBuf;
use std::time::Duration;

use support::MockTransport;
use torn_sdk_planner::models::manual::key::KeyLogTargetId;
use torn_sdk_planner::{
    CapabilitiesDocument, ExecutorConfig, KeyOptions, PropertyOptions, RateLimitConfig,
    RequestExecutor, RequestPlanner, SdkError, TornClient, TornSdk, TransportError,
    TransportResponse,
};

fn capabilities_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("spec")
        .join("capabilities.json")
}

fn production_capabilities() -> CapabilitiesDocument {
    CapabilitiesDocument::from_path(capabilities_path()).expect("capabilities should load")
}

fn test_config() -> ExecutorConfig {
    ExecutorConfig {
        base_url_v2: "https://api.torn.com/v2".to_string(),
        base_url_v1: "https://api.torn.com".to_string(),
        timeout: Duration::from_secs(5),
        user_agent: "key-property-contract-test".to_string(),
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
    let planner = RequestPlanner::from_capabilities(production_capabilities());
    let transport = MockTransport::with_responses(responses);
    let executor = RequestExecutor::new(transport.clone(), vec!["k1".to_string()], test_config())
        .expect("executor should initialize");
    let client = TornClient::new(planner, executor);
    (TornSdk::new(client), transport)
}

#[tokio::test]
async fn key_log_and_property_helpers_deserialize_typed_payloads() {
    let (sdk, transport) = make_sdk_with_responses(vec![
        Ok(TransportResponse {
            status: 200,
            body: r#"{"log":[{"timestamp":1710000000,"type":"user","selections":"profile","id":42,"comment":"numeric id","ip":"127.0.0.1"},{"timestamp":1710000300,"type":"property","selections":"lookup","id":"property-1","comment":null,"ip":"127.0.0.2"}]}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"selections":["property","lookup","timestamp"]}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"timestamp":1710000400}"#.to_string(),
        }),
    ]);

    let log = sdk
        .key()
        .log(KeyOptions::default())
        .await
        .expect("key.log should deserialize");
    assert_eq!(log.log.len(), 2);
    assert!(matches!(log.log[0].id, Some(KeyLogTargetId::Numeric(42))));
    assert!(matches!(
        log.log[1].id.as_ref(),
        Some(KeyLogTargetId::Text(value)) if value == "property-1"
    ));

    let lookup = sdk
        .property()
        .lookup(PropertyOptions::default())
        .await
        .expect("property.lookup should deserialize");
    assert_eq!(lookup.selections, vec!["property", "lookup", "timestamp"]);

    let timestamp = sdk
        .property()
        .timestamp(PropertyOptions::default())
        .await
        .expect("property.timestamp should deserialize");
    assert_eq!(timestamp.timestamp, Some(1_710_000_400));

    let requests = transport.requests();
    assert_eq!(requests.len(), 3);
    assert_eq!(requests[0].path, "/key");
    assert_eq!(
        requests[0].query.get("selections"),
        Some(&"log".to_string())
    );
    assert_eq!(requests[1].path, "/property");
    assert_eq!(
        requests[1].query.get("selections"),
        Some(&"lookup".to_string())
    );
    assert_eq!(
        requests[2].query.get("selections"),
        Some(&"timestamp".to_string())
    );
}

#[tokio::test]
async fn property_typed_helpers_preserve_existing_validation_rules() {
    let (sdk, transport) = make_sdk_with_responses(vec![Ok(TransportResponse {
        status: 200,
        body: r#"{"timestamp":1710000500}"#.to_string(),
    })]);

    let error = sdk
        .property()
        .property(PropertyOptions::default())
        .await
        .expect_err("property.property should still require id");
    assert!(matches!(error, SdkError::Validation(_)));

    let timestamp = sdk
        .property()
        .timestamp(PropertyOptions::default())
        .await
        .expect("property.timestamp should not require id");
    assert_eq!(timestamp.timestamp, Some(1_710_000_500));

    let requests = transport.requests();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].path, "/property");
    assert_eq!(
        requests[0].query.get("selections"),
        Some(&"timestamp".to_string())
    );
    assert!(!requests[0].query.contains_key("id"));
}
