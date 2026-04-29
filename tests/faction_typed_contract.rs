#[allow(dead_code)]
mod support;

use std::path::PathBuf;
use std::time::Duration;

use support::MockTransport;
use torn_sdk_planner::models::manual::faction::{
    FactionCrimeExpSelection, FactionLegacyItemSelection, FactionWarfareEntry,
};
use torn_sdk_planner::{
    BaseOptions, ExecutorConfig, FactionOptions, RateLimitConfig, RequestExecutor, RequestPlanner,
    TornClient, TornSdk, TransportError, TransportResponse,
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
        user_agent: "faction-typed-contract-test".to_string(),
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
    let client = TornClient::new(planner, executor);
    (TornSdk::new(client), transport)
}

#[tokio::test]
async fn faction_typed_methods_deserialize_stable_selection_shapes() {
    let (sdk, transport) = make_sdk_with_responses(vec![
        Ok(TransportResponse {
            status: 200,
            body: r#"{"applications":[{"id":1,"message":"hello","valid_until":1710000200,"status":"pending","user":{"id":99,"name":"Applicant","level":30,"stats":{"strength":1,"speed":2,"dexterity":3,"defense":4}}}]}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"selections":["applications","chain","chains","news","territoryownership","warfare"]}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"chain":{"id":0,"current":0,"max":10,"timeout":0,"modifier":1,"cooldown":0,"start":0,"end":1777422590}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"chains":[{"id":62264214,"chain":19,"respect":53,"start":1777416675,"end":1777417656}],"_metadata":{"links":{"prev":"https://api.torn.com/v2/faction/16312/chains?&limit=1&sort=desc&to=1777416675","next":null}}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"chainreport":{"id":62264214,"faction_id":16312,"start":1777416675,"end":1777417656,"details":{"chain":19,"respect":53,"members":2,"targets":19,"war":0,"best":10,"leave":17,"mug":2,"hospitalize":0,"assists":0,"retaliations":0,"overseas":0,"draws":0,"escapes":2,"losses":0},"bonuses":[{"attacker_id":1328948,"defender_id":487915,"chain":10,"respect":10}],"attackers":[{"id":8491,"respect":{"total":5.81,"average":2.91,"best":3.33},"attacks":{"total":2,"leave":0,"mug":2,"hospitalize":0,"assists":0,"retaliations":0,"overseas":0,"draws":0,"escapes":0,"losses":0,"war":0,"bonuses":0}}],"non_attackers":[2959112,21955]}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"balance":{"faction":{"money":1000,"points":50,"scope":1},"members":[{"id":1,"username":"One","money":10,"points":2}]}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"contributors":[{"id":1,"username":"One","value":100,"in_faction":true}]}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"rackets":[{"territory":"IFE","faction_id":7835,"name":"Bootleg Distillery IV","level":4,"description":"80x Bottle of Moonshine daily","reward":{"type":"Item","quantity":80,"id":984},"created_at":1592153824,"changed_at":1774501023}]}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"hof":{"rank":{"rank":"A","value":"Top"},"respect":{"rank":4,"value":5000},"chain":{"rank":2,"value":300}}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"positions":[{"name":"Leader","is_default":false,"abilities":["armory","kick"]}]}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"search":[{"id":123,"name":"Faction A","respect":500,"members":10,"leader":{"id":1,"name":"Lead"},"co_leader":null,"image":null,"tag_image":"tag.png","tag":"TAG","is_destroyed":false,"is_recruiting":true}],"_metadata":{"links":{"next":null,"prev":null}}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"stats":[{"name":"attacks","value":25}]}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"raids":[{"id":7382,"start":1757164550,"end":1757168483,"aggressor":{"id":16312,"name":"39th Street Killers X","score":0},"defender":{"id":54858,"name":"39th Street Creepers","score":0}}],"_metadata":{"links":{"prev":"https://api.torn.com/v2/faction/raids?&limit=1&sort=desc&to=1757164550","next":null}}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"timestamp":1710000300}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"territory":[{"id":"KDD","acquired_at":1724193041,"sector":5,"size":3,"density":2,"slots":21,"respect":90,"coordinates":{"x":4257.67,"y":1819.67},"racket":{"territory":"KDD","faction_id":10174,"name":"Money Launderer II","level":2,"description":"$20,000,000 daily","reward":{"type":"Money","quantity":20000000,"id":null},"created_at":1598893024,"changed_at":1776009423}}]}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"territoryOwnership":[{"id":"AAB","owned_by":23193,"acquired_at":1773784300},{"id":"AAG","owned_by":null,"acquired_at":null}]}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"state":"peace","upgrades":{"core":{"upgrades":[{"id":1,"name":"Core A","ability":"Thing","level":2,"cost":1000,"unlocked_at":1710000000}]},"peace":[{"name":"Peace","order":1,"multiplier":1.5,"upgrades":[]}],"war":[]}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"warfare":[{"id":62267143,"chain":10,"respect":44.35,"start":1777422161,"end":1777422431,"faction":{"id":56304,"name":"Reckless Pirate Co"}}],"_metadata":{"links":{"prev":null,"next":"https://api.torn.com/v2/faction/warfare?cat=chain&from=1777422161"}}}"#.to_string(),
        }),
    ]);

    let applications = sdk
        .faction()
        .applications(FactionOptions::default())
        .await
        .expect("applications should deserialize");
    assert_eq!(applications.applications.len(), 1);
    assert_eq!(applications.applications[0].id, Some(1));

    let lookup = sdk
        .faction()
        .lookup(FactionOptions::default())
        .await
        .expect("lookup should deserialize");
    assert!(
        lookup
            .selections
            .iter()
            .any(|selection| selection == "warfare")
    );

    let chain = sdk
        .faction()
        .chain(FactionOptions::default())
        .await
        .expect("chain should deserialize");
    assert_eq!(chain.chain.max, Some(10));

    let chains = sdk
        .faction()
        .chains(FactionOptions::default().with_base(BaseOptions::default().with_limit(1)))
        .await
        .expect("chains should deserialize");
    assert_eq!(chains.chains[0].respect, Some(53.0));
    assert!(chains._metadata.is_some());

    let chainreport = sdk
        .faction()
        .chainreport(FactionOptions::default().with_chain_id("62264214"))
        .await
        .expect("chainreport should deserialize");
    assert_eq!(
        chainreport
            .chainreport
            .details
            .as_ref()
            .and_then(|details| details.escapes),
        Some(2)
    );
    assert_eq!(chainreport.chainreport.non_attackers.len(), 2);

    let balance = sdk
        .faction()
        .balance(FactionOptions::default())
        .await
        .expect("balance should deserialize");
    assert_eq!(
        balance
            .balance
            .faction
            .as_ref()
            .and_then(|value| value.money),
        Some(1000)
    );

    let contributors = sdk
        .faction()
        .contributors(
            FactionOptions::default().with_base(BaseOptions::default().with_stat("attacks")),
        )
        .await
        .expect("contributors should deserialize");
    assert_eq!(contributors.contributors[0].value, Some(100));

    let rackets = sdk
        .faction()
        .rackets(FactionOptions::default())
        .await
        .expect("rackets should deserialize");
    assert_eq!(
        rackets.rackets[0]
            .reward
            .as_ref()
            .and_then(|reward| reward.id),
        Some(984)
    );

    let hof = sdk
        .faction()
        .hof(FactionOptions::default().with_id("123"))
        .await
        .expect("hof should deserialize");
    assert_eq!(
        hof.hof.respect.as_ref().and_then(|value| value.value),
        Some(5000)
    );

    let positions = sdk
        .faction()
        .positions(FactionOptions::default())
        .await
        .expect("positions should deserialize");
    assert_eq!(positions.positions[0].name.as_deref(), Some("Leader"));

    let search = sdk
        .faction()
        .search(FactionOptions::default().with_base(BaseOptions::default().with_name("Fact")))
        .await
        .expect("search should deserialize");
    assert_eq!(search.search.len(), 1);
    assert!(search._metadata.is_some());

    let stats = sdk
        .faction()
        .stats(FactionOptions::default())
        .await
        .expect("stats should deserialize");
    assert_eq!(stats.stats[0].value, Some(25));

    let raids = sdk
        .faction()
        .raids(FactionOptions::default().with_base(BaseOptions::default().with_limit(1)))
        .await
        .expect("raids should deserialize");
    assert_eq!(
        raids.raids[0].aggressor.as_ref().and_then(|value| value.id),
        Some(16312)
    );
    assert!(raids._metadata.is_some());

    let timestamp = sdk
        .faction()
        .timestamp(FactionOptions::default())
        .await
        .expect("timestamp should deserialize");
    assert_eq!(timestamp.timestamp, Some(1710000300));

    let territory = sdk
        .faction()
        .territory(FactionOptions::default())
        .await
        .expect("territory should deserialize");
    assert_eq!(territory.territory[0].id.as_deref(), Some("KDD"));
    assert_eq!(
        territory.territory[0]
            .racket
            .as_ref()
            .and_then(|racket| racket.reward.as_ref())
            .and_then(|reward| reward.quantity),
        Some(20_000_000)
    );

    let territoryownership = sdk
        .faction()
        .territoryownership(FactionOptions::default())
        .await
        .expect("territoryownership should deserialize");
    assert_eq!(territoryownership.territory_ownership.len(), 2);
    assert_eq!(territoryownership.territory_ownership[1].owned_by, None);

    let upgrades = sdk
        .faction()
        .upgrades(FactionOptions::default())
        .await
        .expect("upgrades should deserialize");
    assert_eq!(upgrades.state.as_deref(), Some("peace"));
    assert_eq!(
        upgrades
            .upgrades
            .as_ref()
            .and_then(|value| value.core.as_ref())
            .map(|value| value.upgrades.len()),
        Some(1)
    );

    let warfare = sdk
        .faction()
        .warfare(FactionOptions::default().with_base(BaseOptions::default().with_cat("chain")))
        .await
        .expect("warfare should deserialize");
    match &warfare.warfare[0] {
        FactionWarfareEntry::Chain(entry) => {
            assert_eq!(entry.chain, Some(10));
            assert_eq!(
                entry
                    .faction
                    .as_ref()
                    .and_then(|faction| faction.name.as_deref()),
                Some("Reckless Pirate Co")
            );
        }
        other => panic!("expected chain warfare entry, got {other:?}"),
    }
    assert!(warfare._metadata.is_some());

    let requests = transport.requests();
    assert_eq!(requests.len(), 18);
    let contributors_request = requests
        .iter()
        .find(|request| request.query.get("selections") == Some(&"contributors".to_string()))
        .expect("contributors request should be captured");
    assert_eq!(
        contributors_request.query.get("stat"),
        Some(&"attacks".to_string())
    );

    let hof_request = requests
        .iter()
        .find(|request| request.query.get("selections") == Some(&"hof".to_string()))
        .expect("hof request should be captured");
    assert_eq!(hof_request.query.get("id"), Some(&"123".to_string()));

    let warfare_request = requests
        .iter()
        .find(|request| request.query.get("selections") == Some(&"warfare".to_string()))
        .expect("warfare request should be captured");
    assert_eq!(warfare_request.query.get("cat"), Some(&"chain".to_string()));

    let chain_request = requests
        .iter()
        .find(|request| request.query.get("selections") == Some(&"chain".to_string()))
        .expect("chain request should be captured");
    assert!(!chain_request.query.contains_key("id"));

    let chains_request = requests
        .iter()
        .find(|request| request.query.get("selections") == Some(&"chains".to_string()))
        .expect("chains request should be captured");
    assert!(!chains_request.query.contains_key("id"));

    let chainreport_request = requests
        .iter()
        .find(|request| request.query.get("selections") == Some(&"chainreport".to_string()))
        .expect("chainreport request should be captured");
    assert_eq!(
        chainreport_request.query.get("id"),
        Some(&"62264214".to_string())
    );

    let raids_request = requests
        .iter()
        .find(|request| request.query.get("selections") == Some(&"raids".to_string()))
        .expect("raids request should be captured");
    assert!(!raids_request.query.contains_key("id"));

    let territory_request = requests
        .iter()
        .find(|request| request.query.get("selections") == Some(&"territory".to_string()))
        .expect("territory request should be captured");
    assert!(!territory_request.query.contains_key("id"));
}

#[tokio::test]
async fn faction_validation_reports_missing_required_query_fields() {
    let (sdk, _) = make_sdk_with_responses(Vec::new());

    let contributors_error = sdk
        .faction()
        .contributors(FactionOptions::default())
        .await
        .expect_err("contributors without stat should fail");
    assert!(matches!(
        contributors_error,
        torn_sdk_planner::SdkError::Validation(message) if message.contains("stat")
    ));

    let search_error = sdk
        .faction()
        .search(FactionOptions::default())
        .await
        .expect_err("search without name should fail");
    assert!(matches!(
        search_error,
        torn_sdk_planner::SdkError::Validation(message) if message.contains("name")
    ));

    let crime_error = sdk
        .faction()
        .crime(FactionOptions::default())
        .await
        .expect_err("crime without crimeId should fail");
    assert!(matches!(
        crime_error,
        torn_sdk_planner::SdkError::Validation(message) if message.contains("crimeId")
    ));

    let warfare_error = sdk
        .faction()
        .warfare(FactionOptions::default())
        .await
        .expect_err("warfare without cat should fail");
    assert!(matches!(
        warfare_error,
        torn_sdk_planner::SdkError::Validation(message) if message.contains("cat")
    ));
}

#[tokio::test]
async fn faction_typed_armory_and_crime_surfaces_deserialize() {
    let (sdk, transport) = make_sdk_with_responses(vec![
        Ok(TransportResponse {
            status: 200,
            body: r#"{"armor":{"11":{"name":"Kevlar Vest","type":"Defensive","quantity":2,"available":1,"loaned":1,"loaned_to":8491,"armor":35.5}}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"boosters":[{"id":12,"name":"Boxing Booster","type":"Booster","quantity":3,"available":3,"loaned":0}]}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"caches":{"13":{"name":"Weapons Cache","quantity":1,"available":1}}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"cesium":[{"id":14,"name":"Cesium-137","quantity":2,"available":2}]}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"crime":{"id":1186857,"name":"Cracking the Vault","previous_crime_id":1186000,"difficulty":8,"status":"Successful","created_at":1710000000,"planning_at":1710000100,"ready_at":1710000200,"expired_at":1710000300,"executed_at":1710000400,"slots":[{"position":"Hacker","position_info":{"id":"P1","label":"Hacker","number":1},"position_id":"P1","position_number":1,"item_requirement":{"id":77,"is_reusable":true,"is_available":true},"user":{"id":8491,"outcome":"Successful","outcome_duration":45,"item_outcome":{"owned_by":"User","item_id":77,"item_uid":9001,"outcome":"Used"},"joined_at":1710000050,"progress":99.5},"checkpoint_pass_rate":42}],"rewards":{"money":750000,"items":[{"id":5,"quantity":2}],"respect":12,"scope":3,"payout":{"type":"automatic","percentage":100.0,"paid_by":1631210,"paid_at":1710000500}}}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"crimeexp":{"8491":{"name":"Hencia","crimeexp":1250,"position":"Planner"},"21955":{"name":"Charger","crime_exp":1001}}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"crimes":[{"id":1186857,"name":"Cracking the Vault","previous_crime_id":null,"difficulty":8,"status":"Planning","created_at":1710000000,"planning_at":1710000100,"ready_at":1710000200,"expired_at":1710000300,"executed_at":null,"slots":[{"position":"Hacker","position_info":{"id":"P1","label":"Hacker","number":1},"position_id":"P1","position_number":1,"item_requirement":null,"user":null,"checkpoint_pass_rate":42}],"rewards":null}],"_metadata":{"links":{"prev":null,"next":"https://api.torn.com/v2/faction/crimes?offset=1"}}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"drugs":{"15":{"name":"Xanax","quantity":25,"available":20,"loaned":5}}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"medical":[{"id":16,"name":"Morphine","quantity":8,"available":6,"loaned":2}]}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"temporary":{"17":{"name":"Smoke Grenade","quantity":4,"available":4,"bonus":"escape"}}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"utilities":[{"id":18,"name":"Flashlight","quantity":9,"available":9,"required":false}]}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"weapons":{"19":{"name":"Armalite","type":"Primary","quantity":2,"available":1,"loaned":1,"damage":75.5,"accuracy":62.0}}}"#.to_string(),
        }),
    ]);

    let armor = sdk
        .faction()
        .armor(FactionOptions::default())
        .await
        .expect("armor should deserialize");
    match &armor.armor {
        FactionLegacyItemSelection::Map(entries) => {
            let item = entries.get("11").expect("armor item should exist");
            assert_eq!(item.name.as_deref(), Some("Kevlar Vest"));
            assert_eq!(item.loaned, Some(1));
        }
        other => panic!("expected armor map selection, got {other:?}"),
    }

    let boosters = sdk
        .faction()
        .boosters(FactionOptions::default())
        .await
        .expect("boosters should deserialize");
    match &boosters.boosters {
        FactionLegacyItemSelection::List(entries) => {
            assert_eq!(entries[0].item_type.as_deref(), Some("Booster"));
            assert_eq!(entries[0].quantity, Some(3));
        }
        other => panic!("expected boosters list selection, got {other:?}"),
    }

    let caches = sdk
        .faction()
        .caches(FactionOptions::default())
        .await
        .expect("caches should deserialize");
    match &caches.caches {
        FactionLegacyItemSelection::Map(entries) => {
            assert_eq!(entries.get("13").and_then(|entry| entry.available), Some(1));
        }
        other => panic!("expected caches map selection, got {other:?}"),
    }

    let cesium = sdk
        .faction()
        .cesium(FactionOptions::default())
        .await
        .expect("cesium should deserialize");
    match &cesium.cesium {
        FactionLegacyItemSelection::List(entries) => {
            assert_eq!(entries[0].name.as_deref(), Some("Cesium-137"));
        }
        other => panic!("expected cesium list selection, got {other:?}"),
    }

    let crime = sdk
        .faction()
        .crime(FactionOptions::default().with_crime_id("1186857"))
        .await
        .expect("crime should deserialize");
    assert_eq!(crime.crime.id, Some(1_186_857));
    assert_eq!(crime.crime.slots.len(), 1);
    assert_eq!(
        crime.crime.slots[0]
            .position_info
            .as_ref()
            .and_then(|info| info.id.as_deref()),
        Some("P1")
    );
    assert_eq!(
        crime.crime.slots[0]
            .user
            .as_ref()
            .and_then(|user| user.item_outcome.as_ref())
            .and_then(|outcome| outcome.item_uid),
        Some(9_001)
    );
    assert_eq!(
        crime
            .crime
            .rewards
            .as_ref()
            .and_then(|reward| reward.payout.as_ref())
            .and_then(|payout| payout.percentage),
        Some(100.0)
    );

    let crimeexp = sdk
        .faction()
        .crimeexp(FactionOptions::default())
        .await
        .expect("crimeexp should deserialize");
    match &crimeexp.crimeexp {
        FactionCrimeExpSelection::Map(entries) => {
            assert_eq!(
                entries.get("8491").and_then(|entry| entry.crime_exp),
                Some(1_250)
            );
            assert_eq!(
                entries.get("21955").and_then(|entry| entry.crime_exp),
                Some(1_001)
            );
        }
        other => panic!("expected crimeexp map selection, got {other:?}"),
    }

    let crimes = sdk
        .faction()
        .crimes(FactionOptions::default().with_base(BaseOptions::default().with_limit(1)))
        .await
        .expect("crimes should deserialize");
    assert_eq!(crimes.crimes.len(), 1);
    assert_eq!(crimes.crimes[0].status.as_deref(), Some("Planning"));
    assert!(crimes._metadata.is_some());

    let drugs = sdk
        .faction()
        .drugs(FactionOptions::default())
        .await
        .expect("drugs should deserialize");
    match &drugs.drugs {
        FactionLegacyItemSelection::Map(entries) => {
            assert_eq!(entries.get("15").and_then(|entry| entry.loaned), Some(5));
        }
        other => panic!("expected drugs map selection, got {other:?}"),
    }

    let medical = sdk
        .faction()
        .medical(FactionOptions::default())
        .await
        .expect("medical should deserialize");
    match &medical.medical {
        FactionLegacyItemSelection::List(entries) => {
            assert_eq!(entries[0].name.as_deref(), Some("Morphine"));
        }
        other => panic!("expected medical list selection, got {other:?}"),
    }

    let temporary = sdk
        .faction()
        .temporary(FactionOptions::default())
        .await
        .expect("temporary should deserialize");
    match &temporary.temporary {
        FactionLegacyItemSelection::Map(entries) => {
            assert_eq!(
                entries.get("17").and_then(|entry| entry.bonus.as_deref()),
                Some("escape")
            );
        }
        other => panic!("expected temporary map selection, got {other:?}"),
    }

    let utilities = sdk
        .faction()
        .utilities(FactionOptions::default())
        .await
        .expect("utilities should deserialize");
    match &utilities.utilities {
        FactionLegacyItemSelection::List(entries) => {
            assert_eq!(entries[0].required, Some(false));
        }
        other => panic!("expected utilities list selection, got {other:?}"),
    }

    let weapons = sdk
        .faction()
        .weapons(FactionOptions::default())
        .await
        .expect("weapons should deserialize");
    match &weapons.weapons {
        FactionLegacyItemSelection::Map(entries) => {
            let item = entries.get("19").expect("weapon should exist");
            assert_eq!(item.item_type.as_deref(), Some("Primary"));
            assert_eq!(item.damage, Some(75.5));
        }
        other => panic!("expected weapons map selection, got {other:?}"),
    }

    let requests = transport.requests();
    assert_eq!(requests.len(), 12);

    let armor_request = requests
        .iter()
        .find(|request| request.query.get("selections") == Some(&"armor".to_string()))
        .expect("armor request should be captured");
    assert_eq!(armor_request.path, "/faction");

    let crime_request = requests
        .iter()
        .find(|request| request.query.get("selections") == Some(&"crime".to_string()))
        .expect("crime request should be captured");
    assert_eq!(crime_request.path, "/faction");
    assert_eq!(crime_request.query.get("id"), Some(&"1186857".to_string()));

    let crimes_request = requests
        .iter()
        .find(|request| request.query.get("selections") == Some(&"crimes".to_string()))
        .expect("crimes request should be captured");
    assert_eq!(crimes_request.path, "/faction");
    assert_eq!(crimes_request.query.get("limit"), Some(&"1".to_string()));
    assert!(!crimes_request.query.contains_key("id"));
}

#[tokio::test]
async fn faction_typed_history_and_report_surfaces_deserialize() {
    let (sdk, transport) = make_sdk_with_responses(vec![
        Ok(TransportResponse {
            status: 200,
            body: r#"{"attacks":[{"id":1,"code":"abc","started":1710000000,"ended":1710000005,"attacker":{"id":10,"name":"Attacker","level":50,"faction":{"id":16312,"name":"39th Street Killers X"}},"defender":{"id":20,"name":"Defender","level":45,"faction":{"id":9,"name":"Target Faction"}},"result":"Hospitalized","respect_gain":1.23,"respect_loss":0.45,"chain":10,"is_interrupted":false,"is_stealthed":false,"is_raid":false,"is_ranked_war":true,"modifiers":{"fair_fight":1.1,"war":1,"chain":1.5},"finishing_hit_effects":[{"name":"Bleed","value":5}]}],"_metadata":{"links":{"prev":null,"next":"https://api.torn.com/v2/faction/attacks?from=1710000000"}}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"attacks":[{"id":2,"code":"def","started":1710000010,"ended":1710000015,"attacker":{"id":11,"faction_id":16312},"defender":{"id":21,"faction_id":8},"result":"Mugged","respect_gain":0.5,"respect_loss":0.1}],"_metadata":{"links":{"prev":null,"next":null}}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"reports":[{"type":"spy","target_id":1,"reporter_id":2,"faction_id":16312,"timestamp":1710000100,"report":{"strength":1000,"speed":900}}],"_metadata":{"links":{"prev":null,"next":null}}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"revives":[{"id":3,"reviver":{"id":12,"name":"Medic","faction":{"id":16312,"name":"39th Street Killers X"},"skill":88.5},"target":{"id":22,"name":"Target","faction":{"id":7,"name":"Other"},"hospital_reason":"Mugged","early_discharge":false,"last_action":{"status":"Offline","relative":"1 minute","timestamp":1710000000},"online_status":"Offline"},"success_chance":95.5,"result":"success","timestamp":1710000200}],"_metadata":{"links":{"prev":null,"next":null}}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"revives":[{"id":4,"reviver":{"id":13,"faction_id":16312},"target":{"id":23,"faction_id":6,"hospital_reason":"Attacked","early_discharge":true,"last_action":1710000000,"online_status":"Online"},"success_chance":90.0,"result":"fail","timestamp":1710000300}],"_metadata":{"links":{"prev":null,"next":null}}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"territorywars":[{"id":45231,"war_id":45231,"territory":"RYD","start":1777420000,"end":1777423000,"target":777,"result":"Won","factions":[{"id":16312,"name":"39th Street Killers X","score":100,"is_aggressor":true},{"id":777,"name":"Target Faction","score":55,"is_aggressor":false}]}],"_metadata":{"links":{"prev":null,"next":null}}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"raidreport":{"id":7382,"start":1757164550,"end":1757168483,"aggressor":{"id":16312,"name":"39th Street Killers X","score":0,"attackers":[{"user":{"id":1,"name":"One"},"attacks":4,"damage":1234}],"non_attackers":[{"id":2,"name":"Two"}]},"defender":{"id":54858,"name":"39th Street Creepers","score":0,"attackers":[],"non_attackers":[]}}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"rankedwarreport":{"id":40884,"start":1777421000,"end":1777429000,"winner":16312,"forfeit":false,"factions":[{"id":16312,"name":"39th Street Killers X","score":1000,"attacks":50,"rank":{"before":"Diamond II","after":"Diamond I"},"rewards":{"respect":12.5,"points":100,"items":[{"id":1,"name":"Point","quantity":5}]},"members":[{"id":1,"name":"One","level":100,"score":500,"attacks":25}]}]}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"territorywarreport":{"id":45231,"territory":"RYD","started_at":1777420000,"ended_at":1777423000,"winner":16312,"result":"Won","factions":[{"id":16312,"name":"39th Street Killers X","score":100,"is_aggressor":true,"clears":3,"joins":5,"members":[{"id":1,"username":"One","level":100,"score":55,"joins":2,"clears":1}]}]}}"#.to_string(),
        }),
    ]);

    let attacks = sdk
        .faction()
        .attacks(FactionOptions::default().with_base(BaseOptions::default().with_limit(1)))
        .await
        .expect("attacks should deserialize");
    assert_eq!(attacks.attacks[0].chain, Some(10));
    assert!(attacks._metadata.is_some());

    let attacksfull = sdk
        .faction()
        .attacksfull(FactionOptions::default())
        .await
        .expect("attacksfull should deserialize");
    assert_eq!(attacksfull.attacks[0].id, Some(2));

    let reports = sdk
        .faction()
        .reports(FactionOptions::default())
        .await
        .expect("reports should deserialize");
    assert_eq!(reports.reports[0].report_type.as_deref(), Some("spy"));

    let revives = sdk
        .faction()
        .revives(FactionOptions::default())
        .await
        .expect("revives should deserialize");
    assert_eq!(revives.revives[0].id, Some(3));

    let revivesfull = sdk
        .faction()
        .revivesfull(FactionOptions::default())
        .await
        .expect("revivesfull should deserialize");
    assert_eq!(revivesfull.revives[0].id, Some(4));

    let territorywars = sdk
        .faction()
        .territorywars(FactionOptions::default())
        .await
        .expect("territorywars should deserialize");
    assert_eq!(territorywars.territorywars[0].war_id, Some(45231));
    assert_eq!(territorywars.territorywars[0].factions.len(), 2);

    let raidreport = sdk
        .faction()
        .raidreport(FactionOptions::default().with_raid_war_id("7382"))
        .await
        .expect("raidreport should deserialize");
    assert_eq!(raidreport.raidreport.id, Some(7382));
    assert_eq!(
        raidreport
            .raidreport
            .aggressor
            .as_ref()
            .map(|side| side.attackers.len()),
        Some(1)
    );

    let rankedwarreport = sdk
        .faction()
        .rankedwarreport(FactionOptions::default().with_ranked_war_id("40884"))
        .await
        .expect("rankedwarreport should deserialize");
    assert_eq!(rankedwarreport.rankedwarreport.id, Some(40884));
    assert_eq!(rankedwarreport.rankedwarreport.factions.len(), 1);

    let territorywarreport = sdk
        .faction()
        .territorywarreport(FactionOptions::default().with_territory_war_id("45231"))
        .await
        .expect("territorywarreport should deserialize");
    assert_eq!(territorywarreport.territorywarreport.id, Some(45231));
    assert_eq!(
        territorywarreport.territorywarreport.factions[0].members[0]
            .username
            .as_deref(),
        Some("One")
    );

    let requests = transport.requests();
    assert_eq!(requests.len(), 9);

    let territorywars_request = requests
        .iter()
        .find(|request| request.query.get("selections") == Some(&"territorywars".to_string()))
        .expect("territorywars request should be captured");
    assert!(!territorywars_request.query.contains_key("id"));

    let raidreport_request = requests
        .iter()
        .find(|request| request.query.get("selections") == Some(&"raidreport".to_string()))
        .expect("raidreport request should be captured");
    assert_eq!(
        raidreport_request.query.get("id"),
        Some(&"7382".to_string())
    );

    let rankedwarreport_request = requests
        .iter()
        .find(|request| request.query.get("selections") == Some(&"rankedwarreport".to_string()))
        .expect("rankedwarreport request should be captured");
    assert_eq!(
        rankedwarreport_request.query.get("id"),
        Some(&"40884".to_string())
    );

    let territorywarreport_request = requests
        .iter()
        .find(|request| request.query.get("selections") == Some(&"territorywarreport".to_string()))
        .expect("territorywarreport request should be captured");
    assert_eq!(
        territorywarreport_request.query.get("id"),
        Some(&"45231".to_string())
    );
}

#[tokio::test]
async fn faction_generic_route_typed_helpers_allow_idless_owned_surfaces() {
    let (sdk, _) = make_sdk_with_responses(vec![
        Ok(TransportResponse {
            status: 200,
            body: r#"{"chain":{"id":0,"current":0,"max":10,"timeout":0,"modifier":1,"cooldown":0,"start":0,"end":1777422590}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"chains":[{"id":62264214,"chain":19,"respect":53,"start":1777416675,"end":1777417656}],"_metadata":{"links":{"prev":null,"next":null}}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"chainreport":{"id":62264214,"faction_id":16312,"start":1777416675,"end":1777417656,"details":{"chain":19,"respect":53,"members":2,"targets":19,"war":0,"best":10,"leave":17,"mug":2,"hospitalize":0,"assists":0,"retaliations":0,"overseas":0,"draws":0,"escapes":2,"losses":0},"bonuses":[],"attackers":[],"non_attackers":[]}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"raids":[{"id":7382,"start":1757164550,"end":1757168483,"aggressor":{"id":16312,"name":"39th Street Killers X","score":0},"defender":{"id":54858,"name":"39th Street Creepers","score":0}}],"_metadata":{"links":{"prev":null,"next":null}}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"territoryOwnership":[{"id":"RYD","owned_by":16312,"acquired_at":1773093363}],"_metadata":{"links":{"prev":null,"next":null}}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"territory":[{"id":"RYD","acquired_at":1773093363,"sector":6,"size":4,"density":2,"slots":26,"respect":118,"coordinates":{"x":4158.11,"y":1581},"racket":null}]}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"territorywars":[{"id":45231,"war_id":45231,"territory":"RYD","start":1777420000,"end":1777423000,"target":777,"result":"Won","factions":[{"id":16312,"name":"39th Street Killers X","score":100,"is_aggressor":true},{"id":777,"name":"Target Faction","score":55,"is_aggressor":false}]}],"_metadata":{"links":{"prev":null,"next":null}}}"#.to_string(),
        }),
    ]);

    sdk.faction()
        .chain(FactionOptions::default())
        .await
        .expect("chain should support owned-faction generic route");
    sdk.faction()
        .chains(FactionOptions::default())
        .await
        .expect("chains should support owned-faction generic route");
    sdk.faction()
        .chainreport(FactionOptions::default())
        .await
        .expect("chainreport should support owned-faction generic route");
    sdk.faction()
        .raids(FactionOptions::default())
        .await
        .expect("raids should support owned-faction generic route");
    sdk.faction()
        .territoryownership(FactionOptions::default())
        .await
        .expect("territoryownership should support owned-faction generic route");
    sdk.faction()
        .territory(FactionOptions::default())
        .await
        .expect("territory should support owned-faction generic route");
    sdk.faction()
        .territorywars(FactionOptions::default())
        .await
        .expect("territorywars should support owned-faction generic route");
}
