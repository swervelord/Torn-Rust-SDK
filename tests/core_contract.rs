use std::collections::{BTreeSet, VecDeque};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use serde::Deserialize;
use torn_sdk_planner::{
    BaseOptions, CapabilitiesDocument, DataRequestOptions, ExecutionOptions, ExecutorConfig,
    FactionApi, FactionOptions, ForumApi, ForumOptions, HttpTransport, KeyApi, KeyOptions,
    MarketApi, MarketOptions, PropertyApi, PropertyOptions, RacingApi, RacingOptions,
    RateLimitConfig, RequestExecutor, RequestPlanner, SdkError, TornApi, TornClient, TornOptions,
    TornSdk, TransportError, TransportRequest, TransportResponse, UserApi, UserOptions,
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

fn fixture_capabilities() -> CapabilitiesDocument {
    let json = r#"
{
  "spec": { "version": "5.4.1" },
  "resources": {
    "user": {
      "generic_path": "/user",
      "generic_parameters": [
        { "name": "selections" },
        { "name": "id" },
        { "name": "striptags" }
      ],
      "selections": [
        {
          "name": "profile",
          "can_use_generic_endpoint": true,
          "endpoints": ["/user/profile"]
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
        user_agent: "core-contract-test".to_string(),
        max_attempts: 1,
        network_retry_backoff: Duration::from_millis(1),
        rate_limits: RateLimitConfig {
            per_key_per_minute: 1000,
            per_ip_per_minute: 1000,
        },
        max_in_flight: 4,
    }
}

#[derive(Debug, Deserialize)]
struct UserProfileBundle {
    profile: serde_json::Value,
}

#[tokio::test]
async fn core_contract_supports_typed_requests_with_execution_overrides() {
    let planner = RequestPlanner::from_capabilities(fixture_capabilities());
    let transport = MockTransport::with_responses(vec![Ok(TransportResponse {
        status: 200,
        body: r#"{"profile":{"player_id":3637232}}"#.to_string(),
    })]);

    let executor = RequestExecutor::new(transport.clone(), vec!["k1".to_string()], test_config())
        .expect("executor should initialize");
    let client = TornClient::new(planner, executor);

    let data: UserProfileBundle = client
        .get_resource_data_typed(
            "user",
            vec!["profile"],
            DataRequestOptions::default()
                .with_id("3637232")
                .with_filter("striptags", "true")
                .with_execution_options(
                    ExecutionOptions::default()
                        .with_max_attempts(2)
                        .with_request_timeout(Duration::from_secs(2)),
                ),
        )
        .await
        .expect("typed request should succeed");

    assert_eq!(data.profile["player_id"], 3637232);

    let requests = transport.requests();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].path, "/user");
    assert_eq!(
        requests[0].query.get("selections"),
        Some(&"profile".to_string())
    );
    assert_eq!(requests[0].timeout, Some(Duration::from_secs(2)));
}

fn capabilities_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("spec")
        .join("capabilities.json")
}

fn production_capabilities() -> CapabilitiesDocument {
    CapabilitiesDocument::from_path(capabilities_path()).expect("capabilities should load")
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
async fn wrapper_typed_methods_deserialize_for_each_resource() {
    let (sdk, _) = make_sdk_with_responses(vec![
        Ok(TransportResponse {
            status: 200,
            body: r#"{"profile":{"id":3637232,"name":"Example User","level":15}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"basic":{"id":12345,"name":"Faction Name","members":42}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"thread":{"id":777,"title":"Contract Thread","posts":12}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"info":{"access":{"type":"public","level":3},"selections":{"user":["profile"]}}}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"bazaar":{"advanced_item":[{"id":1,"name":"Bazaar A","is_open":true}]}}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"property":{"id":1,"status":"Owned","owner":{"id":99,"name":"Owner"}}}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"race":{"id":9001,"laps":10,"participants":{"current":5}}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"calendar":{"events":[]}}"#.to_string(),
        }),
    ]);

    let user = sdk
        .user()
        .profile("3637232")
        .await
        .expect("user profile should deserialize");
    assert_eq!(user.profile.id, Some(3637232));
    assert_eq!(user.profile.name.as_deref(), Some("Example User"));

    let faction = sdk
        .faction()
        .basic(FactionOptions::default().with_id("12345"))
        .await
        .expect("faction basic should deserialize");
    assert_eq!(faction.basic.id, Some(12345));
    assert_eq!(faction.basic.name.as_deref(), Some("Faction Name"));

    let forum = sdk
        .forum()
        .thread(ForumOptions::default().with_thread_id("777"))
        .await
        .expect("forum thread should deserialize");
    assert_eq!(forum.thread.id, Some(777));
    assert_eq!(forum.thread.title.as_deref(), Some("Contract Thread"));

    let key = sdk
        .key()
        .info(KeyOptions::default())
        .await
        .expect("key info should deserialize");
    assert_eq!(
        key.info
            .access
            .as_ref()
            .and_then(|access| access.access_type.as_deref()),
        Some("public")
    );

    let market = sdk
        .market()
        .bazaar(MarketOptions::default().with_id("100"))
        .await
        .expect("market bazaar should deserialize");
    assert_eq!(market.bazaar.advanced_item.len(), 1);
    assert_eq!(market.bazaar.advanced_item[0].id, Some(1));

    let property = sdk
        .property()
        .property(PropertyOptions::default().with_id("1"))
        .await
        .expect("property should deserialize");
    assert_eq!(property.property.id, Some(1));
    assert_eq!(
        property.property.owner.as_ref().and_then(|owner| owner.id),
        Some(99)
    );

    let racing = sdk
        .racing()
        .race(RacingOptions::default().with_race_id("9001"))
        .await
        .expect("race should deserialize");
    assert_eq!(racing.race.id, Some(9001));
    assert_eq!(racing.race.laps, Some(10));

    let torn = sdk
        .torn()
        .calendar(TornOptions::default())
        .await
        .expect("calendar should deserialize");
    assert!(torn.calendar.events.is_empty());
}

#[tokio::test]
async fn wrapper_additional_typed_methods_cover_production_targets() {
    let (sdk, _) = make_sdk_with_responses(vec![
        Ok(TransportResponse {
            status: 200,
            body: r#"{"profile":{"id":3637232,"name":"Basic User","status":{"state":"Okay"}}}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"discord":{"discord_id":"sample#0001","user_id":3637232}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"faction":{"id":12345,"name":"Faction A","position":"Member"}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"money":{"wallet":1000,"faction":{"money":500,"points":10}}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"bars":{"energy":{"current":120,"maximum":150},"chain":{"current":5,"max":10}}}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"cooldowns":{"booster":10,"drug":20,"medical":30}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"travel":{"destination":"Mexico","method":"Air","time_left":1200}}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"members":[{"id":1,"name":"Member A","level":50}]}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"wars":{"raids":[{"war_id":321,"factions":[{"id":12345,"name":"Faction A","score":10}]}],"ranked":{"war_id":654,"winner":12345}},"pacts":[]}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"rankedwars":[{"id":88,"winner":12345,"factions":[{"id":12345,"name":"Faction A","score":5}]}],"_metadata":{"links":{"next":null,"prev":null}}}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"itemmarket":{"cache_timestamp":1,"item":{"id":2,"name":"Item","type":"Primary","average_price":1000},"listings":[{"amount":1,"price":900}]}}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"items":[{"id":1,"name":"Item A","type":"Weapon","circulation":100}]}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"honors":[{"id":1,"name":"Honor A","rarity":"Rare","type":{"id":7,"title":"Combat"}}]}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"medals":[{"id":3,"name":"Medal A","rarity":"Common","type":{"id":9,"title":"General"}}]}"#
                .to_string(),
        }),
    ]);

    let basic = sdk
        .user()
        .basic("3637232")
        .await
        .expect("user basic should deserialize");
    assert_eq!(basic.profile.id, Some(3637232));

    let discord = sdk
        .user()
        .discord("3637232")
        .await
        .expect("user discord should deserialize");
    assert_eq!(discord.discord.discord_id.as_deref(), Some("sample#0001"));

    let faction = sdk
        .user()
        .faction("3637232")
        .await
        .expect("user faction should deserialize");
    assert_eq!(faction.faction.id, Some(12345));

    let money = sdk
        .user()
        .money(UserOptions::default())
        .await
        .expect("user money should deserialize");
    assert_eq!(money.money.wallet, Some(1000));

    let bars = sdk
        .user()
        .bars(UserOptions::default())
        .await
        .expect("user bars should deserialize");
    assert_eq!(
        bars.bars.energy.as_ref().and_then(|energy| energy.current),
        Some(120)
    );

    let cooldowns = sdk
        .user()
        .cooldowns(UserOptions::default())
        .await
        .expect("user cooldowns should deserialize");
    assert_eq!(cooldowns.cooldowns.booster, Some(10));

    let travel = sdk
        .user()
        .travel(UserOptions::default())
        .await
        .expect("user travel should deserialize");
    assert_eq!(travel.travel.destination.as_deref(), Some("Mexico"));

    let members = sdk
        .faction()
        .members(FactionOptions::default().with_id("12345"))
        .await
        .expect("faction members should deserialize");
    assert_eq!(members.members.len(), 1);
    assert_eq!(members.members[0].id, Some(1));

    let wars = sdk
        .faction()
        .wars(FactionOptions::default().with_id("12345"))
        .await
        .expect("faction wars should deserialize");
    assert_eq!(wars.wars.raids.len(), 1);
    assert_eq!(wars.wars.raids[0].war_id, Some(321));

    let rankedwars = sdk
        .faction()
        .rankedwars(FactionOptions::default().with_id("12345"))
        .await
        .expect("faction ranked wars should deserialize");
    assert_eq!(rankedwars.rankedwars.len(), 1);
    assert_eq!(rankedwars.rankedwars[0].id, Some(88));

    let itemmarket = sdk
        .market()
        .itemmarket(MarketOptions::default().with_id("2"))
        .await
        .expect("market itemmarket should deserialize");
    assert_eq!(
        itemmarket.itemmarket.item.as_ref().and_then(|item| item.id),
        Some(2)
    );

    let items = sdk
        .torn()
        .items(TornOptions::default().with_ids("1,2"))
        .await
        .expect("torn items should deserialize");
    assert_eq!(items.items.len(), 1);
    assert_eq!(items.items[0].id, Some(1));

    let honors = sdk
        .torn()
        .honors(TornOptions::default().with_ids("1,2"))
        .await
        .expect("torn honors should deserialize");
    assert_eq!(honors.honors.len(), 1);
    assert_eq!(honors.honors[0].id, Some(1));

    let medals = sdk
        .torn()
        .medals(TornOptions::default().with_ids("3,4"))
        .await
        .expect("torn medals should deserialize");
    assert_eq!(medals.medals.len(), 1);
    assert_eq!(medals.medals[0].id, Some(3));
}

#[tokio::test]
async fn wrapper_options_propagate_to_transport_request() {
    let (sdk, transport) = make_sdk_with_responses(vec![Ok(TransportResponse {
        status: 200,
        body: r#"{"profile":{"player_id":3637232}}"#.to_string(),
    })]);

    let options = UserOptions::default()
        .with_base(
            BaseOptions::default()
                .with_limit(25)
                .with_from(1_700_000_000)
                .with_to(1_700_000_300)
                .with_striptags(true)
                .with_comment("wrapper-options-test")
                .with_request_timeout(Duration::from_secs(2))
                .with_max_attempts(2),
        )
        .with_id("3637232");

    let bundle = sdk
        .user()
        .profile_raw(options)
        .await
        .expect("raw profile should succeed");

    assert_eq!(
        bundle.get("profile").expect("profile field")["player_id"],
        3637232
    );

    let requests = transport.requests();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].path, "/user");
    assert_eq!(
        requests[0].query.get("selections"),
        Some(&"profile".to_string())
    );
    assert_eq!(requests[0].query.get("id"), Some(&"3637232".to_string()));
    assert_eq!(requests[0].query.get("limit"), Some(&"25".to_string()));
    assert_eq!(
        requests[0].query.get("striptags"),
        Some(&"true".to_string())
    );
    assert_eq!(
        requests[0].query.get("comment"),
        Some(&"wrapper-options-test".to_string())
    );
    assert_eq!(requests[0].timeout, Some(Duration::from_secs(2)));
}

#[tokio::test]
async fn wrapper_path_arguments_are_forwarded_to_direct_endpoints() {
    let (sdk, transport) = make_sdk_with_responses(vec![
        Ok(TransportResponse {
            status: 200,
            body: r#"{"thread":{"id":777,"title":"Topic"}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"race":{"id":9001}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"itemmarket":{"item":{"id":2},"listings":[]}}"#.to_string(),
        }),
    ]);

    sdk.forum()
        .thread(
            ForumOptions::default()
                .with_thread_id("777")
                .with_base(BaseOptions::default().with_timestamp("1")),
        )
        .await
        .expect("forum thread should succeed");

    sdk.racing()
        .race(RacingOptions::default().with_race_id("9001"))
        .await
        .expect("racing race should succeed");

    sdk.market()
        .itemmarket(MarketOptions::default().with_id("2"))
        .await
        .expect("market itemmarket should succeed");

    let requests = transport.requests();
    assert_eq!(requests.len(), 3);
    assert_eq!(requests[0].path, "/forum");
    assert_eq!(requests[0].query.get("id"), Some(&"777".to_string()));
    assert_eq!(requests[0].query.get("timestamp"), Some(&"1".to_string()));
    assert_eq!(requests[1].path, "/racing");
    assert_eq!(requests[1].query.get("id"), Some(&"9001".to_string()));
    assert_eq!(requests[2].path, "/market");
    assert_eq!(requests[2].query.get("id"), Some(&"2".to_string()));
}

#[tokio::test]
async fn wrapper_error_mapping_uses_sdk_error_variants() {
    let (validation_sdk, _) = make_sdk_with_responses(Vec::new());
    let validation_error = validation_sdk
        .user()
        .raw_selection("not-a-real-selection", UserOptions::default())
        .await
        .expect_err("invalid selection should return validation error");
    assert!(matches!(validation_error, SdkError::Validation(_)));

    let (client_sdk, _) = make_sdk_with_responses(vec![Ok(TransportResponse {
        status: 500,
        body: r#"{"error":{"code":5000,"error":"boom"}}"#.to_string(),
    })]);
    let client_error = client_sdk
        .user()
        .profile_raw(UserOptions::default().with_id("3637232"))
        .await
        .expect_err("api error should return client variant");
    assert!(matches!(client_error, SdkError::Client(_)));

    let (decode_sdk, _) = make_sdk_with_responses(vec![Ok(TransportResponse {
        status: 200,
        body: "{}".to_string(),
    })]);
    let decode_error = decode_sdk
        .user()
        .profile("3637232")
        .await
        .expect_err("missing profile field should return decode variant");
    assert!(matches!(decode_error, SdkError::Decode(_)));
}

#[tokio::test]
async fn wrapper_validation_reports_missing_path_args_and_invalid_ranges() {
    let (sdk, _) = make_sdk_with_responses(Vec::new());

    let market_error = sdk
        .market()
        .bazaar(MarketOptions::default())
        .await
        .expect_err("bazaar without id should fail");
    match market_error {
        SdkError::Validation(message) => {
            assert!(message.contains("bazaar"));
            assert!(message.contains("id"));
        }
        other => panic!("unexpected error: {other:?}"),
    }

    let forum_error = sdk
        .forum()
        .thread(ForumOptions::default())
        .await
        .expect_err("thread without thread_id should fail");
    assert!(matches!(forum_error, SdkError::Validation(_)));

    let faction_error = sdk
        .faction()
        .search_raw(FactionOptions::default())
        .await
        .expect_err("search without name should fail");
    match faction_error {
        SdkError::Validation(message) => assert!(message.contains("name")),
        other => panic!("unexpected error: {other:?}"),
    }

    let range_error = sdk
        .user()
        .events_raw(
            UserOptions::default().with_base(BaseOptions::default().with_from(20).with_to(10)),
        )
        .await
        .expect_err("from > to should fail");
    match range_error {
        SdkError::Validation(message) => assert!(message.contains("from")),
        other => panic!("unexpected error: {other:?}"),
    }
}

#[test]
fn wrapper_selection_matrix_matches_capabilities() {
    let capabilities = production_capabilities();

    let actual_resources = capabilities
        .resources
        .keys()
        .cloned()
        .collect::<BTreeSet<_>>();
    let expected_resources = BTreeSet::from([
        "faction".to_string(),
        "forum".to_string(),
        "key".to_string(),
        "market".to_string(),
        "property".to_string(),
        "racing".to_string(),
        "torn".to_string(),
        "user".to_string(),
    ]);
    assert_eq!(actual_resources, expected_resources);

    assert_resource_selection_matrix(
        "user",
        &capabilities,
        UserApi::<MockTransport>::SUPPORTED_SELECTIONS,
    );
    assert_resource_selection_matrix(
        "faction",
        &capabilities,
        FactionApi::<MockTransport>::SUPPORTED_SELECTIONS,
    );
    assert_resource_selection_matrix(
        "forum",
        &capabilities,
        ForumApi::<MockTransport>::SUPPORTED_SELECTIONS,
    );
    assert_resource_selection_matrix(
        "key",
        &capabilities,
        KeyApi::<MockTransport>::SUPPORTED_SELECTIONS,
    );
    assert_resource_selection_matrix(
        "market",
        &capabilities,
        MarketApi::<MockTransport>::SUPPORTED_SELECTIONS,
    );
    assert_resource_selection_matrix(
        "property",
        &capabilities,
        PropertyApi::<MockTransport>::SUPPORTED_SELECTIONS,
    );
    assert_resource_selection_matrix(
        "racing",
        &capabilities,
        RacingApi::<MockTransport>::SUPPORTED_SELECTIONS,
    );
    assert_resource_selection_matrix(
        "torn",
        &capabilities,
        TornApi::<MockTransport>::SUPPORTED_SELECTIONS,
    );
}

fn assert_resource_selection_matrix(
    resource: &str,
    capabilities: &CapabilitiesDocument,
    wrapper_supported: &[&str],
) {
    let capability_resource = capabilities
        .resources
        .get(resource)
        .expect("resource should exist in capabilities");

    let capability_set = capability_resource
        .selections
        .iter()
        .map(|selection| selection.name.clone())
        .collect::<BTreeSet<_>>();
    let wrapper_set = wrapper_supported
        .iter()
        .map(|selection| selection.to_string())
        .collect::<BTreeSet<_>>();

    assert_eq!(
        capability_set, wrapper_set,
        "selection set mismatch for resource {resource}"
    );
}
