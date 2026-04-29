mod support;

use std::path::PathBuf;
use std::time::Duration;

use support::MockTransport;
use torn_sdk_planner::models::manual::torn::TornCardShort;
use torn_sdk_planner::{
    BaseOptions, CapabilitiesDocument, ExecutorConfig, RateLimitConfig, RequestExecutor,
    RequestPlanner, SdkError, TornClient, TornOptions, TornSdk, TransportError, TransportResponse,
};

fn test_config() -> ExecutorConfig {
    ExecutorConfig {
        base_url_v2: "https://api.torn.com/v2".to_string(),
        base_url_v1: "https://api.torn.com".to_string(),
        timeout: Duration::from_secs(5),
        user_agent: "torn-typed-contract-test".to_string(),
        max_attempts: 1,
        network_retry_backoff: Duration::from_millis(1),
        rate_limits: RateLimitConfig {
            per_key_per_minute: 1000,
            per_ip_per_minute: 1000,
        },
        max_in_flight: 4,
    }
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
async fn torn_wrapper_typed_catalog_methods_deserialize() {
    let (sdk, transport) = make_sdk_with_responses(vec![
        Ok(TransportResponse {
            status: 200,
            body: r#"{"logcategories":[{"id":1,"title":"Account creation"}]}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"logtypes":[{"id":1,"title":"Created account"}]}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"logtypes":[{"id":101,"title":"Successful login"}]}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"merits":[{"id":7,"name":"Bank Interest","description":"Extra interest"}]}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"itemammo":[{"id":2,"name":"9mm Parabellum Round","price":9,"types":["Standard","Tracer"]}]}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"itemmods":[{"id":1,"name":"Reflex Sight","description":"+1 Accuracy","dual_fit":false,"weapons":["Rifle","SMG"]}]}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"properties":[{"id":1,"name":"Trailer","cost":5000,"happy":110,"upkeep":10,"modifications":["Hot Tub"],"staff":["Maid"]}]}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"stocks":[{"id":1,"name":"Torn & Shanghai Banking","acronym":"TSB","images":{"logo":"logo.svg","full":"full.svg"},"market":{"price":1179.43,"cap":100,"shares":10,"investors":3},"bonus":{"passive":false,"frequency":31,"requirement":3000000,"description":"$50,000,000"}}]}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"stocks":{"id":1,"name":"Torn & Shanghai Banking","acronym":"TSB","images":{"logo":"logo.svg","full":"full.svg"},"market":{"price":1179.43,"cap":100,"shares":10,"investors":3},"bonus":{"passive":false,"frequency":31,"requirement":3000000,"description":"$50,000,000"},"chart":{"performance":{"last_hour":{"change":-0.74,"change_percentage":-0.06,"start":1180.17,"end":1179.43,"high":1180.19,"low":1178.82}},"history":[{"timestamp":1777419420,"price":1179.43,"change":-0.11}]}}}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"bounties_timestamp":1777419350,"bounties_delay":30,"bounties":[{"target_id":2488304,"target_name":"nippythefish","target_level":100,"lister_id":2487783,"lister_name":"Offertory","reward":5000001,"reason":null,"quantity":1,"is_anonymous":false,"valid_until":1777841442}],"_metadata":{"links":{"prev":null,"next":"https://api.torn.com/v2/torn/bounties?limit=1&offset=1"},"total":5226}}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"bank":{"1w":39.19,"2w":41.11,"1m":43.51,"2m":46.32,"3m":47.82}}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"attacklog":{"summary":[{"id":123,"name":"Attacker","hits":2,"misses":1,"damage":450},{"id":456,"name":"Defender","hits":0,"misses":0,"damage":0}],"log":[{"text":"Attacker fired","timestamp":1710000600,"action":"hit","icon":"attack","attacker":{"id":123,"name":"Attacker","item":{"id":1,"name":"AK-47"}},"defender":{"id":456,"name":"Defender"}},{"text":"Stealthed temporary","timestamp":1710000601,"action":"special","icon":"temporary","attacker":null,"defender":null,"attacker_item":{"id":2,"name":"Smoke Grenade"}}]},"_metadata":{"links":{"prev":null,"next":"https://api.torn.com/v2/torn/attacklog?offset=2"}}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"cards":{"1":{"name":"Two of Spades","short":2,"rank":2,"class":"spades-2"},"49":{"name":"Ace of Spades","short":"A","rank":14,"class":"spades-a"}}}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"cityshops":{"100":{"name":"Big Al's Gun Shop","inventory":{"1":{"name":"Hammer","type":"Melee","price":75,"in_stock":998}}}}}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"companies":{"1":{"name":"Hair Salon","cost":750000,"default_employees":4,"positions":{"Stylist":{"man_required":1500,"int_required":0,"end_required":750,"man_gain":34,"int_gain":0,"end_gain":17,"special_ability":"None","description":"Cuts hair"}}}}}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"gyms":{"1":{"name":"Premier Fitness","stage":1,"cost":10,"energy":5,"strength":20,"speed":20,"defense":20,"dexterity":20,"note":""}}}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"itemstats":{"armoryID":17,"ID":1,"sell":500,"Qty":3,"SellTotal":1500,"name":"AK-47","type2":"Rifle","dmg":42.5,"damage":43.0,"accuracy":55.0,"acc":54.5,"arm":null,"weptype":"Primary","UID":999,"type":"Weapon","originalType":"Primary","market_price":2500000,"stats":[{"damage":43}],"OddCase":"preserved"}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"itemdetails":{"id":1,"name":"AK-47","type":"Primary","details":{"category":"Weapons"}}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"pawnshop":{"points_value":0,"donatorpack_value":23500000}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"pokertables":{"2":{"name":"Newbie Corner","big_blind":10,"small_blind":5,"speed":30,"current_players":3,"maximum_players":9}}}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"selections":["attacklog","calendar","lookup","timestamp"]}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"timestamp":1710000500}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"competition":[]}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"crimes":[{"id":1,"name":"Bootlegging"}]}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"education":[{"id":1,"name":"General Studies"}]}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"elimination":{"teams":[{"id":1,"name":"Red"}]}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"eliminationteam":[{"id":1,"name":"Red","rank":2,"score":150.5}],"_metadata":{"links":{"prev":null,"next":null}}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"factionhof":{"respect":{"rank":1,"value":1000}}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"factiontree":{"branches":[{"id":1,"name":"Core"}]}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"hof":{"level":{"rank":1,"value":100}}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"organisedcrimes":{"1":{"name":"Bomb Threat"}}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"organizedcrimes":[{"id":1,"name":"Blackmailing"}]}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"rockpaperscissors":[{"type":"rock","count":7}]}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"searchforcash":{"beach":{"title":"Beach","percentage":12.5}}}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"shoplifting":{"mall":[{"title":"Candy Bar","disabled":false}]}}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"stats":{"players":1000,"items":5000}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"subcrimes":{"id":99,"name":"Kidnapping"}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"territory":{"sectors":[{"id":1,"name":"Docks"}]}}"#.to_string(),
        }),
    ]);

    let logcategories = sdk
        .torn()
        .logcategories(TornOptions::default())
        .await
        .expect("logcategories should deserialize");
    assert_eq!(logcategories.logcategories.len(), 1);
    assert_eq!(logcategories.logcategories[0].id, Some(1));

    let logtypes = sdk
        .torn()
        .logtypes(TornOptions::default())
        .await
        .expect("logtypes should deserialize");
    assert_eq!(logtypes.logtypes[0].id, Some(1));

    let scoped_logtypes = sdk
        .torn()
        .logtypes(TornOptions::default().with_log_category_id("1"))
        .await
        .expect("scoped logtypes should deserialize");
    assert_eq!(scoped_logtypes.logtypes[0].id, Some(101));

    let merits = sdk
        .torn()
        .merits(TornOptions::default())
        .await
        .expect("merits should deserialize");
    assert_eq!(merits.merits[0].name.as_deref(), Some("Bank Interest"));

    let itemammo = sdk
        .torn()
        .itemammo(TornOptions::default())
        .await
        .expect("itemammo should deserialize");
    assert_eq!(itemammo.itemammo[0].types.len(), 2);

    let itemmods = sdk
        .torn()
        .itemmods(TornOptions::default())
        .await
        .expect("itemmods should deserialize");
    assert_eq!(itemmods.itemmods[0].dual_fit, Some(false));

    let properties = sdk
        .torn()
        .properties(TornOptions::default())
        .await
        .expect("properties should deserialize");
    assert_eq!(properties.properties[0].modifications[0], "Hot Tub");

    let stocks = sdk
        .torn()
        .stocks(TornOptions::default())
        .await
        .expect("stocks should deserialize");
    assert_eq!(stocks.stocks[0].id, Some(1));

    let stock = sdk
        .torn()
        .stock(TornOptions::default().with_stock_id("1"))
        .await
        .expect("stock should deserialize");
    assert_eq!(stock.stock.id, Some(1));
    assert_eq!(
        stock
            .stock
            .chart
            .as_ref()
            .and_then(|chart| chart.history.first())
            .and_then(|point| point.timestamp),
        Some(1777419420)
    );

    let bounties = sdk
        .torn()
        .bounties(TornOptions::default())
        .await
        .expect("bounties should deserialize");
    assert_eq!(bounties.bounties[0].reward, Some(5_000_001));
    assert_eq!(
        bounties
            ._metadata
            .as_ref()
            .and_then(|metadata| metadata.total),
        Some(5226)
    );

    let bank = sdk
        .torn()
        .bank(TornOptions::default())
        .await
        .expect("bank should deserialize");
    assert_eq!(bank.bank.one_week, Some(39.19));

    let attacklog = sdk
        .torn()
        .attacklog(TornOptions::default().with_base(BaseOptions::default().with_log("deadbeef")))
        .await
        .expect("attacklog should deserialize");
    assert_eq!(attacklog.attacklog.summary[0].hits, Some(2));
    assert_eq!(
        attacklog.attacklog.log[0]
            .attacker
            .as_ref()
            .and_then(|attacker| attacker.item.as_ref())
            .and_then(|item| item.name.as_deref()),
        Some("AK-47")
    );
    assert_eq!(
        attacklog.attacklog.log[1]
            .attacker_item
            .as_ref()
            .and_then(|item| item.name.as_deref()),
        Some("Smoke Grenade")
    );
    assert_eq!(
        attacklog
            ._metadata
            .as_ref()
            .and_then(|metadata| metadata.links.as_ref())
            .and_then(|links| links.next.as_deref()),
        Some("https://api.torn.com/v2/torn/attacklog?offset=2")
    );

    let cards = sdk
        .torn()
        .cards(TornOptions::default())
        .await
        .expect("cards should deserialize");
    assert!(matches!(
        cards.cards["49"].short,
        Some(TornCardShort::Label(ref value)) if value == "A"
    ));

    let cityshops = sdk
        .torn()
        .cityshops(TornOptions::default())
        .await
        .expect("cityshops should deserialize");
    assert_eq!(
        cityshops.cityshops["100"].inventory["1"]
            .item_type
            .as_deref(),
        Some("Melee")
    );

    let companies = sdk
        .torn()
        .companies(TornOptions::default())
        .await
        .expect("companies should deserialize");
    assert_eq!(
        companies.companies["1"].positions["Stylist"]
            .special_ability
            .as_deref(),
        Some("None")
    );

    let gyms = sdk
        .torn()
        .gyms(TornOptions::default())
        .await
        .expect("gyms should deserialize");
    assert_eq!(gyms.gyms["1"].strength, Some(20));

    let itemstats = sdk
        .torn()
        .itemstats(TornOptions::default().with_id("1"))
        .await
        .expect("itemstats should deserialize");
    assert_eq!(itemstats.itemstats.armory_id, Some(17));
    assert_eq!(itemstats.itemstats.qty, Some(3));
    assert_eq!(itemstats.itemstats.dmg, Some(42.5));
    assert_eq!(itemstats.itemstats.damage, Some(43.0));
    assert_eq!(itemstats.itemstats.weptype.as_deref(), Some("Primary"));
    assert_eq!(
        itemstats.itemstats.extra.get("OddCase"),
        Some(&serde_json::json!("preserved"))
    );

    let itemdetails = sdk
        .torn()
        .itemdetails(TornOptions::default().with_id("1"))
        .await
        .expect("itemdetails should deserialize");
    assert_eq!(
        itemdetails
            .itemdetails
            .as_ref()
            .and_then(|value| value.get("name"))
            .and_then(|value| value.as_str()),
        Some("AK-47")
    );

    let pawnshop = sdk
        .torn()
        .pawnshop(TornOptions::default())
        .await
        .expect("pawnshop should deserialize");
    assert_eq!(pawnshop.pawnshop.donatorpack_value, Some(23_500_000));

    let pokertables = sdk
        .torn()
        .pokertables(TornOptions::default())
        .await
        .expect("pokertables should deserialize");
    assert_eq!(pokertables.pokertables["2"].maximum_players, Some(9));

    let lookup = sdk
        .torn()
        .lookup(TornOptions::default())
        .await
        .expect("lookup should deserialize");
    assert!(lookup.selections.iter().any(|value| value == "lookup"));

    let timestamp = sdk
        .torn()
        .timestamp(TornOptions::default())
        .await
        .expect("timestamp should deserialize");
    assert_eq!(timestamp.timestamp, Some(1710000500));

    let competition = sdk
        .torn()
        .competition(TornOptions::default())
        .await
        .expect("competition should deserialize");
    assert!(competition.competition.is_empty());

    let crimes = sdk
        .torn()
        .crimes(TornOptions::default())
        .await
        .expect("crimes should deserialize");
    assert_eq!(crimes.crimes.len(), 1);

    let education = sdk
        .torn()
        .education(TornOptions::default())
        .await
        .expect("education should deserialize");
    assert_eq!(education.education.len(), 1);

    let elimination = sdk
        .torn()
        .elimination(TornOptions::default())
        .await
        .expect("elimination should deserialize");
    assert!(elimination.elimination.is_some());

    let eliminationteam = sdk
        .torn()
        .eliminationteam(TornOptions::default().with_id("1"))
        .await
        .expect("eliminationteam should deserialize");
    assert_eq!(
        eliminationteam.eliminationteam[0].name.as_deref(),
        Some("Red")
    );
    assert!(eliminationteam._metadata.is_some());

    let factionhof = sdk
        .torn()
        .factionhof(TornOptions::default())
        .await
        .expect("factionhof should deserialize");
    assert!(factionhof.factionhof.is_some());

    let factiontree = sdk
        .torn()
        .factiontree(TornOptions::default())
        .await
        .expect("factiontree should deserialize");
    assert!(factiontree.factiontree.is_some());

    let hof = sdk
        .torn()
        .hof(TornOptions::default())
        .await
        .expect("hof should deserialize");
    assert!(hof.hof.is_some());

    let organisedcrimes = sdk
        .torn()
        .organisedcrimes(TornOptions::default())
        .await
        .expect("organisedcrimes should deserialize");
    assert!(organisedcrimes.organisedcrimes.contains_key("1"));

    let organizedcrimes = sdk
        .torn()
        .organizedcrimes(TornOptions::default())
        .await
        .expect("organizedcrimes should deserialize");
    assert_eq!(organizedcrimes.organizedcrimes.len(), 1);

    let rockpaperscissors = sdk
        .torn()
        .rockpaperscissors(TornOptions::default())
        .await
        .expect("rockpaperscissors should deserialize");
    assert_eq!(rockpaperscissors.rockpaperscissors[0].count, Some(7));

    let searchforcash = sdk
        .torn()
        .searchforcash(TornOptions::default())
        .await
        .expect("searchforcash should deserialize");
    assert_eq!(searchforcash.searchforcash["beach"].percentage, Some(12.5));

    let shoplifting = sdk
        .torn()
        .shoplifting(TornOptions::default())
        .await
        .expect("shoplifting should deserialize");
    assert_eq!(shoplifting.shoplifting["mall"][0].disabled, Some(false));

    let stats = sdk
        .torn()
        .stats(TornOptions::default())
        .await
        .expect("stats should deserialize");
    assert_eq!(stats.stats["players"], serde_json::json!(1000));

    let subcrimes = sdk
        .torn()
        .subcrimes(TornOptions::default().with_crime_id("99"))
        .await
        .expect("subcrimes should deserialize");
    assert!(subcrimes.subcrimes.is_some());

    let territory = sdk
        .torn()
        .territory(TornOptions::default())
        .await
        .expect("territory should deserialize");
    assert!(territory.territory.is_some());

    assert_eq!(transport.requests().len(), 38);
}

#[tokio::test]
async fn torn_wrapper_stock_helper_validation_is_explicit() {
    let (sdk, _) = make_sdk_with_responses(Vec::new());

    let list_error = sdk
        .torn()
        .stocks(TornOptions::default().with_stock_id("1"))
        .await
        .expect_err("list helper should reject stockId");
    assert!(matches!(list_error, SdkError::Validation(_)));

    let detail_error = sdk
        .torn()
        .stock(TornOptions::default())
        .await
        .expect_err("detail helper should require stockId");
    assert!(matches!(detail_error, SdkError::Validation(_)));

    let attacklog_error = sdk
        .torn()
        .attacklog(TornOptions::default())
        .await
        .expect_err("attacklog should require log");
    assert!(matches!(attacklog_error, SdkError::Validation(_)));

    let itemstats_error = sdk
        .torn()
        .itemstats(TornOptions::default())
        .await
        .expect_err("itemstats should require id");
    assert!(matches!(itemstats_error, SdkError::Validation(_)));

    let itemdetails_error = sdk
        .torn()
        .itemdetails(TornOptions::default())
        .await
        .expect_err("itemdetails should require id");
    assert!(matches!(itemdetails_error, SdkError::Validation(_)));

    let eliminationteam_error = sdk
        .torn()
        .eliminationteam(TornOptions::default())
        .await
        .expect_err("eliminationteam should require id");
    assert!(matches!(eliminationteam_error, SdkError::Validation(_)));
}
