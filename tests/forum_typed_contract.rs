#[allow(dead_code)]
mod support;

use std::path::PathBuf;
use std::time::Duration;

use support::MockTransport;
use torn_sdk_planner::{
    BaseOptions, ExecutorConfig, ForumOptions, RateLimitConfig, RequestExecutor, RequestPlanner,
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
        user_agent: "forum-contract-test".to_string(),
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
async fn forum_typed_helpers_deserialize_contract_payloads() {
    let (sdk, _) = make_sdk_with_responses(vec![
        Ok(TransportResponse {
            status: 200,
            body: r#"{"categories":[{"id":63,"title":"API Development","acronym":"API","threads":1026}]}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"selections":["categories","lookup","posts","thread","threads","timestamp"]}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"threads":[{"id":16559714,"title":"How do I become the best degenerate gambler in this game","forum_id":15,"posts":22,"rating":-12,"views":297,"author":{"id":4265422,"username":"JohnJohn_Site","karma":0},"last_poster":{"id":4265422,"username":"JohnJohn_Site","karma":0},"first_post_time":1777369314,"last_post_time":1777419396,"has_poll":false,"is_locked":false,"is_sticky":false}],"_metadata":{"links":{"prev":"https://api.torn.com/v2/forum/threads?&limit=1&sort=desc&to=1777419396","next":null}}}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"thread":{"id":16559714,"title":"How do I become the best degenerate gambler in this game","forum_id":15,"posts":22,"rating":-12,"views":297,"author":{"id":4265422,"username":"JohnJohn_Site","karma":0},"last_poster":{"id":4265422,"username":"JohnJohn_Site","karma":0},"first_post_time":1777369314,"last_post_time":1777419396,"has_poll":false,"is_locked":false,"is_sticky":false,"content":"Plain text body","content_raw":"<p>Plain text body</p>","poll":null}}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"posts":[{"id":27283947,"thread_id":16559714,"author":{"id":4265422,"username":"JohnJohn_Site","karma":0},"is_legacy":false,"is_topic":true,"is_edited":false,"is_pinned":false,"created_time":1777369314,"edited_by":null,"has_quote":false,"quoted_post_id":null,"content":"Plain text body","likes":1,"dislikes":13}],"_metadata":{"links":{"prev":null,"next":"https://api.torn.com/v2/forum/16559714/posts?&limit=1&stripTags=true&offset=1"}}}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"timestamp":1710000400}"#.to_string(),
        }),
    ]);

    let categories = sdk
        .forum()
        .categories(ForumOptions::default())
        .await
        .expect("forum categories should deserialize");
    assert_eq!(categories.categories.len(), 1);
    assert_eq!(categories.categories[0].id, Some(63));
    assert_eq!(categories.categories[0].acronym.as_deref(), Some("API"));

    let lookup = sdk
        .forum()
        .lookup(ForumOptions::default())
        .await
        .expect("forum lookup should deserialize");
    assert!(
        lookup
            .selections
            .iter()
            .any(|selection| selection == "thread")
    );

    let threads = sdk
        .forum()
        .threads(ForumOptions::default().with_base(BaseOptions::default().with_limit(1)))
        .await
        .expect("forum threads should deserialize");
    assert_eq!(threads.threads.len(), 1);
    assert_eq!(threads.threads[0].id, Some(16559714));
    assert_eq!(threads.threads[0].rating, Some(-12));
    assert_eq!(
        threads
            ._metadata
            .as_ref()
            .and_then(|metadata| metadata.links.as_ref())
            .and_then(|links| links.prev.as_deref()),
        Some("https://api.torn.com/v2/forum/threads?&limit=1&sort=desc&to=1777419396")
    );

    let thread = sdk
        .forum()
        .thread(ForumOptions::default().with_thread_id("16559714"))
        .await
        .expect("forum thread should deserialize");
    assert_eq!(thread.thread.id, Some(16559714));
    assert_eq!(
        thread.thread.content_raw.as_deref(),
        Some("<p>Plain text body</p>")
    );
    assert!(thread.thread.poll.is_none());

    let posts = sdk
        .forum()
        .posts(ForumOptions::default().with_thread_id("16559714"))
        .await
        .expect("forum posts should deserialize");
    assert_eq!(posts.posts.len(), 1);
    assert_eq!(posts.posts[0].thread_id, Some(16559714));
    assert_eq!(posts.posts[0].is_topic, Some(true));
    assert_eq!(posts.posts[0].likes, Some(1));

    let timestamp = sdk
        .forum()
        .timestamp(ForumOptions::default())
        .await
        .expect("forum timestamp should deserialize");
    assert_eq!(timestamp.timestamp, Some(1710000400));
}

#[tokio::test]
async fn forum_threads_supports_global_and_category_scoped_routes() {
    let (sdk, transport) = make_sdk_with_responses(vec![
        Ok(TransportResponse {
            status: 200,
            body: r#"{"threads":[],"_metadata":{"links":{"prev":null,"next":null}}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"threads":[],"_metadata":{"links":{"prev":null,"next":null}}}"#.to_string(),
        }),
    ]);

    sdk.forum()
        .threads(ForumOptions::default())
        .await
        .expect("global forum threads should work without category ids");

    sdk.forum()
        .threads(ForumOptions::default().with_category_ids("63,67"))
        .await
        .expect("category scoped forum threads should deserialize");

    let requests = transport.requests();
    assert_eq!(requests.len(), 2);
    assert_eq!(
        requests[0].query.get("selections"),
        Some(&"threads".to_string())
    );
    assert!(!requests[0].query.contains_key("id"));

    let scoped_request = &requests[1];
    let scoped_via_path =
        scoped_request.path.contains("63,67") && scoped_request.path.contains("threads");
    let scoped_via_query = scoped_request.query.get("id") == Some(&"63,67".to_string())
        && scoped_request.query.get("selections") == Some(&"threads".to_string());
    assert!(
        scoped_via_path || scoped_via_query,
        "expected category-scoped threads request, got path={} query={:?}",
        scoped_request.path,
        scoped_request.query
    );
}

#[tokio::test]
async fn forum_posts_still_require_thread_id() {
    let (sdk, _) = make_sdk_with_responses(Vec::new());

    let error = sdk
        .forum()
        .posts(ForumOptions::default())
        .await
        .expect_err("forum posts without thread id should fail validation");

    match error {
        SdkError::Validation(message) => {
            assert!(message.contains("forum"));
            assert!(message.contains("posts"));
            assert!(message.contains("threadId"));
        }
        other => panic!("expected validation error, got {other:?}"),
    }
}
