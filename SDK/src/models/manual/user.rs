use std::collections::BTreeMap;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct UserProfileBundle {
    pub profile: UserProfile,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserBasicBundle {
    pub profile: UserBasicProfile,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserDiscordBundle {
    pub discord: UserDiscord,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserFactionBundle {
    pub faction: UserFaction,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserMoneyBundle {
    pub money: UserMoney,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserBarsBundle {
    pub bars: UserBars,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserCooldownsBundle {
    pub cooldowns: UserCooldowns,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserTravelBundle {
    pub travel: UserTravel,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserProfile {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub level: Option<u32>,
    #[serde(default)]
    pub rank: Option<String>,
    #[serde(default)]
    pub faction_id: Option<u64>,
    #[serde(default)]
    pub age: Option<u32>,
    #[serde(default)]
    pub karma: Option<i64>,
    #[serde(default)]
    pub revivable: Option<bool>,
    #[serde(default)]
    pub life: Option<UserLifeBar>,
    #[serde(default)]
    pub last_action: Option<UserLastAction>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserBasicProfile {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub level: Option<u32>,
    #[serde(default)]
    pub gender: Option<String>,
    #[serde(default)]
    pub status: Option<UserStatus>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserStatus {
    #[serde(default)]
    pub state: Option<String>,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub details: Option<String>,
    #[serde(default)]
    pub plane_image_type: Option<String>,
    #[serde(default)]
    pub until: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserDiscord {
    #[serde(default, alias = "discordID")]
    pub discord_id: Option<String>,
    #[serde(default)]
    pub user_id: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserFaction {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub position: Option<String>,
    #[serde(default)]
    pub tag: Option<String>,
    #[serde(default)]
    pub tag_image: Option<String>,
    #[serde(default)]
    pub days_in_faction: Option<u32>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserMoney {
    #[serde(default)]
    pub wallet: Option<i64>,
    #[serde(default)]
    pub vault: Option<i64>,
    #[serde(default)]
    pub company: Option<i64>,
    #[serde(default)]
    pub points: Option<i64>,
    #[serde(default)]
    pub cayman_bank: Option<i64>,
    #[serde(default)]
    pub daily_networth: Option<i64>,
    #[serde(default)]
    pub city_bank: Option<UserCityBank>,
    #[serde(default)]
    pub faction: Option<UserFactionMoney>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserCityBank {
    #[serde(default)]
    pub amount: Option<i64>,
    #[serde(default)]
    pub duration: Option<u64>,
    #[serde(default)]
    pub interest_rate: Option<f64>,
    #[serde(default)]
    pub invested_at: Option<u64>,
    #[serde(default)]
    pub profit: Option<i64>,
    #[serde(default)]
    pub until: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserFactionMoney {
    #[serde(default)]
    pub money: Option<i64>,
    #[serde(default)]
    pub points: Option<i64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserBars {
    #[serde(default)]
    pub energy: Option<UserResourceBar>,
    #[serde(default)]
    pub happy: Option<UserResourceBar>,
    #[serde(default)]
    pub life: Option<UserLifeBar>,
    #[serde(default)]
    pub nerve: Option<UserResourceBar>,
    #[serde(default)]
    pub chain: Option<UserChainBar>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserResourceBar {
    #[serde(default)]
    pub current: Option<u64>,
    #[serde(default)]
    pub maximum: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserLifeBar {
    #[serde(default)]
    pub current: Option<u64>,
    #[serde(default)]
    pub maximum: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserChainBar {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub current: Option<u64>,
    #[serde(default)]
    pub max: Option<u64>,
    #[serde(default)]
    pub modifier: Option<f64>,
    #[serde(default)]
    pub cooldown: Option<u64>,
    #[serde(default)]
    pub start: Option<u64>,
    #[serde(default)]
    pub end: Option<u64>,
    #[serde(default)]
    pub timeout: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserCooldowns {
    #[serde(default)]
    pub booster: Option<u64>,
    #[serde(default)]
    pub drug: Option<u64>,
    #[serde(default)]
    pub medical: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserTravel {
    #[serde(default)]
    pub destination: Option<String>,
    #[serde(default)]
    pub method: Option<String>,
    #[serde(default)]
    pub time_left: Option<u64>,
    #[serde(default)]
    pub arrival_at: Option<u64>,
    #[serde(default)]
    pub departed_at: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserLastAction {
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub relative: Option<String>,
    #[serde(default)]
    pub timestamp: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}
