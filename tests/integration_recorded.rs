use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use torn_sdk_planner::{
    ApiVersion, CapabilitiesDocument, DataRequestOptions, ExecutorConfig, HttpTransport,
    RateLimitConfig, RequestExecutor, RequestPlanner, TornClient, TransportError, TransportRequest,
    TransportResponse,
};

#[derive(Debug)]
struct MockTransportState {
    responses: Mutex<VecDeque<Result<TransportResponse, TransportError>>>,
    requests: Mutex<Vec<TransportRequest>>,
}

#[derive(Debug, Clone)]
struct MockTransport {
    state: Arc<MockTransportState>,
}

impl MockTransport {
    fn with_responses(responses: Vec<Result<TransportResponse, TransportError>>) -> Self {
        Self {
            state: Arc::new(MockTransportState {
                responses: Mutex::new(responses.into()),
                requests: Mutex::new(Vec::new()),
            }),
        }
    }

    fn requests(&self) -> Vec<TransportRequest> {
        self.state
            .requests
            .lock()
            .expect("requests lock should not be poisoned")
            .clone()
    }
}

impl HttpTransport for MockTransport {
    async fn execute(
        &self,
        request: &TransportRequest,
    ) -> Result<TransportResponse, TransportError> {
        self.state
            .requests
            .lock()
            .expect("requests lock should not be poisoned")
            .push(request.clone());
        self.state
            .responses
            .lock()
            .expect("responses lock should not be poisoned")
            .pop_front()
            .expect("missing mocked response")
    }
}

fn recorded_capabilities() -> CapabilitiesDocument {
    let json = r#"
{
  "spec": { "version": "5.4.1" },
  "resources": {
    "user": {
      "generic_path": "/user",
      "generic_parameters": [
        { "name": "selections" },
        { "name": "id" },
        { "name": "striptags" },
        { "name": "limit" }
      ],
      "selections": [
        {
          "name": "profile",
          "can_use_generic_endpoint": true,
          "endpoints": ["/user/profile"]
        },
        {
          "name": "discord",
          "can_use_generic_endpoint": true,
          "endpoints": ["/user/discord"]
        },
        {
          "name": "hof",
          "can_use_generic_endpoint": true,
          "endpoints": ["/user/hof"]
        },
        {
          "name": "networth",
          "can_use_generic_endpoint": true,
          "endpoints": ["/user/networth"]
        }
      ]
    }
  }
}
"#;
    CapabilitiesDocument::from_json_str(json).expect("fixture should parse")
}

fn test_config() -> ExecutorConfig {
    ExecutorConfig {
        base_url_v2: "https://api.torn.com/v2".to_string(),
        base_url_v1: "https://api.torn.com".to_string(),
        timeout: Duration::from_secs(5),
        user_agent: "recorded-integration-test".to_string(),
        max_attempts: 1,
        network_retry_backoff: Duration::from_millis(1),
        rate_limits: RateLimitConfig {
            per_key_per_minute: 1000,
            per_ip_per_minute: 1000,
        },
        max_in_flight: 4,
    }
}

#[tokio::test]
async fn combines_selections_against_recorded_v2_payload() {
    let planner = RequestPlanner::from_capabilities(recorded_capabilities());
    let transport = MockTransport::with_responses(vec![Ok(TransportResponse {
        status: 200,
        body: include_str!("fixtures/user_profile_discord_hof_v2.json").to_string(),
    })]);
    let executor = RequestExecutor::new(transport.clone(), vec!["k1".to_string()], test_config())
        .expect("executor should initialize");
    let client = TornClient::new(planner, executor);

    let report = client
        .get_user_data(
            vec!["profile", "discord", "hof"],
            DataRequestOptions::default()
                .with_id("3637232")
                .with_filter("striptags", "true"),
        )
        .await
        .expect("request should succeed");

    assert_eq!(report.calls.len(), 1);
    assert_eq!(report.merged_json["profile"]["player_id"], 3637232);
    assert_eq!(report.merged_json["discord"]["discordID"], "sample#0001");
    assert_eq!(report.merged_json["hof"]["rank"], 42);
}

#[tokio::test]
async fn uses_runtime_v2_to_v1_fallback_with_recorded_payloads() {
    let planner = RequestPlanner::from_capabilities(recorded_capabilities());
    let transport = MockTransport::with_responses(vec![
        Ok(TransportResponse {
            status: 400,
            body: include_str!("fixtures/v2_migration_error_networth.json").to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: include_str!("fixtures/user_networth_v1.json").to_string(),
        }),
    ]);
    let executor = RequestExecutor::new(transport.clone(), vec!["k1".to_string()], test_config())
        .expect("executor should initialize");
    let client = TornClient::new(planner, executor);

    let report = client
        .get_user_data(
            vec!["networth"],
            DataRequestOptions::default()
                .with_id("3637232")
                .with_filter("limit", "10")
                .with_filter("foo", "bar"),
        )
        .await
        .expect("request should succeed");

    assert_eq!(report.calls.len(), 1);
    assert!(report.calls[0].fallback);
    assert_eq!(report.calls[0].api_version, ApiVersion::V1);
    assert_eq!(report.merged_json["networth"]["total"], 1234567890_u64);

    let requests = transport.requests();
    assert_eq!(requests.len(), 2);
    assert_eq!(requests[1].path, "/user/3637232");
    assert_eq!(
        requests[1].query.get("selections"),
        Some(&"networth".to_string())
    );
    assert_eq!(requests[1].query.get("limit"), Some(&"10".to_string()));
    assert!(!requests[1].query.contains_key("id"));
    assert!(!requests[1].query.contains_key("foo"));
}
