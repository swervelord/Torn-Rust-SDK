mod support;

use std::path::PathBuf;
use std::time::Duration;

use support::MockTransport;
use torn_sdk_planner::{
    BaseOptions, ExecutorConfig, RacingOptions, RateLimitConfig, RequestExecutor, RequestPlanner,
    TornClient, TornSdk, TransportError, TransportResponse,
};

fn capabilities_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("spec")
        .join("capabilities.json")
}

fn test_config() -> ExecutorConfig {
    ExecutorConfig {
        base_url_v2: "https://api.torn.com/v2".to_string(),
        base_url_v1: "https://api.torn.com".to_string(),
        timeout: Duration::from_secs(5),
        user_agent: "racing-contract-test".to_string(),
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
    let planner =
        RequestPlanner::from_capabilities_file(capabilities_path()).expect("capabilities load");
    let transport = MockTransport::with_responses(responses);
    let executor = RequestExecutor::new(transport.clone(), vec!["k1".to_string()], test_config())
        .expect("executor should initialize");
    (TornSdk::new(TornClient::new(planner, executor)), transport)
}

#[tokio::test]
async fn racing_typed_methods_deserialize_stable_surfaces() {
    let (sdk, _) = make_sdk_with_responses(vec![
        Ok(TransportResponse {
            status: 200,
            body: r#"{"tracks":[{"id":6,"title":"Uptown","description":"Fast corners"}]}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"cars":[{"car_item_id":77,"car_item_name":"Tabata RM2","top_speed":25,"acceleration":20,"braking":15,"dirt":15,"handling":20,"safety":20,"tarmac":20,"class":"D"}]}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"carupgrades":[{"id":1,"class_required":"E","name":"Strip Out","description":"Reduce weight","category":"Weight Reduction","subcategory":"Strip out","effects":{"top_speed":1,"acceleration":1,"braking":1,"handling":1,"safety":0,"dirt":0,"tarmac":0},"cost":{"points":1,"cash":0}}]}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"races":[{"id":18901349,"title":"Quick Speedway","track_id":21,"creator_id":983221,"status":"open","laps":1,"participants":{"minimum":2,"maximum":2,"current":1},"schedule":{"join_from":1777419313,"join_until":1777419373,"start":1777419373,"end":null},"requirements":{"driver_class":null,"car_class":null,"car_item_id":null,"requires_stock_car":false,"requires_password":false,"join_fee":0},"is_official":false}],"_metadata":{"links":{"prev":"https://api.torn.com/v2/racing/races?limit=1&to=1777419373","next":null}}}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"records":[{"driver_id":1,"driver_name":"Racer","car_item_id":77,"lap_time":123.45,"car_item_name":"Tabata RM2"}]}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"race":{"id":18900843,"title":"Off race","track_id":23,"creator_id":4192829,"status":"finished","laps":8,"participants":{"minimum":3,"maximum":6,"current":6},"schedule":{"join_from":1777415699,"join_until":1777419299,"start":1777419367,"end":1777420944},"requirements":{"driver_class":"E","car_class":null,"car_item_id":null,"requires_stock_car":false,"requires_password":false,"join_fee":0},"is_official":true,"results":[{"driver_id":2,"position":1,"car_id":900,"car_item_id":78,"car_item_name":"Edomondo NSX","car_class":"A","has_crashed":false,"best_lap_time":54.32,"race_time":450.12,"time_ended":1777420944}]}}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"selections":["cars","carupgrades","lookup","race","races","records","timestamp","tracks"]}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"timestamp":1777419373}"#.to_string(),
        }),
    ]);

    let tracks = sdk
        .racing()
        .tracks(RacingOptions::default())
        .await
        .expect("tracks should deserialize");
    assert_eq!(tracks.tracks[0].id, Some(6));
    assert_eq!(tracks.tracks[0].title.as_deref(), Some("Uptown"));

    let cars = sdk
        .racing()
        .cars(RacingOptions::default())
        .await
        .expect("cars should deserialize");
    assert_eq!(cars.cars[0].car_item_id, Some(77));
    assert_eq!(cars.cars[0].car_class.as_deref(), Some("D"));

    let carupgrades = sdk
        .racing()
        .carupgrades(RacingOptions::default())
        .await
        .expect("carupgrades should deserialize");
    assert_eq!(carupgrades.carupgrades[0].id, Some(1));
    assert_eq!(
        carupgrades.carupgrades[0]
            .effects
            .as_ref()
            .and_then(|effects| effects.top_speed),
        Some(1)
    );

    let races = sdk
        .racing()
        .races(RacingOptions::default().with_base(BaseOptions::default().with_limit(1)))
        .await
        .expect("races should deserialize");
    assert_eq!(races.races[0].id, Some(18901349));
    assert_eq!(
        races.races[0].schedule.as_ref().and_then(|s| s.join_from),
        Some(1777419313)
    );
    assert!(
        races
            ._metadata
            .as_ref()
            .and_then(|metadata| metadata.links.as_ref())
            .and_then(|links| links.prev.as_deref())
            .is_some()
    );

    let records = sdk
        .racing()
        .records(
            RacingOptions::default()
                .with_track_id("6")
                .with_base(BaseOptions::default().with_cat("D")),
        )
        .await
        .expect("records should deserialize");
    assert_eq!(records.records[0].driver_id, Some(1));
    assert_eq!(records.records[0].lap_time, Some(123.45));

    let race = sdk
        .racing()
        .race(RacingOptions::default().with_race_id("18900843"))
        .await
        .expect("race should deserialize");
    assert_eq!(race.race.id, Some(18900843));
    assert_eq!(race.race.results[0].car_class.as_deref(), Some("A"));
    assert_eq!(race.race.results[0].best_lap_time, Some(54.32));

    let lookup = sdk
        .racing()
        .lookup(RacingOptions::default())
        .await
        .expect("lookup should deserialize");
    assert!(
        lookup
            .selections
            .iter()
            .any(|selection| selection == "races")
    );

    let timestamp = sdk
        .racing()
        .timestamp(RacingOptions::default())
        .await
        .expect("timestamp should deserialize");
    assert_eq!(timestamp.timestamp, Some(1777419373));
}

#[tokio::test]
async fn racing_records_validation_and_path_forwarding_match_direct_endpoints() {
    let (sdk, transport) = make_sdk_with_responses(vec![
        Ok(TransportResponse {
            status: 200,
            body: r#"{"records":[{"driver_id":1,"driver_name":"Racer","car_item_id":77,"lap_time":123.45,"car_item_name":"Tabata RM2"}]}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"race":{"id":9001,"title":"Contract Race","track_id":6,"creator_id":12,"status":"finished","laps":10,"participants":{"minimum":2,"maximum":2,"current":2},"schedule":{"join_from":100,"join_until":200,"start":300,"end":400},"requirements":{"driver_class":"D","car_class":"D","car_item_id":77,"requires_stock_car":false,"requires_password":false,"join_fee":0},"is_official":false,"results":[]}}"#
                .to_string(),
        }),
    ]);

    let missing_cat_error = sdk
        .racing()
        .records(RacingOptions::default().with_track_id("6"))
        .await
        .expect_err("records without cat should fail");
    assert!(format!("{missing_cat_error}").contains("cat"));

    let missing_track_error = sdk
        .racing()
        .records(RacingOptions::default().with_base(BaseOptions::default().with_cat("D")))
        .await
        .expect_err("records without trackId should fail");
    assert!(format!("{missing_track_error}").contains("trackId"));

    sdk.racing()
        .records(
            RacingOptions::default()
                .with_track_id("6")
                .with_base(BaseOptions::default().with_cat("D").with_timestamp("1")),
        )
        .await
        .expect("records should succeed");

    sdk.racing()
        .race(RacingOptions::default().with_race_id("9001"))
        .await
        .expect("race should succeed");

    let requests = transport.requests();
    assert_eq!(requests.len(), 2);
    assert_eq!(requests[0].path, "/racing");
    assert_eq!(
        requests[0].query.get("selections"),
        Some(&"records".to_string())
    );
    assert_eq!(requests[0].query.get("id"), Some(&"6".to_string()));
    assert_eq!(requests[0].query.get("cat"), Some(&"D".to_string()));
    assert_eq!(requests[0].query.get("timestamp"), Some(&"1".to_string()));
    assert_eq!(requests[1].path, "/racing");
    assert_eq!(
        requests[1].query.get("selections"),
        Some(&"race".to_string())
    );
    assert_eq!(requests[1].query.get("id"), Some(&"9001".to_string()));
}
