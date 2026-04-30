mod support;

use std::path::PathBuf;
use std::time::Duration;

use support::MockTransport;
use torn_sdk_planner::{
    BaseOptions, CompanyOptions, ExecutorConfig, RateLimitConfig, RequestExecutor, RequestPlanner,
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
        user_agent: "company-contract-test".to_string(),
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
async fn company_typed_helpers_deserialize_stable_company_shapes() {
    let (sdk, _) = make_sdk_with_responses(vec![
        Ok(TransportResponse {
            status: 200,
            body: r#"{"applications":[{"id":44,"message":"Ready to work","valid_until":1710000600,"status":"pending","user":{"id":2077404,"name":"RezVX","level":55}}]}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"company":null,"company_timestamp":1710000650,"company_delay":60}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"selections":["applications","companies","employees","lookup","news","profile","search","stock","timestamp"]}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"timestamp":1710000700}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"company":{"ID":76323,"name":"Novel Ideas","company_type":10,"rating":7,"age":923,"daily_income":3210000,"weekly_income":21987000,"employees":{"2077404":{"name":"RezVX","position":"Florist"}},"status":{"state":"Okay","color":"green","description":"Open","details":"","until":0}}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"employees":[{"id":2077404,"name":"RezVX","position":{"id":1,"name":"Florist"},"days_in_company":12,"joined_at":1700000000,"wage":0,"stats":{"manual_labor":2802,"intelligence":3143,"endurance":6473},"effectiveness":{"working_stats":126,"settled_in":10,"director_education":12,"total":148},"value":12345,"status":{"state":"Okay","color":"green","description":"Working","details":"","until":0},"last_action":{"status":"Online","timestamp":1710000000,"relative":"1 minute ago"}}]}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"news":{"abc123":{"news":"Director hired a new employee","timestamp":1710000750}}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"stock":[{"id":1,"name":"Erotic DVD","cost":1400,"rrp":2000,"price":2200,"in_stock":120,"on_order":12,"sold_amount":43,"sold_worth":94600},{"id":2,"name":"Lube","cost":400,"rrp":600,"price":650,"in_stock":55,"on_order":5,"sold_amount":19,"sold_worth":12350}]}"#.to_string(),
        }),
    ]);

    let applications = sdk
        .company()
        .applications(CompanyOptions::default())
        .await
        .expect("company applications should deserialize");
    assert_eq!(applications.applications.len(), 1);
    assert_eq!(applications.applications[0].id, Some(44));
    assert_eq!(
        applications.applications[0]
            .user
            .as_ref()
            .and_then(|user| user.name.as_deref()),
        Some("RezVX")
    );

    let companies = sdk
        .company()
        .companies(CompanyOptions::default().with_id("76323"))
        .await
        .expect("company companies should deserialize");
    assert!(companies.company.is_none());
    assert_eq!(companies.company_delay, Some(60));

    let lookup = sdk
        .company()
        .lookup(CompanyOptions::default())
        .await
        .expect("company lookup should deserialize");
    assert!(
        lookup
            .selections
            .iter()
            .any(|selection| selection == "profile")
    );

    let timestamp = sdk
        .company()
        .timestamp(CompanyOptions::default())
        .await
        .expect("company timestamp should deserialize");
    assert_eq!(timestamp.timestamp, Some(1_710_000_700));

    let profile = sdk
        .company()
        .profile(CompanyOptions::default().with_id("76323"))
        .await
        .expect("company profile should deserialize");
    assert_eq!(profile.company.id, Some(76_323));
    assert_eq!(profile.company.name.as_deref(), Some("Novel Ideas"));
    assert_eq!(profile.company.company_type, Some(10));
    assert_eq!(profile.company.daily_income, Some(3_210_000));
    assert_eq!(
        profile
            .company
            .employees
            .as_ref()
            .and_then(serde_json::Value::as_object)
            .map(|employees| employees.len()),
        Some(1)
    );
    assert_eq!(
        profile
            .company
            .status
            .as_ref()
            .and_then(|status| status.description.as_deref()),
        Some("Open")
    );

    let employees = sdk
        .company()
        .employees(CompanyOptions::default().with_id("76323"))
        .await
        .expect("company employees should deserialize");
    let employee = employees
        .employees
        .iter()
        .find(|employee| employee.id == Some(2_077_404))
        .expect("employee should exist");
    assert_eq!(employee.position.as_deref(), Some("Florist"));
    assert_eq!(
        employee.stats.as_ref().and_then(|stats| stats.manual_labor),
        Some(2_802)
    );
    assert_eq!(
        employee
            .effectiveness
            .as_ref()
            .and_then(|effectiveness| effectiveness.total),
        Some(148)
    );

    let news = sdk
        .company()
        .news(CompanyOptions::default().with_id("76323"))
        .await
        .expect("company news should deserialize");
    let news_entry = news.news.get("abc123").expect("news item should exist");
    assert_eq!(
        news_entry.news.as_deref(),
        Some("Director hired a new employee")
    );
    assert_eq!(news_entry.timestamp, Some(1_710_000_750));

    let stock = sdk
        .company()
        .stock(CompanyOptions::default().with_id("76323"))
        .await
        .expect("company stock should deserialize");
    let dvd = stock
        .stock
        .iter()
        .find(|item| item.name.as_deref() == Some("Erotic DVD"))
        .expect("stock item should exist");
    assert_eq!(dvd.cost, Some(1_400));
    assert_eq!(dvd.price, Some(2_200));
    assert_eq!(dvd.in_stock, Some(120));
    assert_eq!(dvd.sold_amount, Some(43));
}

#[tokio::test]
async fn company_wrapper_supports_owner_and_id_scoped_routes() {
    let (sdk, transport) = make_sdk_with_responses(vec![
        Ok(TransportResponse {
            status: 200,
            body: r#"{"selections":["lookup"]}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"company":{"ID":76323,"name":"Novel Ideas"}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"company_employees":null}"#.to_string(),
        }),
    ]);

    sdk.company()
        .lookup(CompanyOptions::default())
        .await
        .expect("lookup should work without a company id");

    sdk.company()
        .profile(CompanyOptions::default().with_id("76323"))
        .await
        .expect("profile should work with an explicit company id");

    let employees = sdk
        .company()
        .employees(
            CompanyOptions::default().with_base(BaseOptions::default().with_from(10).with_to(20)),
        )
        .await
        .expect("employees should allow owner-scoped requests without an id");
    assert!(employees.company_employees.is_none());

    let requests = transport.requests();
    assert_eq!(requests.len(), 3);

    assert_eq!(requests[0].path, "/company/lookup");
    assert!(!requests[0].query.contains_key("selections"));
    assert!(!requests[0].query.contains_key("id"));

    assert_eq!(requests[1].path, "/company/76323/profile");
    assert!(!requests[1].query.contains_key("selections"));
    assert!(!requests[1].query.contains_key("id"));

    assert_eq!(requests[2].path, "/company/employees");
    assert!(!requests[2].query.contains_key("selections"));
    assert!(!requests[2].query.contains_key("from"));
    assert!(!requests[2].query.contains_key("to"));
}

#[tokio::test]
async fn company_wrapper_preserves_raw_escape_hatches_and_validation() {
    let (sdk, _) = make_sdk_with_responses(vec![Ok(TransportResponse {
        status: 200,
        body: r#"{"news":[{"timestamp":1710000800,"news":"Director hired a new employee"}]}"#
            .to_string(),
    })]);

    let raw = sdk
        .company()
        .news_raw(CompanyOptions::default().with_base(BaseOptions::default().with_limit(1)))
        .await
        .expect("raw company news should remain available");
    assert!(raw.get("news").is_some());

    let invalid_selection = sdk
        .company()
        .raw_selection("not-real", CompanyOptions::default())
        .await
        .expect_err("unknown company selection should fail validation");
    assert!(matches!(invalid_selection, SdkError::Validation(_)));

    let invalid_range = sdk
        .company()
        .raw_selection(
            "news",
            CompanyOptions::default().with_base(BaseOptions::default().with_from(20).with_to(10)),
        )
        .await
        .expect_err("invalid ranges should fail before request execution");
    match invalid_range {
        SdkError::Validation(message) => {
            assert!(message.contains("company"));
            assert!(message.contains("'from' <= 'to'"));
        }
        other => panic!("unexpected error: {other:?}"),
    }
}
