mod support;

use std::collections::BTreeSet;
use std::path::PathBuf;
use std::time::Duration;

use serde::Deserialize;
use support::MockTransport;
use torn_sdk_planner::models::manual::key::KeyLogTargetId;
use torn_sdk_planner::models::manual::user::{TradeItem, UserEmployment, UserTradeTimestamp};
use torn_sdk_planner::{
    BaseOptions, CapabilitiesDocument, CompanyApi, DataRequestOptions, ExecutionOptions,
    ExecutorConfig, FactionApi, FactionOptions, ForumApi, ForumOptions, KeyApi, KeyOptions,
    MarketApi, MarketOptions, PropertyApi, PropertyOptions, RacingApi, RacingOptions,
    RateLimitConfig, RequestExecutor, RequestPlanner, SdkError, TornApi, TornClient, TornOptions,
    TornSdk, TransportError, TransportResponse, UserApi, UserOptions,
};

fn fixture_capabilities() -> CapabilitiesDocument {
    let json = r#"
{
  "spec": { "version": "5.7.1" },
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

fn request_targets_resource(request: &torn_sdk_planner::TransportRequest, resource: &str) -> bool {
    request.path.starts_with(&format!("/{resource}"))
}

fn request_targets_selection(
    request: &torn_sdk_planner::TransportRequest,
    resource: &str,
    selection: &str,
) -> bool {
    request_targets_resource(request, resource)
        && (request.query.get("selections").map(String::as_str) == Some(selection)
            || request.path.split('/').any(|segment| segment == selection))
}

fn request_carries_id(request: &torn_sdk_planner::TransportRequest, id: &str) -> bool {
    request.query.get("id").map(String::as_str) == Some(id)
        || request.path.split('/').any(|segment| segment == id)
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
    assert!(request_targets_selection(&requests[0], "user", "profile"));
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
            body: r#"{"casino":{"tokens":25,"streak":-7}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"inventory":{"items":[{"id":788,"amount":2,"equipped":false,"name":"Certificate of Awesome","uid":null,"faction_owned":false}],"timestamp":1710000000},"_metadata":{"links":{"next":null,"prev":null},"total":1}}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"trades":[{"id":12036957,"timestamp":false,"description":"g","user":{"id":3637232,"name":"Swervelord"},"trader":{"id":3054822,"name":"Miya"}}],"_metadata":{"links":{"next":null,"prev":null}}}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"trade":{"id":42,"timestamp":1710000100,"description":"sample","type":"private","user":{"id":3637232,"name":"Swervelord"},"trader":{"id":3054822,"name":"Miya"},"items":[{"user_id":3054822,"type":"Money","details":{"amount":500000}},{"user_id":3637232,"type":"Item","details":{"id":27,"uid":null,"amount":1}},{"user_id":3637232,"type":"Mystery","details":{"foo":"bar"}}]}}"#
                .to_string(),
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

    let casino = sdk
        .user()
        .casino()
        .await
        .expect("user casino should deserialize");
    assert_eq!(casino.casino.tokens, Some(25));
    assert_eq!(casino.casino.streak, Some(-7));

    let inventory = sdk
        .user()
        .inventory(
            UserOptions::default().with_base(
                BaseOptions::default()
                    .with_cat("Other")
                    .with_limit(1)
                    .with_offset(0),
            ),
        )
        .await
        .expect("user inventory should deserialize");
    assert_eq!(inventory.inventory.items.len(), 1);
    assert_eq!(inventory.inventory.items[0].id, Some(788));
    assert_eq!(
        inventory
            ._metadata
            .as_ref()
            .and_then(|metadata| metadata.total),
        Some(1)
    );

    let trades = sdk
        .user()
        .trades(UserOptions::default().with_base(BaseOptions::default().with_cat("ongoing")))
        .await
        .expect("user trades should deserialize");
    assert_eq!(trades.trades.len(), 1);
    assert!(matches!(
        trades.trades[0].timestamp,
        UserTradeTimestamp::Unavailable(false)
    ));

    let trade = sdk
        .user()
        .trade(UserOptions::default().with_trade_id("42"))
        .await
        .expect("user trade should deserialize");
    assert_eq!(trade.trade.id, Some(42));
    assert_eq!(trade.trade.trade_type.as_deref(), Some("private"));
    assert_eq!(trade.trade.items.len(), 3);
    match &trade.trade.items[0] {
        TradeItem::Money(item) => assert_eq!(item.details.amount, Some(500000)),
        other => panic!("unexpected first trade item: {other:?}"),
    }
    match &trade.trade.items[1] {
        TradeItem::Inventory(item) => {
            assert_eq!(item.details.id, Some(27));
            assert_eq!(item.details.uid, None);
        }
        other => panic!("unexpected second trade item: {other:?}"),
    }
    match &trade.trade.items[2] {
        TradeItem::Unknown(item) => {
            assert_eq!(item.item_type.as_deref(), Some("Mystery"));
            assert_eq!(
                item.details.as_ref().and_then(|value| value.get("foo")),
                Some(&"bar".into())
            );
        }
        other => panic!("unexpected third trade item: {other:?}"),
    }

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
async fn wrapper_expanded_typed_methods_cover_new_resources() {
    let (sdk, _) = make_sdk_with_responses(vec![
        Ok(TransportResponse {
            status: 200,
            body: r#"{"log":[{"timestamp":1710000000,"type":"user","selections":"profile,bars","id":"3637232","comment":"typed smoke"}]}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"categories":[{"id":63,"title":"Announcements","acronym":"ANN","threads":42}]}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"threads":[{"id":16559714,"title":"Thread A","forum_id":63,"posts":10,"rating":-2,"author":{"id":1,"username":"Poster"}}],"_metadata":{"links":{"next":null,"prev":null}}}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"posts":[{"id":987,"thread_id":16559714,"created_time":1710001000,"content":"Hello","author":{"id":1,"username":"Poster"}}],"_metadata":{"links":{"next":null,"prev":null}}}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"selections":["lookup","property","timestamp"]}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"timestamp":1710000200}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"selections":["lookup","tracks","records","races","cars"]}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"tracks":[{"id":6,"title":"Uptown","description":"Street circuit"}]}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"records":[{"driver_id":1,"driver_name":"Racer","car_item_id":10,"car_item_name":"Cosworth","lap_time":62.41}]}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"timestamp":1710000300}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"selections":["lookup","pointsmarket","auctionhouse"]}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"pointsmarket":{"20085593":{"cost":33395,"quantity":500,"total_cost":16697500}}}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"auctionhouse":[{"id":514309,"price":64000001,"item":{"id":219,"uid":10772039092,"name":"Enfield SA-80","type":"Weapon","sub_type":"Rifle"}}],"_metadata":{"links":{"next":null,"prev":null}}}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"logcategories":[{"id":1,"title":"Account creation"}]}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"stocks":[{"id":1,"name":"Torn & Shanghai Banking","acronym":"TSB","market":{"price":1179.43,"cap":100,"shares":10,"investors":3}}]}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"timestamp":1710000400}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"notifications":{"messages":2,"events":5,"awards":1,"competition":0}}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"icons":[{"id":7,"title":"Flying","description":null,"until":1710000500}]}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"job":{"company_id":321,"company_name":"Widgets Inc.","position":"Clerk","job":"Employee","salary":12345}}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"stocks":[{"id":1,"shares":100,"transactions":[{"shares":10,"bought":1710000600,"price":123.45}],"bonus":{"total":1,"available":false}}]}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"education":{"complete":[1,2],"current":{"id":7,"name":"History 101","duration":14}}}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"competition":{"name":"Elimination","team":"red","score":50,"lives":3}}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"racingrecords":[{"track":{"id":6,"name":"Uptown"},"records":[{"car_item_id":10,"car_item_name":"Cosworth","lap_time":2656}]}]}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"honors":[{"id":1,"timestamp":1710000700}]}"#.to_string(),
        }),
    ]);

    let key_log = sdk
        .key()
        .log(KeyOptions::default().with_base(BaseOptions::default().with_limit(1)))
        .await
        .expect("key log should deserialize");
    assert_eq!(key_log.log.len(), 1);
    assert!(matches!(
        key_log.log[0].id,
        Some(KeyLogTargetId::Text(ref value)) if value == "3637232"
    ));

    let forum_categories = sdk
        .forum()
        .categories(ForumOptions::default())
        .await
        .expect("forum categories should deserialize");
    assert_eq!(forum_categories.categories[0].id, Some(63));

    let forum_threads = sdk
        .forum()
        .threads(ForumOptions::default().with_base(BaseOptions::default().with_limit(1)))
        .await
        .expect("forum threads should deserialize");
    assert_eq!(forum_threads.threads[0].rating, Some(-2));
    assert!(forum_threads._metadata.is_some());

    let forum_posts = sdk
        .forum()
        .posts(ForumOptions::default().with_thread_id("16559714"))
        .await
        .expect("forum posts should deserialize");
    assert_eq!(forum_posts.posts[0].thread_id, Some(16_559_714));

    let property_lookup = sdk
        .property()
        .lookup(PropertyOptions::default())
        .await
        .expect("property lookup should deserialize");
    assert!(
        property_lookup
            .selections
            .iter()
            .any(|item| item == "timestamp")
    );

    let property_timestamp = sdk
        .property()
        .timestamp(PropertyOptions::default())
        .await
        .expect("property timestamp should deserialize");
    assert_eq!(property_timestamp.timestamp, Some(1_710_000_200));

    let racing_lookup = sdk
        .racing()
        .lookup(RacingOptions::default())
        .await
        .expect("racing lookup should deserialize");
    assert!(
        racing_lookup
            .selections
            .iter()
            .any(|item| item == "records")
    );

    let racing_tracks = sdk
        .racing()
        .tracks(RacingOptions::default())
        .await
        .expect("racing tracks should deserialize");
    assert_eq!(racing_tracks.tracks[0].title.as_deref(), Some("Uptown"));

    let racing_records = sdk
        .racing()
        .records(
            RacingOptions::default()
                .with_track_id("6")
                .with_base(BaseOptions::default().with_cat("D")),
        )
        .await
        .expect("racing records should deserialize");
    assert_eq!(racing_records.records[0].lap_time, Some(62.41));

    let market_timestamp = sdk
        .market()
        .timestamp(MarketOptions::default())
        .await
        .expect("market timestamp should deserialize");
    assert_eq!(market_timestamp.timestamp, Some(1_710_000_300));

    let market_lookup = sdk
        .market()
        .lookup(MarketOptions::default())
        .await
        .expect("market lookup should deserialize");
    assert!(
        market_lookup
            .selections
            .iter()
            .any(|item| item == "auctionhouse")
    );

    let pointsmarket = sdk
        .market()
        .pointsmarket(MarketOptions::default())
        .await
        .expect("market pointsmarket should deserialize");
    assert_eq!(
        pointsmarket
            .pointsmarket
            .get("20085593")
            .and_then(|entry| entry.quantity),
        Some(500)
    );

    let auctionhouse = sdk
        .market()
        .auctionhouse(MarketOptions::default().with_base(BaseOptions::default().with_limit(1)))
        .await
        .expect("market auctionhouse should deserialize");
    assert_eq!(auctionhouse.auctionhouse[0].id, Some(514_309));

    let logcategories = sdk
        .torn()
        .logcategories(TornOptions::default())
        .await
        .expect("torn logcategories should deserialize");
    assert_eq!(
        logcategories.logcategories[0].title.as_deref(),
        Some("Account creation")
    );

    let stocks = sdk
        .torn()
        .stocks(TornOptions::default())
        .await
        .expect("torn stocks should deserialize");
    assert_eq!(stocks.stocks[0].acronym.as_deref(), Some("TSB"));

    let timestamp = sdk
        .user()
        .timestamp(UserOptions::default())
        .await
        .expect("user timestamp should deserialize");
    assert_eq!(timestamp.timestamp, Some(1_710_000_400));

    let notifications = sdk
        .user()
        .notifications(UserOptions::default())
        .await
        .expect("user notifications should deserialize");
    assert_eq!(notifications.notifications.events, Some(5));

    let icons = sdk
        .user()
        .icons(UserOptions::default())
        .await
        .expect("user icons should deserialize");
    assert_eq!(icons.icons[0].description, None);

    let job = sdk
        .user()
        .job(UserOptions::default())
        .await
        .expect("user job should deserialize");
    match job.job.as_ref() {
        Some(UserEmployment::Unknown(entry)) => {
            assert_eq!(entry.extra.get("company_id"), Some(&321.into()));
            assert_eq!(
                entry.extra.get("company_name"),
                Some(&"Widgets Inc.".into())
            );
        }
        Some(UserEmployment::Company(entry)) => {
            assert_eq!(entry.id, Some(321));
            assert_eq!(entry.name.as_deref(), Some("Widgets Inc."));
        }
        Some(other) => panic!("unexpected employment variant: {other:?}"),
        None => panic!("expected user job payload"),
    }

    let user_stocks = sdk
        .user()
        .stocks(UserOptions::default())
        .await
        .expect("user stocks should deserialize");
    assert_eq!(user_stocks.stocks[0].shares, Some(100));
    assert_eq!(user_stocks.stocks[0].transactions[0].price, Some(123.45));

    let education = sdk
        .user()
        .education(UserOptions::default())
        .await
        .expect("user education should deserialize");
    assert_eq!(education.education.complete, vec![1, 2]);

    let competition = sdk
        .user()
        .competition(UserOptions::default())
        .await
        .expect("user competition should deserialize");
    assert!(competition.competition.is_some());

    let racingrecords = sdk
        .user()
        .racingrecords(UserOptions::default())
        .await
        .expect("user racingrecords should deserialize");
    assert_eq!(
        racingrecords.racingrecords[0]
            .track
            .as_ref()
            .and_then(|track| track.id),
        Some(6)
    );

    let honors = sdk
        .user()
        .honors(UserOptions::default())
        .await
        .expect("user honors should deserialize");
    assert_eq!(honors.honors[0].timestamp, Some(1_710_000_700));
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
    assert!(request_targets_selection(&requests[0], "user", "profile"));
    assert!(request_carries_id(&requests[0], "3637232"));
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
    assert!(request_targets_selection(&requests[0], "forum", "thread"));
    assert!(request_carries_id(&requests[0], "777"));
    assert_eq!(requests[0].query.get("timestamp"), Some(&"1".to_string()));
    assert!(request_targets_selection(&requests[1], "racing", "race"));
    assert!(request_carries_id(&requests[1], "9001"));
    assert!(request_targets_selection(
        &requests[2],
        "market",
        "itemmarket"
    ));
    assert!(request_carries_id(&requests[2], "2"));
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

    let trade_error = sdk
        .user()
        .trade_raw(UserOptions::default())
        .await
        .expect_err("trade without trade_id should fail");
    match trade_error {
        SdkError::Validation(message) => assert!(message.contains("tradeId")),
        other => panic!("unexpected error: {other:?}"),
    }

    let inventory_error = sdk
        .user()
        .inventory_raw(UserOptions::default())
        .await
        .expect_err("inventory without cat should fail");
    match inventory_error {
        SdkError::Validation(message) => assert!(message.contains("cat")),
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
        "company".to_string(),
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
        "company",
        &capabilities,
        CompanyApi::<MockTransport>::SUPPORTED_SELECTIONS,
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
