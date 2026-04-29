mod support;

use std::path::PathBuf;
use std::time::Duration;

use support::MockTransport;
use torn_sdk_planner::models::manual::user::{
    UserCompetition, UserEmployment, UserOrganizedCrimeSelection, UserPersonalStatsSelection,
    UserReviveLastAction,
};
use torn_sdk_planner::{
    BaseOptions, CapabilitiesDocument, ExecutorConfig, RateLimitConfig, RequestExecutor,
    RequestPlanner, TornClient, TornSdk, TransportError, TransportResponse, UserOptions,
};

fn test_config() -> ExecutorConfig {
    ExecutorConfig {
        base_url_v2: "https://api.torn.com/v2".to_string(),
        base_url_v1: "https://api.torn.com".to_string(),
        timeout: Duration::from_secs(5),
        user_agent: "user-typed-contract-test".to_string(),
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
async fn typed_user_methods_cover_stable_low_risk_surfaces() {
    let (sdk, transport) = make_sdk_with_responses(vec![
        Ok(TransportResponse {
            status: 200,
            body: r#"{"timestamp":1710000000}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"notifications":{"messages":1,"events":2,"awards":3,"competition":4}}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"icons":[{"id":6,"title":"Male","description":null,"until":null}]}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"enlistedcars":[{"id":1016331,"car_item_id":77,"car_item_name":"Tabata RM2","car_name":"Roadster","top_speed":30,"acceleration":27,"braking":18,"handling":23,"safety":20,"dirt":15,"tarmac":20,"class":"D","worth":57000,"points_spent":15,"races_entered":42,"races_won":12,"is_removed":true,"parts":[1,35]}]}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"job":{"type":"company","id":92041,"type_id":38,"name":"Black Salts","rating":10,"position":"Director","days_in_company":117}}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"jobpoints":{"jobs":{"army":0,"casino":1,"education":2,"grocer":3,"law":4,"medical":5},"companies":[{"company":{"id":38,"name":"Mining Corporation"},"points":235}]}}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"jobranks":{"army":"Corporal","grocer":"Bagboy","casino":"Dealer","medical":"Medical Student","law":"Law Student","education":"Principal"}}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"refills":{"energy":true,"nerve":false,"token":true,"special_count":7}}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"skills":[{"slug":"racing","name":"Racing","level":32.43}]}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"stocks":[{"id":9,"shares":100000,"transactions":[{"id":17950269,"shares":100000,"price":323.94,"timestamp":1747624406}],"bonus":{"available":false,"increment":1,"progress":3,"frequency":31}}]}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"calendar":{"start_time":"13:00 TCT"}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"selections":["profile","virus","organizedcrimes"]}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"factionBalance":{"money":5000,"points":10}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"education":{"complete":[1,2,3],"current":{"id":50,"until":1777949732}}}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"competition":{"name":"Rock, Paper, Scissors","status":"paper","hp":{"current":179,"maximum":269}}}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"weaponexp":[{"id":1,"name":"Armalite M-15A4 Rifle","exp":100}]}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"racingrecords":[{"track":{"id":21,"name":"Speedway"},"records":[{"car_id":522,"car_name":"Veloria LFA","lap_time":2656}]}]}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"honors":[{"id":1,"timestamp":1748501303}]}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"medals":[{"id":3,"timestamp":1748501304}]}"#.to_string(),
        }),
    ]);

    let timestamp = sdk
        .user()
        .timestamp(UserOptions::default())
        .await
        .expect("user timestamp should deserialize");
    assert_eq!(timestamp.timestamp, Some(1710000000));

    let notifications = sdk
        .user()
        .notifications(UserOptions::default())
        .await
        .expect("user notifications should deserialize");
    assert_eq!(notifications.notifications.awards, Some(3));

    let icons = sdk
        .user()
        .icons(UserOptions::default())
        .await
        .expect("user icons should deserialize");
    assert_eq!(icons.icons.len(), 1);
    assert_eq!(icons.icons[0].description, None);

    let enlistedcars = sdk
        .user()
        .enlistedcars(UserOptions::default())
        .await
        .expect("user enlisted cars should deserialize");
    assert_eq!(
        enlistedcars.enlistedcars[0].car_name.as_deref(),
        Some("Roadster")
    );
    assert_eq!(enlistedcars.enlistedcars[0].parts, vec![1, 35]);

    let job = sdk
        .user()
        .job(UserOptions::default())
        .await
        .expect("user job should deserialize");
    match job.job.as_ref() {
        Some(UserEmployment::Company(company)) => {
            assert_eq!(company.type_id, Some(38));
            assert_eq!(company.position.as_deref(), Some("Director"));
        }
        other => panic!("unexpected job payload: {other:?}"),
    }

    let jobpoints = sdk
        .user()
        .jobpoints(UserOptions::default())
        .await
        .expect("user jobpoints should deserialize");
    assert_eq!(
        jobpoints
            .jobpoints
            .jobs
            .as_ref()
            .and_then(|jobs| jobs.medical),
        Some(5)
    );
    assert_eq!(jobpoints.jobpoints.companies.len(), 1);

    let jobranks = sdk
        .user()
        .jobranks(UserOptions::default())
        .await
        .expect("user jobranks should deserialize");
    assert_eq!(jobranks.jobranks.education.as_deref(), Some("Principal"));

    let refills = sdk
        .user()
        .refills(UserOptions::default())
        .await
        .expect("user refills should deserialize");
    assert_eq!(refills.refills.special_count, Some(7));

    let skills = sdk
        .user()
        .skills(UserOptions::default())
        .await
        .expect("user skills should deserialize");
    assert_eq!(skills.skills[0].level, Some(32.43));

    let stocks = sdk
        .user()
        .stocks(UserOptions::default())
        .await
        .expect("user stocks should deserialize");
    assert_eq!(stocks.stocks[0].transactions[0].price, Some(323.94));

    let calendar = sdk
        .user()
        .calendar(UserOptions::default())
        .await
        .expect("user calendar should deserialize");
    assert_eq!(calendar.calendar.start_time.as_deref(), Some("13:00 TCT"));

    let lookup = sdk
        .user()
        .lookup(UserOptions::default())
        .await
        .expect("user lookup should deserialize");
    assert!(
        lookup
            .selections
            .iter()
            .any(|selection| selection == "virus")
    );
    assert!(
        lookup
            .selections
            .iter()
            .any(|selection| selection == "organizedcrimes")
    );

    let factionbalance = sdk
        .user()
        .factionbalance(UserOptions::default())
        .await
        .expect("user factionbalance should deserialize");
    assert_eq!(
        factionbalance
            .faction_balance
            .as_ref()
            .and_then(|balance| balance.money),
        Some(5000)
    );

    let education = sdk
        .user()
        .education(UserOptions::default())
        .await
        .expect("user education should deserialize");
    assert_eq!(education.education.complete, vec![1, 2, 3]);
    assert_eq!(
        education
            .education
            .current
            .as_ref()
            .and_then(|current| current.id),
        Some(50)
    );

    let competition = sdk
        .user()
        .competition(UserOptions::default())
        .await
        .expect("user competition should deserialize");
    match competition.competition.as_ref() {
        Some(UserCompetition::RockPaperScissors(rps)) => {
            assert_eq!(rps.status.as_deref(), Some("paper"));
            assert_eq!(rps.hp.as_ref().and_then(|hp| hp.maximum), Some(269));
        }
        other => panic!("unexpected competition payload: {other:?}"),
    }

    let weaponexp = sdk
        .user()
        .weaponexp(UserOptions::default())
        .await
        .expect("user weaponexp should deserialize");
    assert_eq!(weaponexp.weaponexp[0].exp, Some(100));

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
        Some(21)
    );
    assert_eq!(
        racingrecords.racingrecords[0].records[0]
            .car_name
            .as_deref(),
        Some("Veloria LFA")
    );

    let honors = sdk
        .user()
        .honors(UserOptions::default())
        .await
        .expect("user honors should deserialize");
    assert_eq!(honors.honors[0].id, Some(1));

    let medals = sdk
        .user()
        .medals(UserOptions::default())
        .await
        .expect("user medals should deserialize");
    assert_eq!(medals.medals[0].timestamp, Some(1748501304));

    let requests = transport.requests();
    assert_eq!(requests.len(), 19);
    assert!(!requests[2].query.contains_key("id"));
    assert!(!requests[4].query.contains_key("id"));
}

#[tokio::test]
async fn typed_user_account_state_slice_deserializes() {
    let (sdk, transport) = make_sdk_with_responses(vec![
        Ok(TransportResponse {
            status: 200,
            body: r#"{"ammo":[{"id":2,"name":"9mm Parabellum Round","types":[{"name":"Standard","quantity":8557,"equipped":false},{"name":"Tracer","quantity":1000,"equipped":true}]}]}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"equipment":[{"id":399,"name":"ArmaLite M-15A4","uid":18528121925,"type":"Weapon","sub_type":"Rifle","stats":{"damage":84.26,"accuracy":68.85,"armor":null,"quality":281.07},"bonuses":[{"id":72,"title":"Assassinate","description":"116% increased damage on the first turn","value":116}],"rarity":"red","slot":1},{"id":676,"name":"Marauder Body","uid":17279982078,"type":"Armor","sub_type":null,"stats":{"damage":null,"accuracy":null,"armor":52.65,"quality":12.98},"bonuses":[{"id":97,"title":"Imperviable","description":"10% increased life","value":10}],"rarity":"orange","slot":4}],"clothing":[{"id":1129,"name":"Cat Ears","uid":18387458439,"type":"Clothing"},{"id":1465,"name":"Cluster Ring","uid":15777493825,"type":"Jewelry"}]}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"missions":{"credits":1086,"givers":[{"id":4,"name":"Duke","contracts":[{"title":"Out of the Frying Pan","difficulty":"Hard","status":"Completed","created_at":1776643243,"started_at":1776643249,"expires_at":null,"completed_at":1776852865,"rewards":{"money":28000,"credits":27}},{"title":"Birthday Surprise","difficulty":"Expert","status":"Accepted","created_at":1777334461,"started_at":1777334465,"expires_at":1777593665,"completed_at":null,"rewards":null}]}],"rewards":[{"type":"Ammo","details":{"id":5,"name":"5.56mm Rifle Round","type":"Hollow Point"},"amount":500,"cost":73,"expires_at":1777347345},{"type":"Item","details":{"id":68,"name":"Small First Aid Kit","type":"Medical","sub_type":null},"amount":50,"cost":26,"expires_at":1777343626},{"type":"Upgrade","details":{"id":14,"name":"Extra Magazine"},"amount":1,"cost":374,"expires_at":1777168501}]}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"battlestats":{"strength":{"value":267340633,"modifier":-5,"modifiers":[{"effect":"+20% to Strength from Faction","value":20,"type":"Faction"},{"effect":"-25% to Strength from Drug","value":-25,"type":"Drug"}]},"defense":{"value":350191493,"modifier":-25,"modifiers":[{"effect":"-25% to Defense from Drug","value":-25,"type":"Drug"}]},"speed":{"value":275316330,"modifier":-5,"modifiers":[{"effect":"+20% to Speed from Faction","value":20,"type":"Faction"}]},"dexterity":{"value":1503806,"modifier":-25,"modifiers":[{"effect":"-25% to Dexterity from Drug","value":-25,"type":"Drug"}]},"total":894352262}}"#.to_string(),
        }),
    ]);

    let ammo = sdk
        .user()
        .ammo(UserOptions::default())
        .await
        .expect("user ammo should deserialize");
    assert_eq!(ammo.ammo[0].id, Some(2));
    assert_eq!(ammo.ammo[0].types[1].equipped, Some(true));
    assert_eq!(ammo.ammo[0].types[0].quantity, Some(8557));

    let equipment = sdk
        .user()
        .equipment(UserOptions::default())
        .await
        .expect("user equipment should deserialize");
    assert_eq!(equipment.equipment.len(), 2);
    assert_eq!(equipment.clothing.len(), 2);
    assert_eq!(
        equipment.equipment[0]
            .stats
            .as_ref()
            .and_then(|stats| stats.damage),
        Some(84.26)
    );
    assert_eq!(
        equipment.equipment[1]
            .stats
            .as_ref()
            .and_then(|stats| stats.armor),
        Some(52.65)
    );
    assert_eq!(
        equipment.equipment[0].bonuses[0].title.as_deref(),
        Some("Assassinate")
    );
    assert_eq!(equipment.clothing[1].item_type.as_deref(), Some("Jewelry"));

    let missions = sdk
        .user()
        .missions(UserOptions::default())
        .await
        .expect("user missions should deserialize");
    assert_eq!(missions.missions.credits, Some(1086));
    assert_eq!(missions.missions.givers[0].contracts.len(), 2);
    assert_eq!(
        missions.missions.givers[0].contracts[0]
            .rewards
            .as_ref()
            .and_then(|rewards| rewards.credits),
        Some(27)
    );
    assert!(missions.missions.givers[0].contracts[1].rewards.is_none());
    assert_eq!(
        missions.missions.rewards[0]
            .details
            .as_ref()
            .and_then(|details| details.detail_type.as_deref()),
        Some("Hollow Point")
    );
    assert_eq!(
        missions.missions.rewards[1]
            .details
            .as_ref()
            .and_then(|details| details.sub_type.as_deref()),
        None
    );
    assert_eq!(
        missions.missions.rewards[2].reward_type.as_deref(),
        Some("Upgrade")
    );

    let battlestats = sdk
        .user()
        .battlestats(UserOptions::default())
        .await
        .expect("user battlestats should deserialize");
    assert_eq!(battlestats.battlestats.total, Some(894352262));
    assert_eq!(
        battlestats
            .battlestats
            .strength
            .as_ref()
            .and_then(|stat| stat.modifier),
        Some(-5.0)
    );
    assert_eq!(
        battlestats
            .battlestats
            .strength
            .as_ref()
            .map(|stat| stat.modifiers.len()),
        Some(2)
    );
    assert_eq!(
        battlestats
            .battlestats
            .defense
            .as_ref()
            .and_then(|stat| stat.modifiers.first())
            .and_then(|modifier| modifier.modifier_type.as_deref()),
        Some("Drug")
    );

    let requests = transport.requests();
    assert_eq!(requests.len(), 4);
    assert!(!requests[0].query.contains_key("id"));
    assert!(!requests[1].query.contains_key("id"));
    assert!(!requests[2].query.contains_key("id"));
    assert!(!requests[3].query.contains_key("id"));
}

#[tokio::test]
async fn typed_user_unions_preserve_unknown_variants() {
    let (sdk, _) = make_sdk_with_responses(vec![
        Ok(TransportResponse {
            status: 200,
            body: r#"{"job":{"type":"guild","rank":"Architect"}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"competition":{"name":"Snowball Fight","score":99,"team":"blue"}}"#
                .to_string(),
        }),
    ]);

    let job = sdk
        .user()
        .job(UserOptions::default())
        .await
        .expect("unknown user job should deserialize");
    match job.job.as_ref() {
        Some(UserEmployment::Unknown(unknown)) => {
            assert_eq!(unknown.job_type.as_deref(), Some("guild"));
            assert_eq!(unknown.extra.get("rank"), Some(&"Architect".into()));
        }
        other => panic!("unexpected unknown job payload: {other:?}"),
    }

    let competition = sdk
        .user()
        .competition(UserOptions::default())
        .await
        .expect("unknown user competition should deserialize");
    match competition.competition.as_ref() {
        Some(UserCompetition::Unknown(unknown)) => {
            assert_eq!(unknown.name.as_deref(), Some("Snowball Fight"));
            assert_eq!(unknown.extra.get("score"), Some(&99.into()));
        }
        other => panic!("unexpected unknown competition payload: {other:?}"),
    }
}

#[tokio::test]
async fn typed_user_stable_activity_and_property_surfaces_deserialize() {
    let (sdk, transport) = make_sdk_with_responses(vec![
        Ok(TransportResponse {
            status: 200,
            body: r#"{"events":[{"id":"evt-1","timestamp":1710001000,"event":"You found a cache."}],"_metadata":{"links":{"next":null,"prev":null}}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"events":[{"id":"evt-2","timestamp":1710001001,"event":"Fresh event"}]}"#
                .to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"messages":[{"id":278872269,"sender":{"id":1268021,"name":"Toxin"},"timestamp":1777372065,"topic":"Race notice","type":"Faction newsletter","seen":true,"read":false}],"_metadata":{"links":{"next":null,"prev":null}}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"messages":[{"id":278872270,"sender":{"id":1,"name":"Admin"},"timestamp":1777372070,"topic":"Fresh mail","type":"Company newsletter","seen":false,"read":false}]}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"races":[{"id":18895661,"title":"Docks","track_id":10,"creator_id":3637232,"status":"finished","laps":100,"participants":{"minimum":6,"maximum":100,"current":6},"schedule":{"join_from":1777385648,"join_until":1777385648,"start":1777386006,"end":1777403871},"requirements":{"driver_class":null,"car_class":null,"car_item_id":null,"requires_stock_car":false,"requires_password":false,"join_fee":0},"is_official":false,"skill_gain":0.0323,"results":[{"driver_id":511670,"position":1,"car_id":434034,"car_item_id":85,"car_item_name":"Volt GT","car_class":"A","has_crashed":false,"best_lap_time":170.37,"race_time":17851.58,"time_ended":1777403858}]}],"_metadata":{"links":{"next":null,"prev":null}}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"hof":{"attacks":{"value":3199,"rank":32895},"racing_skill":{"value":32.43,"rank":5548},"battle_stats":{"value":892850347,"rank":23754},"working_stats":{"value":188258,"rank":139905}}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"bounties_timestamp":1777422599,"bounties_delay":30,"bounties":[{"target_id":123,"target_name":"Target","target_level":50,"lister_id":456,"lister_name":"Poster","reward":500000,"reason":"Testing","quantity":2,"is_anonymous":false,"valid_until":1778000000}]}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"properties":[{"id":1858770,"owner":{"id":2057854,"name":"Flamer"},"property":{"id":13,"name":"Private Island"},"happy":3725,"upkeep":{"property":100000,"staff":0},"market_price":1057788000,"modifications":["Hot Tub"],"staff":[],"status":"rented","used_by":[],"cost":40000000,"cost_per_day":400000,"rental_period":100,"rental_period_remaining":14,"rented_by":{"id":4057573,"name":"Muthalali"},"lease_extension":null}],"_metadata":{"links":{"next":null,"prev":null}}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"property":{"id":2704636,"owner":{"id":3637232,"name":"Swervelord"},"property":{"id":13,"name":"Private Island"},"happy":5025,"upkeep":{"property":100000,"staff":252500},"market_price":1952788000,"modifications":["Hot Tub","Airstrip"],"staff":[{"type":"Maid","amount":4}],"status":"in_use","used_by":[{"id":2057854,"name":"Flamer"},{"id":3637232,"name":"Swervelord"}]}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"forumThreads":[{"id":16552320,"title":"Selling items","forum_id":10,"posts":206,"rating":7,"views":635,"author":{"id":3637232,"username":"Swervelord","karma":476},"last_poster":{"id":3637232,"username":"Swervelord","karma":476},"first_post_time":1775106615,"last_post_time":1777422128,"has_poll":false,"is_locked":false,"is_sticky":false,"new_posts":0}],"_metadata":{"links":{"next":null,"prev":null}}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"forumPosts":[{"id":27286639,"thread_id":16552320,"author":{"id":3637232,"username":"Swervelord","karma":476},"is_legacy":false,"is_topic":false,"is_edited":false,"is_pinned":false,"created_time":1777422128,"edited_by":null,"has_quote":false,"quoted_post_id":null,"content":"b","likes":0,"dislikes":0}],"_metadata":{"links":{"next":null,"prev":null}}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"forumSubscribedThreads":[{"id":15907925,"forum_id":46,"title":"Looking for Work","posts":{"new":1550,"total":97105},"author":{"id":860829,"username":"Magikarp","karma":1824}}]}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"forumFriends":[{"thread_id":16531657,"post_id":27283828,"title":"RW weapons","timestamp":1777366393,"text":"-Josh- posted on a thread","type":1,"is_seen":false,"user":{"id":3954449,"username":"-Josh-","karma":33}}]}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"forumFeed":[{"thread_id":16480171,"post_id":26104826,"title":"Scripts","timestamp":1777208516,"text":"ANXIETY_ liked your thread","type":3,"is_seen":true,"user":{"id":3853621,"username":"ANXIETY_","karma":44}}]}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"workstats":{"endurance":68987,"intelligence":109030,"manual_labor":10241,"total":188258}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"merits":{"upgrades":[{"id":3,"level":10}],"available":3,"used":400,"medals":168,"honors":235}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"itemmarket":[{"id":69077780,"price":51000000000,"average_price":553,"amount":1,"is_anonymous":false,"available":0,"item":{"id":487,"name":"Thompson","type":"Primary","uid":9300424431,"stats":{"damage":53.49,"accuracy":54.71,"armor":null,"quality":261.97},"rarity":"red","bonuses":[{"id":0,"title":"Revitalize","description":"19% chance","value":19}]}}],"_metadata":{"links":{"next":null,"prev":null}}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"organizedCrime":{"id":1567682,"previous_crime_id":null,"name":"Blast from the Past","difficulty":7,"status":"Planning","created_at":1777161335,"planning_at":1777308831,"executed_at":null,"ready_at":1777827231,"expired_at":1777766135,"slots":[{"position":"Picklock","position_info":{"id":"P1","label":"Picklock #1","number":1},"position_id":"P1","position_number":1,"item_requirement":null,"user":{"outcome":null,"id":3449728,"joined_at":1777379739,"progress":0,"item_outcome":null},"checkpoint_pass_rate":73}],"rewards":null}}"#.to_string(),
        }),
    ]);

    let events = sdk
        .user()
        .events(UserOptions::default())
        .await
        .expect("user events should deserialize");
    assert_eq!(events.events[0].id.as_deref(), Some("evt-1"));
    assert!(events._metadata.is_some());

    let newevents = sdk
        .user()
        .newevents(UserOptions::default())
        .await
        .expect("user newevents should deserialize");
    assert_eq!(newevents.events[0].text.as_deref(), Some("Fresh event"));

    let messages = sdk
        .user()
        .messages(UserOptions::default())
        .await
        .expect("user messages should deserialize");
    assert_eq!(
        messages.messages[0].message_type.as_deref(),
        Some("Faction newsletter")
    );
    assert!(messages._metadata.is_some());

    let newmessages = sdk
        .user()
        .newmessages(UserOptions::default())
        .await
        .expect("user newmessages should deserialize");
    assert_eq!(
        newmessages.messages[0]
            .sender
            .as_ref()
            .and_then(|sender| sender.name.as_deref()),
        Some("Admin")
    );

    let races = sdk
        .user()
        .races(UserOptions::default())
        .await
        .expect("user races should deserialize");
    assert_eq!(races.races[0].skill_gain, Some(0.0323));
    assert_eq!(races.races[0].results[0].best_lap_time, Some(170.37));

    let hof = sdk
        .user()
        .hof("3637232")
        .await
        .expect("user hof should deserialize");
    assert_eq!(
        hof.hof.racing_skill.as_ref().and_then(|value| value.value),
        Some(32.43)
    );
    assert_eq!(
        hof.hof.working_stats.as_ref().and_then(|value| value.rank),
        Some(139905)
    );

    let bounties = sdk
        .user()
        .bounties("3637232")
        .await
        .expect("user bounties should deserialize");
    assert_eq!(bounties.bounties_delay, Some(30));
    assert_eq!(bounties.bounties[0].reward, Some(500000));

    let properties = sdk
        .user()
        .properties("3637232")
        .await
        .expect("user properties should deserialize");
    assert_eq!(properties.properties[0].status.as_deref(), Some("rented"));
    assert_eq!(
        properties.properties[0]
            .rented_by
            .as_ref()
            .and_then(|user| user.name.as_deref()),
        Some("Muthalali")
    );

    let property = sdk
        .user()
        .property("3637232")
        .await
        .expect("user property should deserialize");
    assert_eq!(property.property.used_by.len(), 2);
    assert_eq!(
        property.property.staff[0].staff_type.as_deref(),
        Some("Maid")
    );

    let forumthreads = sdk
        .user()
        .forumthreads("3637232")
        .await
        .expect("user forumthreads should deserialize");
    assert_eq!(
        forumthreads.forum_threads[0]
            .author
            .as_ref()
            .and_then(|user| user.username.as_deref()),
        Some("Swervelord")
    );

    let forumposts = sdk
        .user()
        .forumposts("3637232")
        .await
        .expect("user forumposts should deserialize");
    assert_eq!(forumposts.forum_posts[0].content.as_deref(), Some("b"));

    let subscribed = sdk
        .user()
        .forumsubscribedthreads(UserOptions::default())
        .await
        .expect("user forumsubscribedthreads should deserialize");
    assert_eq!(
        subscribed.forum_subscribed_threads[0]
            .posts
            .as_ref()
            .and_then(|posts| posts.new_posts),
        Some(1550)
    );

    let forumfriends = sdk
        .user()
        .forumfriends(UserOptions::default())
        .await
        .expect("user forumfriends should deserialize");
    assert_eq!(forumfriends.forum_friends[0].entry_type, Some(1));

    let forumfeed = sdk
        .user()
        .forumfeed(UserOptions::default())
        .await
        .expect("user forumfeed should deserialize");
    assert_eq!(
        forumfeed.forum_feed[0]
            .user
            .as_ref()
            .and_then(|user| user.username.as_deref()),
        Some("ANXIETY_")
    );

    let workstats = sdk
        .user()
        .workstats(UserOptions::default())
        .await
        .expect("user workstats should deserialize");
    assert_eq!(workstats.workstats.total, Some(188258));

    let merits = sdk
        .user()
        .merits(UserOptions::default())
        .await
        .expect("user merits should deserialize");
    assert_eq!(merits.merits.upgrades[0].level, Some(10));

    let itemmarket = sdk
        .user()
        .itemmarket(UserOptions::default())
        .await
        .expect("user itemmarket should deserialize");
    assert_eq!(
        itemmarket.itemmarket[0]
            .item
            .as_ref()
            .and_then(|item| item.uid),
        Some(9300424431)
    );
    assert_eq!(
        itemmarket.itemmarket[0]
            .item
            .as_ref()
            .map(|item| item.bonuses.len()),
        Some(1)
    );

    let organizedcrime = sdk
        .user()
        .organizedcrime(UserOptions::default())
        .await
        .expect("user organizedcrime should deserialize");
    match organizedcrime.organized_crime.as_ref() {
        Some(UserOrganizedCrimeSelection::Crime(crime)) => {
            assert_eq!(crime.name.as_deref(), Some("Blast from the Past"));
            assert_eq!(crime.slots[0].checkpoint_pass_rate, Some(73));
        }
        other => panic!("unexpected organized crime payload: {other:?}"),
    }

    let requests = transport.requests();
    assert_eq!(requests.len(), 18);
    assert!(!requests[0].query.contains_key("id"));
    assert_eq!(requests[5].query.get("id"), Some(&"3637232".to_string()));
    assert_eq!(requests[7].query.get("id"), Some(&"3637232".to_string()));
    assert!(!requests[16].query.contains_key("id"));
}

#[tokio::test]
async fn typed_user_combat_history_surfaces_deserialize() {
    let (sdk, transport) = make_sdk_with_responses(vec![
        Ok(TransportResponse {
            status: 200,
            body: r#"{"attacks":[{"id":469638809,"code":"6f5b7edde64f1180770edf1edb80c127","started":1777425242,"ended":1777425248,"attacker":{"id":3637232,"name":"Swervelord","level":74,"faction":{"id":16312,"name":"39th Street Killers X"}},"defender":{"id":1546551,"name":"Piercyy","level":100,"faction":{"id":41234,"name":"Peaky CS"}},"result":"Hospitalize","respect_gain":4.21,"respect_loss":0,"chain":98,"is_interrupted":false,"is_stealthed":false,"is_raid":true,"is_ranked_war":false,"modifiers":{"fair_fight":2.06,"war":2,"retaliation":1,"group":1,"overseas":1,"chain":1.24,"warlord":1.32},"finishing_hit_effects":[{"name":"warlord","value":32}]}],"_metadata":{"links":{"prev":"https://api.torn.com/v2/user/attacks?&limit=1&sort=desc&to=1777425248","next":null}}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"attacks":[{"id":469638809,"code":"6f5b7edde64f1180770edf1edb80c127","started":1777425242,"ended":1777425248,"attacker":{"id":3637232,"faction_id":16312},"defender":{"id":1546551,"faction_id":41234},"result":"Assist","respect_gain":0,"respect_loss":0}],"_metadata":{"links":{"prev":"https://api.torn.com/v2/user/attacksfull?&limit=1&sort=desc&to=1777425248","next":null}}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"revives":[{"id":16836764,"reviver":{"id":2721059,"name":"Raid","faction":{"id":17133,"name":"Torn Medical"},"skill":49.31},"target":{"id":3637232,"name":"Swervelord","faction":{"id":16312,"name":"39th Street Killers X"},"hospital_reason":"Hospitalized by Oops","early_discharge":false,"last_action":1777421593,"online_status":"Online"},"success_chance":89,"result":"success","timestamp":1777421633}],"_metadata":{"links":{"prev":"https://api.torn.com/v2/user/revives?&limit=1&sort=desc&to=1777421633","next":null}}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"revives":[{"id":16836764,"reviver":{"id":2721059,"faction_id":17133},"target":{"id":3637232,"faction_id":16312,"hospital_reason":"Hospitalized by Oops","early_discharge":false,"last_action":1777421593,"online_status":"Online"},"success_chance":89,"result":"success","timestamp":1777421633}],"_metadata":{"links":{"prev":"https://api.torn.com/v2/user/revivesfull?&limit=1&sort=desc&to=1777421633","next":null}}}"#.to_string(),
        }),
    ]);

    let attacks = sdk
        .user()
        .attacks(UserOptions::default())
        .await
        .expect("user attacks should deserialize");
    assert_eq!(attacks.attacks[0].result.as_deref(), Some("Hospitalize"));
    assert_eq!(
        attacks.attacks[0]
            .attacker
            .as_ref()
            .and_then(|attacker| attacker.name.as_deref()),
        Some("Swervelord")
    );
    assert_eq!(
        attacks.attacks[0]
            .modifiers
            .as_ref()
            .and_then(|modifiers| modifiers.warlord),
        Some(1.32)
    );
    assert_eq!(
        attacks.attacks[0].finishing_hit_effects[0].name.as_deref(),
        Some("warlord")
    );
    assert!(attacks._metadata.is_some());

    let attacksfull = sdk
        .user()
        .attacksfull(UserOptions::default())
        .await
        .expect("user attacksfull should deserialize");
    assert_eq!(
        attacksfull.attacks[0]
            .attacker
            .as_ref()
            .and_then(|attacker| attacker.faction_id),
        Some(16312)
    );
    assert_eq!(attacksfull.attacks[0].respect_gain, Some(0.0));
    assert!(attacksfull._metadata.is_some());

    let revives = sdk
        .user()
        .revives(UserOptions::default())
        .await
        .expect("user revives should deserialize");
    assert_eq!(
        revives.revives[0]
            .reviver
            .as_ref()
            .and_then(|reviver| reviver.skill),
        Some(49.31)
    );
    match revives.revives[0]
        .target
        .as_ref()
        .and_then(|target| target.last_action.as_ref())
    {
        Some(UserReviveLastAction::Timestamp(timestamp)) => {
            assert_eq!(*timestamp, 1777421593);
        }
        other => panic!("unexpected revive target last_action: {other:?}"),
    }

    let revivesfull = sdk
        .user()
        .revivesfull(UserOptions::default())
        .await
        .expect("user revivesfull should deserialize");
    assert_eq!(
        revivesfull.revives[0]
            .target
            .as_ref()
            .and_then(|target| target.faction_id),
        Some(16312)
    );
    match revivesfull.revives[0]
        .target
        .as_ref()
        .and_then(|target| target.last_action.as_ref())
    {
        Some(UserReviveLastAction::Timestamp(timestamp)) => {
            assert_eq!(*timestamp, 1777421593);
        }
        other => panic!("unexpected revivefull target last_action: {other:?}"),
    }
    assert!(revivesfull._metadata.is_some());

    let requests = transport.requests();
    assert_eq!(requests.len(), 4);
    assert!(
        requests
            .iter()
            .all(|request| !request.query.contains_key("id"))
    );
}

#[tokio::test]
async fn typed_user_remaining_long_tail_surfaces_deserialize() {
    let (sdk, transport) = make_sdk_with_responses(vec![
        Ok(TransportResponse {
            status: 200,
            body: r#"{"crimes":{"nerve_spent":8152,"skill":100,"progression_bonus":15,"attempts":{"total":4076,"success":3601,"fail":453,"critical_fail":22,"subcrimes":[{"id":1,"total":235,"success":214,"fail":21}]},"rewards":{"money":519735,"ammo":{"standard":200,"special":0},"items":[{"id":196,"amount":25}]},"miscellaneous":{"brewery":{"total":12}},"uniques":[{"id":11762,"rewards":{"items":[],"money":null,"ammo":{"standard":0,"special":500}}}]}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"list":[{"id":3954449,"name":"-Josh-","level":43,"faction_id":22680,"status":{"description":"Okay","details":null,"state":"Okay","color":"green","until":null},"last_action":{"status":"Online","timestamp":1777454799,"relative":"2 minutes ago"}}],"_metadata":{"links":{"prev":null,"next":"https://api.torn.com/v2/user/list?&limit=1&cat=Friends&offset=1"},"total":8}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"log":[{"id":"voierevVYEKWdHMdG2Go","timestamp":1777454585,"details":{"id":6700,"title":"Bounty place","category":"Bounties"},"data":{"target":4083992,"cost":1515000},"params":{}}],"_metadata":{"links":{"prev":"https://api.torn.com/v2/user/log?&limit=1&sort=desc&to=1777454585","next":null}}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"networth":{"pending":0,"wallet":86000,"bank":2384400000,"points":52463440,"cayman":0,"vault":454531711,"piggybank":0,"items":6238889224,"displaycase":8438291,"bazaar":104424125,"trade":0,"itemmarket":531,"properties":1473054750,"stockmarket":3998114000,"auctionhouse":0,"company":23105565120,"bookie":0,"enlistedcars":58134157,"loan":0,"unpaidfees":-8841750,"total":37869259599,"parsetime":0.024905920028686523,"timestamp":1777452107}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"personalstats":{"attacking":{"attacks":{"won":3199}},"networth":{"total":37290259444},"other":{"awards":403}}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"personalstats":[{"name":"networth","value":37428074684,"timestamp":1777334400}]}"#.to_string(),
        }),
    ]);

    let crimes = sdk
        .user()
        .crimes(UserOptions::default().with_crime_id("1"))
        .await
        .expect("user crimes should deserialize");
    assert_eq!(crimes.crimes.skill, Some(100));
    assert_eq!(
        crimes
            .crimes
            .attempts
            .as_ref()
            .map(|attempts| attempts.subcrimes.len()),
        Some(1)
    );
    assert_eq!(
        crimes
            .crimes
            .rewards
            .as_ref()
            .and_then(|rewards| rewards.ammo.as_ref())
            .and_then(|ammo| ammo.standard),
        Some(200)
    );

    let list = sdk
        .user()
        .list(
            UserOptions::default()
                .with_base(BaseOptions::default().with_cat("Friends").with_limit(1)),
        )
        .await
        .expect("user list should deserialize");
    assert_eq!(list.list[0].name.as_deref(), Some("-Josh-"));
    assert_eq!(
        list._metadata.as_ref().and_then(|metadata| metadata.total),
        Some(8)
    );

    let log = sdk
        .user()
        .log(UserOptions::default().with_base(BaseOptions::default().with_limit(1)))
        .await
        .expect("user log should deserialize");
    assert_eq!(
        log.log[0].details.as_ref().and_then(|details| details.id),
        Some(6700)
    );
    assert_eq!(log.log[0].id.as_deref(), Some("voierevVYEKWdHMdG2Go"));

    let networth = sdk
        .user()
        .networth(UserOptions::default())
        .await
        .expect("user networth should deserialize");
    assert_eq!(networth.networth.total, Some(37_869_259_599));
    assert_eq!(networth.networth.unpaidfees, Some(-8_841_750));

    let personalstats_category = sdk
        .user()
        .personalstats(UserOptions::default().with_base(BaseOptions::default().with_cat("popular")))
        .await
        .expect("user personalstats category mode should deserialize");
    match personalstats_category.personalstats.as_ref() {
        Some(UserPersonalStatsSelection::Category(category)) => {
            assert_eq!(
                category
                    .networth
                    .as_ref()
                    .and_then(|value| value.get("total"))
                    .and_then(|value| value.as_i64()),
                Some(37_290_259_444)
            );
        }
        other => panic!("unexpected personalstats category payload: {other:?}"),
    }

    let personalstats_series = sdk
        .user()
        .personalstats(
            UserOptions::default().with_base(BaseOptions::default().with_stat("networth")),
        )
        .await
        .expect("user personalstats stat mode should deserialize");
    match personalstats_series.personalstats.as_ref() {
        Some(UserPersonalStatsSelection::Series(points)) => {
            assert_eq!(points[0].name.as_deref(), Some("networth"));
            assert_eq!(points[0].value, Some(37_428_074_684));
        }
        other => panic!("unexpected personalstats stat payload: {other:?}"),
    }

    let requests = transport.requests();
    assert_eq!(requests.len(), 6);
    assert_eq!(requests[0].query.get("id"), Some(&"1".to_string()));
    assert_eq!(requests[1].query.get("cat"), Some(&"Friends".to_string()));
    assert_eq!(requests[4].query.get("cat"), Some(&"popular".to_string()));
    assert_eq!(requests[5].query.get("stat"), Some(&"networth".to_string()));
}

#[tokio::test]
async fn typed_user_organizedcrime_preserves_error_variant() {
    let (sdk, _) = make_sdk_with_responses(vec![Ok(TransportResponse {
        status: 200,
        body: r#"{"organizedCrime":{"code":27,"error":"User is not in an organized crime."}}"#
            .to_string(),
    })]);

    let organizedcrime = sdk
        .user()
        .organizedcrime(UserOptions::default())
        .await
        .expect("organizedcrime error variant should deserialize");
    match organizedcrime.organized_crime.as_ref() {
        Some(UserOrganizedCrimeSelection::Error(error)) => {
            assert_eq!(error.code, Some(27));
            assert_eq!(
                error.error.as_deref(),
                Some("User is not in an organized crime.")
            );
        }
        other => panic!("unexpected organized crime error payload: {other:?}"),
    }
}

#[tokio::test]
async fn typed_user_legacy_state_surfaces_deserialize_conservatively() {
    let (sdk, transport) = make_sdk_with_responses(vec![
        Ok(TransportResponse {
            status: 200,
            body: r#"{"display":[{"ID":653,"UID":15525247867,"name":"Combat Boots","type":"Defensive","quantity":1,"circulation":152822,"market_price":2585275}]}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"active_gym":26}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"faction_perks":["+ 10% strength gym gains"],"job_perks":["+ 10% course time reduction"],"property_perks":["+ Access to airstrip"],"education_perks":["+ Needle equipping"],"enhancer_perks":["+ 4 travel items (Large Suitcase)"],"book_perks":[],"stock_perks":["+ Company sales boost (TCP)"],"merit_perks":["+ 50% life"]}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"criminalrecord":{"vandalism":9381,"theft":14651,"counterfeiting":5863,"fraud":3874,"illicitservices":633,"cybercrime":3673,"extortion":0,"illegalproduction":0,"total":38075}}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"bazaar_is_open":false,"bazaar":[]}"#.to_string(),
        }),
        Ok(TransportResponse {
            status: 200,
            body: r#"{"reports":[{"type":"money","target_id":3637232,"reporter_id":111111,"faction_id":22222,"timestamp":1710002000,"report":{"amount":2500000,"balance":9900000}}],"_metadata":{"links":{"next":null,"prev":null}}}"#.to_string(),
        }),
    ]);

    let display = sdk
        .user()
        .display(UserOptions::default())
        .await
        .expect("user display should deserialize");
    assert_eq!(display.display[0].id, Some(653));
    assert_eq!(display.display[0].uid, Some(15525247867));
    assert_eq!(display.display[0].item_type.as_deref(), Some("Defensive"));

    let gym = sdk
        .user()
        .gym(UserOptions::default())
        .await
        .expect("user gym should deserialize");
    assert_eq!(gym.active_gym, Some(26));

    let perks = sdk
        .user()
        .perks(UserOptions::default())
        .await
        .expect("user perks should deserialize");
    assert_eq!(perks.faction_perks.len(), 1);
    assert_eq!(perks.book_perks.len(), 0);
    assert_eq!(perks.merit_perks[0], "+ 50% life");

    let criminalrecord = sdk
        .user()
        .criminalrecord(UserOptions::default())
        .await
        .expect("user criminalrecord should deserialize");
    assert_eq!(criminalrecord.criminalrecord.total, Some(38075));
    assert_eq!(criminalrecord.criminalrecord.cybercrime, Some(3673));

    let bazaar = sdk
        .user()
        .bazaar(UserOptions::default())
        .await
        .expect("user bazaar should deserialize");
    assert_eq!(bazaar.bazaar_is_open, Some(false));
    assert!(bazaar.bazaar.is_empty());

    let reports = sdk
        .user()
        .reports(UserOptions::default())
        .await
        .expect("user reports should deserialize");
    assert_eq!(reports.reports.len(), 1);
    assert_eq!(reports.reports[0].report_type.as_deref(), Some("money"));
    assert_eq!(
        reports.reports[0].report.get("balance"),
        Some(&9900000.into())
    );
    assert!(reports._metadata.is_some());

    let requests = transport.requests();
    assert_eq!(requests.len(), 6);
    assert_eq!(requests[0].path, "/user");
    assert_eq!(
        requests[0].query.get("selections"),
        Some(&"display".to_string())
    );
    assert_eq!(requests[1].path, "/user");
    assert_eq!(requests[2].path, "/user");
    assert_eq!(requests[3].path, "/user");
    assert_eq!(requests[4].path, "/user");
    assert_eq!(requests[5].path, "/user");
    assert_eq!(
        requests[5].query.get("selections"),
        Some(&"reports".to_string())
    );
}
