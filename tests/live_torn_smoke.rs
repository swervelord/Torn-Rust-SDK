use std::path::PathBuf;
use std::time::Duration;

use torn_sdk_planner::{
    ApiVersion, DataRequestOptions, ExecutorConfig, FactionOptions, TornClient, TornOptions,
    TornSdk, UserOptions,
};

fn capabilities_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("spec")
        .join("capabilities.json")
}

fn get_env(name: &str) -> Option<String> {
    std::env::var(name)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn live_api_key() -> Option<String> {
    if let Some(value) = get_env("TORN_LIVE_TEST_API_KEY") {
        return Some(value);
    }
    if let Some(value) = get_env("TORN_API_KEY") {
        return Some(value);
    }
    get_env("TORN_API_KEYS").and_then(|keys| {
        keys.split(',')
            .map(str::trim)
            .find(|key| !key.is_empty())
            .map(ToOwned::to_owned)
    })
}

fn test_executor_config() -> ExecutorConfig {
    ExecutorConfig {
        timeout: Duration::from_secs(20),
        max_attempts: 1,
        ..ExecutorConfig::default()
    }
}

#[tokio::test]
#[ignore = "requires real Torn credentials; run with --ignored and env vars set"]
async fn live_multi_resource_smoke_typed_and_raw() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenvy::dotenv();
    let Some(api_key) = live_api_key() else {
        return Ok(());
    };

    let user_id = get_env("TORN_LIVE_TEST_USER_ID").unwrap_or_else(|| "3637232".to_string());
    let item_id = get_env("TORN_LIVE_TEST_ITEM_ID").unwrap_or_else(|| "1".to_string());

    let client = TornClient::from_capabilities_file(
        capabilities_path(),
        vec![api_key],
        test_executor_config(),
    )?;
    let sdk = TornSdk::new(client);

    let profile = sdk.user().profile(&user_id).await?;
    assert!(profile.profile.id.is_some());

    let basic = sdk.user().basic(&user_id).await?;
    assert!(basic.profile.id.is_some());

    let discord = sdk.user().discord(&user_id).await?;
    assert!(discord.discord.discord_id.is_some() || !discord.discord.extra.is_empty());

    let money = sdk.user().money(UserOptions::default()).await?;
    assert!(money.money.wallet.is_some() || money.money.points.is_some());

    let bars = sdk.user().bars(UserOptions::default()).await?;
    assert!(bars.bars.energy.is_some() || bars.bars.life.is_some());

    let cooldowns = sdk.user().cooldowns(UserOptions::default()).await?;
    assert!(
        cooldowns.cooldowns.booster.is_some()
            || cooldowns.cooldowns.drug.is_some()
            || cooldowns.cooldowns.medical.is_some()
    );

    let travel = sdk.user().travel(UserOptions::default()).await?;
    assert!(travel.travel.destination.is_some() || travel.travel.time_left.is_some());

    let user_faction = sdk.user().faction(&user_id).await?;
    if let Some(faction_id) = user_faction.faction.id {
        let faction = sdk
            .faction()
            .basic(FactionOptions::default().with_id(faction_id.to_string()))
            .await?;
        assert!(faction.basic.name.is_some() || faction.basic.id.is_some());
    }

    let itemmarket = sdk
        .market()
        .itemmarket(
            torn_sdk_planner::MarketOptions::default()
                .with_base(torn_sdk_planner::BaseOptions::default().with_limit(1))
                .with_id(item_id),
        )
        .await?;
    assert!(itemmarket.itemmarket.item.is_some() || !itemmarket.itemmarket.listings.is_empty());

    let key_info = sdk
        .key()
        .info(torn_sdk_planner::KeyOptions::default())
        .await?;
    assert!(
        key_info
            .info
            .access
            .as_ref()
            .and_then(|access| access.access_type.as_deref())
            .is_some()
            || key_info.info.selections.is_some()
    );

    let calendar = sdk.torn().calendar(TornOptions::default()).await?;
    assert!(!calendar.calendar.events.is_empty() || !calendar.calendar.competitions.is_empty());

    let raw_forum_lookup = sdk
        .forum()
        .lookup_raw(torn_sdk_planner::ForumOptions::default())
        .await?;
    assert!(!raw_forum_lookup.selections.is_empty());

    let raw_property_lookup = sdk
        .property()
        .lookup_raw(torn_sdk_planner::PropertyOptions::default())
        .await?;
    assert!(!raw_property_lookup.selections.is_empty());

    let raw_racing_lookup = sdk
        .racing()
        .lookup_raw(torn_sdk_planner::RacingOptions::default())
        .await?;
    assert!(!raw_racing_lookup.selections.is_empty());

    Ok(())
}

#[tokio::test]
#[ignore = "requires real Torn credentials; run with --ignored and env vars set"]
async fn live_runtime_v2_to_v1_fallback_smoke() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenvy::dotenv();
    let Some(api_key) = live_api_key() else {
        return Ok(());
    };
    let user_id = get_env("TORN_LIVE_TEST_USER_ID").unwrap_or_else(|| "3637232".to_string());
    let require_runtime_fallback = matches!(
        get_env("TORN_LIVE_TEST_REQUIRE_RUNTIME_FALLBACK")
            .as_deref()
            .map(str::to_ascii_lowercase)
            .as_deref(),
        Some("1" | "true" | "yes")
    );

    let client = TornClient::from_capabilities_file(
        capabilities_path(),
        vec![api_key],
        test_executor_config(),
    )?;

    let report = client
        .get_user_data(
            vec!["networth"],
            DataRequestOptions::default()
                .with_id(user_id)
                .with_filter("limit", "1"),
        )
        .await?;

    let saw_runtime_v2_to_v1 = report
        .calls
        .iter()
        .any(|call| call.fallback && call.api_version == ApiVersion::V1);
    let saw_any_v1 = report
        .calls
        .iter()
        .any(|call| call.api_version == ApiVersion::V1);

    if require_runtime_fallback {
        assert!(
            saw_runtime_v2_to_v1,
            "expected runtime v2->v1 fallback for networth when TORN_LIVE_TEST_REQUIRE_RUNTIME_FALLBACK=true"
        );
    } else {
        // Networth may be routed straight to v1 by capabilities or served by v2 depending on rollout state.
        assert!(
            saw_any_v1
                || report
                    .calls
                    .iter()
                    .any(|call| call.api_version == ApiVersion::V2)
        );
    }

    Ok(())
}
