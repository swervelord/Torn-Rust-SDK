use std::collections::BTreeMap;

use serde::Deserialize;
use serde_json::Value;

use super::common::{PaginatedMetadata, PaginatedMetadataWithTotal};
use super::forum::{ForumPost, ForumThread, ForumUserSummary};

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
pub struct UserCasinoBundle {
    pub casino: UserCasino,
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
pub struct UserAmmoBundle {
    #[serde(default)]
    pub ammo: Vec<UserAmmoItem>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserBattleStatsBundle {
    pub battlestats: UserBattleStats,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserEquipmentBundle {
    #[serde(default)]
    pub equipment: Vec<UserEquipmentItem>,
    #[serde(default)]
    pub clothing: Vec<UserClothingItem>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserMissionsBundle {
    pub missions: UserMissions,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserTimestampBundle {
    #[serde(default)]
    pub timestamp: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserNotificationsBundle {
    pub notifications: UserNotifications,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserIconsBundle {
    #[serde(default)]
    pub icons: Vec<UserIcon>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserEnlistedCarsBundle {
    #[serde(default)]
    pub enlistedcars: Vec<UserEnlistedCar>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserJobBundle {
    #[serde(default)]
    pub job: Option<UserEmployment>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserJobPointsBundle {
    pub jobpoints: UserJobPoints,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserJobRanksBundle {
    pub jobranks: UserJobRanks,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserRefillsBundle {
    pub refills: UserRefills,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserSkillsBundle {
    #[serde(default)]
    pub skills: Vec<UserSkill>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserStocksBundle {
    #[serde(default)]
    pub stocks: Vec<UserStockHolding>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserCalendarBundle {
    pub calendar: UserCalendar,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserLookupBundle {
    #[serde(default)]
    pub selections: Vec<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserFactionBalanceBundle {
    #[serde(default, rename = "factionBalance")]
    pub faction_balance: Option<UserFactionBalance>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserEducationBundle {
    pub education: UserEducation,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserCompetitionBundle {
    #[serde(default)]
    pub competition: Option<UserCompetition>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserWeaponExpBundle {
    #[serde(default)]
    pub weaponexp: Vec<UserWeaponExpEntry>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserRacingRecordsBundle {
    #[serde(default)]
    pub racingrecords: Vec<UserRacingRecordSummary>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserHonorsBundle {
    #[serde(default)]
    pub honors: Vec<UserAwardUnlock>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserMedalsBundle {
    #[serde(default)]
    pub medals: Vec<UserAwardUnlock>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserInventoryBundle {
    pub inventory: UserInventory,
    #[serde(default)]
    pub _metadata: Option<PaginatedMetadataWithTotal>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserTradesBundle {
    #[serde(default)]
    pub trades: Vec<UserTrade>,
    #[serde(default)]
    pub _metadata: Option<PaginatedMetadata>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserTradeBundle {
    pub trade: UserTradeDetailed,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserTravelBundle {
    pub travel: UserTravel,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserDisplayBundle {
    #[serde(default)]
    pub display: Vec<UserLegacyItemSummary>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserGymBundle {
    #[serde(default, rename = "active_gym")]
    pub active_gym: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserPerksBundle {
    #[serde(default)]
    pub faction_perks: Vec<String>,
    #[serde(default)]
    pub job_perks: Vec<String>,
    #[serde(default)]
    pub property_perks: Vec<String>,
    #[serde(default)]
    pub education_perks: Vec<String>,
    #[serde(default)]
    pub enhancer_perks: Vec<String>,
    #[serde(default)]
    pub book_perks: Vec<String>,
    #[serde(default)]
    pub stock_perks: Vec<String>,
    #[serde(default)]
    pub merit_perks: Vec<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserCriminalRecordBundle {
    pub criminalrecord: UserCriminalRecord,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserBazaarBundle {
    #[serde(default)]
    pub bazaar_is_open: Option<bool>,
    #[serde(default)]
    pub bazaar: Vec<UserLegacyItemSummary>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserReportsBundle {
    #[serde(default)]
    pub reports: Vec<UserReportSummary>,
    #[serde(default)]
    pub _metadata: Option<PaginatedMetadata>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserCrimesBundle {
    pub crimes: UserCrimeSummary,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserListBundle {
    #[serde(default)]
    pub list: Vec<UserListEntry>,
    #[serde(default)]
    pub _metadata: Option<PaginatedMetadataWithTotal>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserLogBundle {
    #[serde(default)]
    pub log: Vec<UserLogEntry>,
    #[serde(default)]
    pub _metadata: Option<PaginatedMetadata>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserNetworthBundle {
    pub networth: UserNetworth,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserPersonalStatsBundle {
    #[serde(default)]
    pub personalstats: Option<UserPersonalStatsSelection>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserEventsBundle {
    #[serde(default)]
    pub events: Vec<UserEvent>,
    #[serde(default)]
    pub _metadata: Option<PaginatedMetadata>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserNewEventsBundle {
    #[serde(default)]
    pub events: Vec<UserEvent>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserMessagesBundle {
    #[serde(default)]
    pub messages: Vec<UserMessage>,
    #[serde(default)]
    pub _metadata: Option<PaginatedMetadata>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserNewMessagesBundle {
    #[serde(default)]
    pub messages: Vec<UserMessage>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserRacesBundle {
    #[serde(default)]
    pub races: Vec<UserRace>,
    #[serde(default)]
    pub _metadata: Option<PaginatedMetadata>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserAttacksBundle {
    #[serde(default)]
    pub attacks: Vec<UserAttack>,
    #[serde(default)]
    pub _metadata: Option<PaginatedMetadata>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserAttacksFullBundle {
    #[serde(default)]
    pub attacks: Vec<UserAttackFull>,
    #[serde(default)]
    pub _metadata: Option<PaginatedMetadata>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserRevivesBundle {
    #[serde(default)]
    pub revives: Vec<UserRevive>,
    #[serde(default)]
    pub _metadata: Option<PaginatedMetadata>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserRevivesFullBundle {
    #[serde(default)]
    pub revives: Vec<UserReviveFull>,
    #[serde(default)]
    pub _metadata: Option<PaginatedMetadata>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserHofBundle {
    pub hof: UserHofStats,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserBountiesBundle {
    #[serde(default)]
    pub bounties: Vec<UserBounty>,
    #[serde(default)]
    pub bounties_timestamp: Option<u64>,
    #[serde(default)]
    pub bounties_delay: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserPropertiesBundle {
    #[serde(default)]
    pub properties: Vec<UserPropertySummary>,
    #[serde(default)]
    pub _metadata: Option<PaginatedMetadata>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserPropertyBundle {
    pub property: UserPropertySummary,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserForumThreadsBundle {
    #[serde(default, rename = "forumThreads")]
    pub forum_threads: Vec<ForumThread>,
    #[serde(default)]
    pub _metadata: Option<PaginatedMetadata>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserForumPostsBundle {
    #[serde(default, rename = "forumPosts")]
    pub forum_posts: Vec<ForumPost>,
    #[serde(default)]
    pub _metadata: Option<PaginatedMetadata>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserForumSubscribedThreadsBundle {
    #[serde(default, rename = "forumSubscribedThreads")]
    pub forum_subscribed_threads: Vec<UserForumSubscribedThread>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserForumFeedBundle {
    #[serde(default, rename = "forumFeed")]
    pub forum_feed: Vec<UserForumFeedEntry>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserForumFriendsBundle {
    #[serde(default, rename = "forumFriends")]
    pub forum_friends: Vec<UserForumFeedEntry>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserWorkStatsBundle {
    pub workstats: UserWorkStats,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserMeritsBundle {
    pub merits: UserMerits,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserItemMarketBundle {
    #[serde(default)]
    pub itemmarket: Vec<UserItemMarketListing>,
    #[serde(default)]
    pub _metadata: Option<PaginatedMetadata>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserOrganizedCrimeBundle {
    #[serde(default, rename = "organizedCrime")]
    pub organized_crime: Option<UserOrganizedCrimeSelection>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserLegacyItemSummary {
    #[serde(default, rename = "ID", alias = "id")]
    pub id: Option<u64>,
    #[serde(default, rename = "UID", alias = "uid")]
    pub uid: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default, rename = "type")]
    pub item_type: Option<String>,
    #[serde(default)]
    pub quantity: Option<i64>,
    #[serde(default)]
    pub circulation: Option<i64>,
    #[serde(default)]
    pub market_price: Option<i64>,
    #[serde(default)]
    pub cost: Option<i64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserCriminalRecord {
    #[serde(default)]
    pub vandalism: Option<u64>,
    #[serde(default)]
    pub theft: Option<u64>,
    #[serde(default)]
    pub counterfeiting: Option<u64>,
    #[serde(default)]
    pub fraud: Option<u64>,
    #[serde(default)]
    pub illicitservices: Option<u64>,
    #[serde(default)]
    pub cybercrime: Option<u64>,
    #[serde(default)]
    pub extortion: Option<u64>,
    #[serde(default)]
    pub illegalproduction: Option<u64>,
    #[serde(default)]
    pub total: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserReportSummary {
    #[serde(default, rename = "type")]
    pub report_type: Option<String>,
    #[serde(default)]
    pub target_id: Option<u64>,
    #[serde(default)]
    pub reporter_id: Option<u64>,
    #[serde(default)]
    pub faction_id: Option<u64>,
    #[serde(default)]
    pub timestamp: Option<u64>,
    #[serde(default)]
    pub report: Value,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserCrimeSummary {
    #[serde(default)]
    pub nerve_spent: Option<u64>,
    #[serde(default)]
    pub skill: Option<u64>,
    #[serde(default)]
    pub progression_bonus: Option<u64>,
    #[serde(default)]
    pub rewards: Option<UserCrimeRewards>,
    #[serde(default)]
    pub attempts: Option<UserCrimeAttempts>,
    #[serde(default)]
    pub uniques: Vec<UserCrimeUnique>,
    #[serde(default)]
    pub miscellaneous: Option<Value>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserCrimeRewards {
    #[serde(default)]
    pub money: Option<u64>,
    #[serde(default)]
    pub ammo: Option<UserCrimeRewardAmmo>,
    #[serde(default)]
    pub items: Vec<UserCrimeRewardItem>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserCrimeRewardAmmo {
    #[serde(default)]
    pub standard: Option<u64>,
    #[serde(default)]
    pub special: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserCrimeRewardItem {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default, alias = "amount", alias = "quantity")]
    pub amount: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserCrimeAttempts {
    #[serde(default)]
    pub total: Option<u64>,
    #[serde(default)]
    pub success: Option<u64>,
    #[serde(default)]
    pub fail: Option<u64>,
    #[serde(default)]
    pub critical_fail: Option<u64>,
    #[serde(default)]
    pub subcrimes: Vec<UserCrimeAttemptSubcrime>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserCrimeAttemptSubcrime {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub total: Option<u64>,
    #[serde(default)]
    pub success: Option<u64>,
    #[serde(default)]
    pub fail: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserCrimeUnique {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub rewards: Option<UserCrimeUniqueRewards>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserCrimeUniqueRewards {
    #[serde(default)]
    pub money: Option<u64>,
    #[serde(default)]
    pub ammo: Option<UserCrimeRewardAmmo>,
    #[serde(default)]
    pub items: Vec<UserCrimeRewardItem>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserListEntry {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub level: Option<u32>,
    #[serde(default)]
    pub faction_id: Option<u64>,
    #[serde(default)]
    pub status: Option<UserStatus>,
    #[serde(default)]
    pub last_action: Option<UserLastAction>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserLogEntry {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub timestamp: Option<u64>,
    #[serde(default)]
    pub details: Option<UserLogDetails>,
    #[serde(default)]
    pub data: Value,
    #[serde(default)]
    pub params: Value,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserLogDetails {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserNetworth {
    #[serde(default)]
    pub pending: Option<i64>,
    #[serde(default)]
    pub wallet: Option<i64>,
    #[serde(default)]
    pub bank: Option<i64>,
    #[serde(default)]
    pub points: Option<i64>,
    #[serde(default)]
    pub cayman: Option<i64>,
    #[serde(default)]
    pub vault: Option<i64>,
    #[serde(default)]
    pub piggybank: Option<i64>,
    #[serde(default)]
    pub items: Option<i64>,
    #[serde(default)]
    pub displaycase: Option<i64>,
    #[serde(default)]
    pub bazaar: Option<i64>,
    #[serde(default)]
    pub trade: Option<i64>,
    #[serde(default)]
    pub itemmarket: Option<i64>,
    #[serde(default)]
    pub properties: Option<i64>,
    #[serde(default)]
    pub stockmarket: Option<i64>,
    #[serde(default)]
    pub auctionhouse: Option<i64>,
    #[serde(default)]
    pub company: Option<i64>,
    #[serde(default)]
    pub bookie: Option<i64>,
    #[serde(default)]
    pub enlistedcars: Option<i64>,
    #[serde(default)]
    pub loan: Option<i64>,
    #[serde(default)]
    pub unpaidfees: Option<i64>,
    #[serde(default)]
    pub total: Option<i64>,
    #[serde(default)]
    pub parsetime: Option<f64>,
    #[serde(default)]
    pub timestamp: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum UserPersonalStatsSelection {
    Category(Box<UserPersonalStatsCategory>),
    Series(Vec<UserPersonalStatPoint>),
    Unknown(Value),
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserPersonalStatsCategory {
    #[serde(default)]
    pub attacking: Option<Value>,
    #[serde(default)]
    pub jobs: Option<Value>,
    #[serde(default)]
    pub hospital: Option<Value>,
    #[serde(default)]
    pub crimes: Option<Value>,
    #[serde(default)]
    pub items: Option<Value>,
    #[serde(default)]
    pub travel: Option<Value>,
    #[serde(default)]
    pub drugs: Option<Value>,
    #[serde(default)]
    pub networth: Option<Value>,
    #[serde(default)]
    pub other: Option<Value>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserPersonalStatPoint {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub value: Option<i64>,
    #[serde(default)]
    pub timestamp: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
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
pub struct UserCasino {
    #[serde(default)]
    pub tokens: Option<i64>,
    #[serde(default)]
    pub streak: Option<i64>,
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
pub struct UserEquipmentItem {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub uid: Option<u64>,
    #[serde(default, rename = "type")]
    pub item_type: Option<String>,
    #[serde(default)]
    pub sub_type: Option<String>,
    #[serde(default)]
    pub stats: Option<UserItemMarketItemStats>,
    #[serde(default)]
    pub bonuses: Vec<UserItemMarketItemBonus>,
    #[serde(default)]
    pub rarity: Option<String>,
    #[serde(default)]
    pub slot: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserClothingItem {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub uid: Option<u64>,
    #[serde(default, rename = "type")]
    pub item_type: Option<String>,
    #[serde(default)]
    pub sub_type: Option<String>,
    #[serde(default)]
    pub slot: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
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
pub struct UserAmmoItem {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub types: Vec<UserAmmoVariant>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserAmmoVariant {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub quantity: Option<i64>,
    #[serde(default)]
    pub equipped: Option<bool>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserBattleStats {
    #[serde(default)]
    pub strength: Option<UserBattleStat>,
    #[serde(default)]
    pub defense: Option<UserBattleStat>,
    #[serde(default)]
    pub speed: Option<UserBattleStat>,
    #[serde(default)]
    pub dexterity: Option<UserBattleStat>,
    #[serde(default)]
    pub total: Option<i64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserBattleStat {
    #[serde(default)]
    pub value: Option<i64>,
    #[serde(default)]
    pub modifier: Option<f64>,
    #[serde(default)]
    pub modifiers: Vec<UserBattleStatModifier>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserBattleStatModifier {
    #[serde(default)]
    pub effect: Option<String>,
    #[serde(default)]
    pub value: Option<f64>,
    #[serde(default, rename = "type")]
    pub modifier_type: Option<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserNotifications {
    #[serde(default)]
    pub messages: Option<u64>,
    #[serde(default)]
    pub events: Option<u64>,
    #[serde(default)]
    pub awards: Option<u64>,
    #[serde(default)]
    pub competition: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserIcon {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub until: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserEnlistedCar {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub car_item_id: Option<u64>,
    #[serde(default)]
    pub car_item_name: Option<String>,
    #[serde(default, rename = "car_name", alias = "name")]
    pub car_name: Option<String>,
    #[serde(default)]
    pub top_speed: Option<i64>,
    #[serde(default)]
    pub acceleration: Option<i64>,
    #[serde(default)]
    pub braking: Option<i64>,
    #[serde(default)]
    pub dirt: Option<i64>,
    #[serde(default)]
    pub handling: Option<i64>,
    #[serde(default)]
    pub safety: Option<i64>,
    #[serde(default)]
    pub tarmac: Option<i64>,
    #[serde(default, rename = "class")]
    pub car_class: Option<String>,
    #[serde(default)]
    pub worth: Option<i64>,
    #[serde(default)]
    pub points_spent: Option<u64>,
    #[serde(default)]
    pub races_entered: Option<u64>,
    #[serde(default)]
    pub races_won: Option<u64>,
    #[serde(default)]
    pub is_removed: Option<bool>,
    #[serde(default)]
    pub parts: Vec<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone)]
pub enum UserEmployment {
    Job(UserCityJob),
    Company(UserCompanyJob),
    Unknown(UnknownUserEmployment),
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserCityJob {
    #[serde(default, rename = "type")]
    pub job_type: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub position: Option<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserCompanyJob {
    #[serde(default, rename = "type")]
    pub job_type: Option<String>,
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub type_id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub rating: Option<i64>,
    #[serde(default)]
    pub position: Option<String>,
    #[serde(default)]
    pub days_in_company: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Default)]
pub struct UnknownUserEmployment {
    pub job_type: Option<String>,
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserJobPoints {
    #[serde(default)]
    pub jobs: Option<UserJobPointTotals>,
    #[serde(default)]
    pub companies: Vec<UserCompanyPoints>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserJobPointTotals {
    #[serde(default)]
    pub army: Option<u64>,
    #[serde(default)]
    pub casino: Option<u64>,
    #[serde(default)]
    pub education: Option<u64>,
    #[serde(default)]
    pub grocer: Option<u64>,
    #[serde(default)]
    pub law: Option<u64>,
    #[serde(default)]
    pub medical: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserCompanyPoints {
    #[serde(default)]
    pub company: Option<UserCompanyPointCompany>,
    #[serde(default)]
    pub points: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserCompanyPointCompany {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserJobRanks {
    #[serde(default)]
    pub army: Option<String>,
    #[serde(default)]
    pub grocer: Option<String>,
    #[serde(default)]
    pub casino: Option<String>,
    #[serde(default)]
    pub medical: Option<String>,
    #[serde(default)]
    pub law: Option<String>,
    #[serde(default)]
    pub education: Option<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserRefills {
    #[serde(default)]
    pub energy: Option<bool>,
    #[serde(default)]
    pub nerve: Option<bool>,
    #[serde(default)]
    pub token: Option<bool>,
    #[serde(default)]
    pub special_count: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserSkill {
    #[serde(default)]
    pub slug: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub level: Option<f64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserStockHolding {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub shares: Option<i64>,
    #[serde(default)]
    pub transactions: Vec<UserStockTransaction>,
    #[serde(default)]
    pub bonus: Option<UserStockBonus>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserStockTransaction {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub shares: Option<i64>,
    #[serde(default)]
    pub price: Option<f64>,
    #[serde(default)]
    pub timestamp: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserStockBonus {
    #[serde(default)]
    pub available: Option<bool>,
    #[serde(default)]
    pub increment: Option<u64>,
    #[serde(default)]
    pub progress: Option<u64>,
    #[serde(default)]
    pub frequency: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserCalendar {
    #[serde(default)]
    pub start_time: Option<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserFactionBalance {
    #[serde(default)]
    pub money: Option<i64>,
    #[serde(default)]
    pub points: Option<i64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserMissions {
    #[serde(default)]
    pub credits: Option<u64>,
    #[serde(default)]
    pub givers: Vec<UserMissionGiver>,
    #[serde(default)]
    pub rewards: Vec<UserMissionReward>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserMissionGiver {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub contracts: Vec<UserMissionContract>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserMissionContract {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub difficulty: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub created_at: Option<u64>,
    #[serde(default)]
    pub started_at: Option<u64>,
    #[serde(default)]
    pub expires_at: Option<u64>,
    #[serde(default)]
    pub completed_at: Option<u64>,
    #[serde(default)]
    pub rewards: Option<UserMissionContractRewards>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserMissionContractRewards {
    #[serde(default)]
    pub money: Option<i64>,
    #[serde(default)]
    pub credits: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserMissionReward {
    #[serde(default, rename = "type")]
    pub reward_type: Option<String>,
    #[serde(default)]
    pub details: Option<UserMissionRewardDetails>,
    #[serde(default)]
    pub amount: Option<i64>,
    #[serde(default)]
    pub cost: Option<i64>,
    #[serde(default)]
    pub expires_at: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserMissionRewardDetails {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default, rename = "type")]
    pub detail_type: Option<String>,
    #[serde(default)]
    pub sub_type: Option<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserEducation {
    #[serde(default)]
    pub complete: Vec<u64>,
    #[serde(default)]
    pub current: Option<UserCurrentEducation>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserCurrentEducation {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub until: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone)]
pub enum UserCompetition {
    Halloween(UserHalloweenCompetition),
    EasterEggs(UserEasterCompetition),
    RockPaperScissors(UserRpsCompetition),
    Elimination(UserEliminationCompetition),
    Unknown(UnknownUserCompetition),
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserHalloweenCompetition {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub treats_collected: Option<u64>,
    #[serde(default)]
    pub basket: Option<UserCompetitionBasket>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserEasterCompetition {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub score: Option<u64>,
    #[serde(default)]
    pub total: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserRpsCompetition {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub hp: Option<UserCompetitionHp>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserEliminationCompetition {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub score: Option<u64>,
    #[serde(default)]
    pub attacks: Option<u64>,
    #[serde(default)]
    pub team: Option<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserCompetitionBasket {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserCompetitionHp {
    #[serde(default)]
    pub current: Option<u64>,
    #[serde(default)]
    pub maximum: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Default)]
pub struct UnknownUserCompetition {
    pub name: Option<String>,
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserWeaponExpEntry {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub exp: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserRacingRecordSummary {
    #[serde(default)]
    pub track: Option<UserRaceTrackSummary>,
    #[serde(default)]
    pub records: Vec<UserRacingLapRecord>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserRaceTrackSummary {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserRacingLapRecord {
    #[serde(default)]
    pub car_id: Option<u64>,
    #[serde(default)]
    pub car_name: Option<String>,
    #[serde(default)]
    pub lap_time: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserAwardUnlock {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub timestamp: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserInventory {
    #[serde(default)]
    pub items: Vec<UserInventoryItem>,
    #[serde(default)]
    pub timestamp: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserInventoryItem {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub amount: Option<u64>,
    #[serde(default)]
    pub equipped: Option<bool>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub faction_owned: Option<bool>,
    #[serde(default)]
    pub uid: Option<u64>,
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
pub struct UserEvent {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub timestamp: Option<u64>,
    #[serde(default, rename = "event")]
    pub text: Option<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserMessage {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub sender: Option<UserIdentitySummary>,
    #[serde(default)]
    pub timestamp: Option<u64>,
    #[serde(default)]
    pub topic: Option<String>,
    #[serde(default, rename = "type")]
    pub message_type: Option<String>,
    #[serde(default)]
    pub seen: Option<bool>,
    #[serde(default)]
    pub read: Option<bool>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserIdentitySummary {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserRace {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub track_id: Option<u64>,
    #[serde(default)]
    pub creator_id: Option<u64>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub laps: Option<u64>,
    #[serde(default)]
    pub participants: Option<UserRaceParticipants>,
    #[serde(default)]
    pub schedule: Option<UserRaceSchedule>,
    #[serde(default)]
    pub requirements: Option<UserRaceRequirements>,
    #[serde(default)]
    pub is_official: Option<bool>,
    #[serde(default)]
    pub skill_gain: Option<f64>,
    #[serde(default)]
    pub results: Vec<UserRaceResult>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserRaceParticipants {
    #[serde(default)]
    pub minimum: Option<u64>,
    #[serde(default)]
    pub maximum: Option<u64>,
    #[serde(default)]
    pub current: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserRaceSchedule {
    #[serde(default)]
    pub join_from: Option<u64>,
    #[serde(default)]
    pub join_until: Option<u64>,
    #[serde(default)]
    pub start: Option<u64>,
    #[serde(default)]
    pub end: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserRaceRequirements {
    #[serde(default)]
    pub driver_class: Option<String>,
    #[serde(default)]
    pub car_class: Option<String>,
    #[serde(default)]
    pub car_item_id: Option<u64>,
    #[serde(default)]
    pub requires_stock_car: Option<bool>,
    #[serde(default)]
    pub requires_password: Option<bool>,
    #[serde(default)]
    pub join_fee: Option<i64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserRaceResult {
    #[serde(default)]
    pub driver_id: Option<u64>,
    #[serde(default)]
    pub position: Option<u64>,
    #[serde(default)]
    pub car_id: Option<u64>,
    #[serde(default)]
    pub car_item_id: Option<u64>,
    #[serde(default)]
    pub car_item_name: Option<String>,
    #[serde(default)]
    pub car_class: Option<String>,
    #[serde(default)]
    pub has_crashed: Option<bool>,
    #[serde(default)]
    pub best_lap_time: Option<f64>,
    #[serde(default)]
    pub race_time: Option<f64>,
    #[serde(default)]
    pub time_ended: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserAttack {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub code: Option<String>,
    #[serde(default)]
    pub started: Option<u64>,
    #[serde(default)]
    pub ended: Option<u64>,
    #[serde(default)]
    pub attacker: Option<UserAttackCombatant>,
    #[serde(default)]
    pub defender: Option<UserAttackCombatant>,
    #[serde(default)]
    pub result: Option<String>,
    #[serde(default)]
    pub respect_gain: Option<f64>,
    #[serde(default)]
    pub respect_loss: Option<f64>,
    #[serde(default)]
    pub chain: Option<u64>,
    #[serde(default)]
    pub is_interrupted: Option<bool>,
    #[serde(default)]
    pub is_stealthed: Option<bool>,
    #[serde(default)]
    pub is_raid: Option<bool>,
    #[serde(default)]
    pub is_ranked_war: Option<bool>,
    #[serde(default)]
    pub modifiers: Option<UserAttackModifiers>,
    #[serde(default)]
    pub finishing_hit_effects: Vec<UserAttackFinishingHitEffect>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserAttackFull {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub code: Option<String>,
    #[serde(default)]
    pub started: Option<u64>,
    #[serde(default)]
    pub ended: Option<u64>,
    #[serde(default)]
    pub attacker: Option<UserAttackCombatant>,
    #[serde(default)]
    pub defender: Option<UserAttackCombatant>,
    #[serde(default)]
    pub result: Option<String>,
    #[serde(default)]
    pub respect_gain: Option<f64>,
    #[serde(default)]
    pub respect_loss: Option<f64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserAttackCombatant {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub level: Option<u64>,
    #[serde(default)]
    pub faction: Option<UserAttackFaction>,
    #[serde(default)]
    pub faction_id: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserAttackFaction {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserAttackModifiers {
    #[serde(default)]
    pub fair_fight: Option<f64>,
    #[serde(default)]
    pub war: Option<f64>,
    #[serde(default)]
    pub retaliation: Option<f64>,
    #[serde(default)]
    pub group: Option<f64>,
    #[serde(default)]
    pub overseas: Option<f64>,
    #[serde(default)]
    pub chain: Option<f64>,
    #[serde(default)]
    pub warlord: Option<f64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserAttackFinishingHitEffect {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub value: Option<i64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserRevive {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub reviver: Option<UserReviveReviver>,
    #[serde(default)]
    pub target: Option<UserReviveTarget>,
    #[serde(default)]
    pub success_chance: Option<f64>,
    #[serde(default)]
    pub result: Option<String>,
    #[serde(default)]
    pub timestamp: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserReviveFull {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub reviver: Option<UserReviveReviver>,
    #[serde(default)]
    pub target: Option<UserReviveTarget>,
    #[serde(default)]
    pub success_chance: Option<f64>,
    #[serde(default)]
    pub result: Option<String>,
    #[serde(default)]
    pub timestamp: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserReviveReviver {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub faction: Option<UserAttackFaction>,
    #[serde(default)]
    pub faction_id: Option<u64>,
    #[serde(default)]
    pub skill: Option<f64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserReviveTarget {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub faction: Option<UserAttackFaction>,
    #[serde(default)]
    pub faction_id: Option<u64>,
    #[serde(default)]
    pub hospital_reason: Option<String>,
    #[serde(default)]
    pub early_discharge: Option<bool>,
    #[serde(default)]
    pub last_action: Option<UserReviveLastAction>,
    #[serde(default)]
    pub online_status: Option<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum UserReviveLastAction {
    Structured(UserLastAction),
    Timestamp(u64),
    Raw(Value),
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserHofStats {
    #[serde(default)]
    pub attacks: Option<UserHofValue>,
    #[serde(default)]
    pub busts: Option<UserHofValue>,
    #[serde(default)]
    pub defends: Option<UserHofValue>,
    #[serde(default)]
    pub networth: Option<UserHofValue>,
    #[serde(default)]
    pub offences: Option<UserHofValue>,
    #[serde(default)]
    pub revives: Option<UserHofValue>,
    #[serde(default)]
    pub level: Option<UserHofValue>,
    #[serde(default)]
    pub rank: Option<UserHofValue>,
    #[serde(default)]
    pub awards: Option<UserHofValue>,
    #[serde(default)]
    pub racing_skill: Option<UserHofValueFloat>,
    #[serde(default)]
    pub racing_points: Option<UserHofValue>,
    #[serde(default)]
    pub racing_wins: Option<UserHofValue>,
    #[serde(default)]
    pub travel_time: Option<UserHofValue>,
    #[serde(default)]
    pub working_stats: Option<UserHofValue>,
    #[serde(default)]
    pub battle_stats: Option<UserHofValue>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserHofValue {
    #[serde(default)]
    pub value: Option<i64>,
    #[serde(default)]
    pub rank: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserHofValueFloat {
    #[serde(default)]
    pub value: Option<f64>,
    #[serde(default)]
    pub rank: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserBounty {
    #[serde(default)]
    pub target_id: Option<u64>,
    #[serde(default)]
    pub target_name: Option<String>,
    #[serde(default)]
    pub target_level: Option<u64>,
    #[serde(default)]
    pub lister_id: Option<u64>,
    #[serde(default)]
    pub lister_name: Option<String>,
    #[serde(default)]
    pub reward: Option<i64>,
    #[serde(default)]
    pub reason: Option<String>,
    #[serde(default)]
    pub quantity: Option<u64>,
    #[serde(default)]
    pub is_anonymous: Option<bool>,
    #[serde(default)]
    pub valid_until: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserPropertySummary {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub owner: Option<UserIdentitySummary>,
    #[serde(default, rename = "property")]
    pub property_info: Option<UserPropertyKind>,
    #[serde(default)]
    pub happy: Option<i64>,
    #[serde(default)]
    pub upkeep: Option<UserPropertyUpkeep>,
    #[serde(default)]
    pub market_price: Option<i64>,
    #[serde(default)]
    pub modifications: Vec<String>,
    #[serde(default)]
    pub staff: Vec<UserPropertyStaffEntry>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub used_by: Vec<UserIdentitySummary>,
    #[serde(default)]
    pub cost: Option<i64>,
    #[serde(default)]
    pub cost_per_day: Option<i64>,
    #[serde(default)]
    pub rental_period: Option<u64>,
    #[serde(default)]
    pub rental_period_remaining: Option<u64>,
    #[serde(default)]
    pub rented_by: Option<UserIdentitySummary>,
    #[serde(default)]
    pub renter_asked: Option<UserIdentitySummary>,
    #[serde(default)]
    pub lease_extension: Option<UserPropertyLeaseExtension>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserPropertyKind {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserPropertyUpkeep {
    #[serde(default, rename = "property")]
    pub property_cost: Option<i64>,
    #[serde(default)]
    pub staff: Option<i64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserPropertyStaffEntry {
    #[serde(default, rename = "type")]
    pub staff_type: Option<String>,
    #[serde(default)]
    pub amount: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserPropertyLeaseExtension {
    #[serde(default)]
    pub cost: Option<i64>,
    #[serde(default)]
    pub period: Option<u64>,
    #[serde(default)]
    pub created_at: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserForumSubscribedThread {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub forum_id: Option<u64>,
    #[serde(default)]
    pub author: Option<ForumUserSummary>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub posts: Option<UserForumSubscribedThreadPosts>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserForumSubscribedThreadPosts {
    #[serde(default, rename = "new")]
    pub new_posts: Option<u64>,
    #[serde(default)]
    pub total: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserForumFeedEntry {
    #[serde(default)]
    pub thread_id: Option<u64>,
    #[serde(default)]
    pub post_id: Option<u64>,
    #[serde(default)]
    pub user: Option<ForumUserSummary>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub timestamp: Option<u64>,
    #[serde(default)]
    pub is_seen: Option<bool>,
    #[serde(default, rename = "type")]
    pub entry_type: Option<i64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserWorkStats {
    #[serde(default)]
    pub endurance: Option<u64>,
    #[serde(default)]
    pub intelligence: Option<u64>,
    #[serde(default)]
    pub manual_labor: Option<u64>,
    #[serde(default)]
    pub total: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserMerits {
    #[serde(default)]
    pub upgrades: Vec<UserMeritUpgrade>,
    #[serde(default)]
    pub available: Option<u64>,
    #[serde(default)]
    pub used: Option<u64>,
    #[serde(default)]
    pub medals: Option<u64>,
    #[serde(default)]
    pub honors: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserMeritUpgrade {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub level: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserItemMarketListing {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub price: Option<i64>,
    #[serde(default)]
    pub average_price: Option<i64>,
    #[serde(default)]
    pub amount: Option<u64>,
    #[serde(default)]
    pub is_anonymous: Option<bool>,
    #[serde(default)]
    pub available: Option<u64>,
    #[serde(default)]
    pub item: Option<UserItemMarketItemDetails>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserItemMarketItemDetails {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default, rename = "type")]
    pub item_type: Option<String>,
    #[serde(default)]
    pub rarity: Option<String>,
    #[serde(default)]
    pub uid: Option<u64>,
    #[serde(default)]
    pub stats: Option<UserItemMarketItemStats>,
    #[serde(default)]
    pub bonuses: Vec<UserItemMarketItemBonus>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserItemMarketItemStats {
    #[serde(default)]
    pub damage: Option<f64>,
    #[serde(default)]
    pub accuracy: Option<f64>,
    #[serde(default)]
    pub armor: Option<f64>,
    #[serde(default)]
    pub quality: Option<f64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserItemMarketItemBonus {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub value: Option<i64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone)]
pub enum UserOrganizedCrimeSelection {
    Crime(UserOrganizedCrime),
    Error(UserOrganizedCrimeError),
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserOrganizedCrime {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub previous_crime_id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub difficulty: Option<u64>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub created_at: Option<u64>,
    #[serde(default)]
    pub planning_at: Option<u64>,
    #[serde(default)]
    pub ready_at: Option<u64>,
    #[serde(default)]
    pub expired_at: Option<u64>,
    #[serde(default)]
    pub executed_at: Option<u64>,
    #[serde(default)]
    pub slots: Vec<UserOrganizedCrimeSlot>,
    #[serde(default)]
    pub rewards: Option<Value>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserOrganizedCrimeSlot {
    #[serde(default)]
    pub position: Option<String>,
    #[serde(default)]
    pub position_info: Option<UserOrganizedCrimePositionInfo>,
    #[serde(default)]
    pub position_id: Option<String>,
    #[serde(default)]
    pub position_number: Option<u64>,
    #[serde(default)]
    pub item_requirement: Option<UserOrganizedCrimeItemRequirement>,
    #[serde(default)]
    pub user: Option<UserOrganizedCrimeUser>,
    #[serde(default)]
    pub checkpoint_pass_rate: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserOrganizedCrimePositionInfo {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub number: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserOrganizedCrimeItemRequirement {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub is_reusable: Option<bool>,
    #[serde(default)]
    pub is_available: Option<bool>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserOrganizedCrimeUser {
    #[serde(default)]
    pub outcome: Option<String>,
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub joined_at: Option<u64>,
    #[serde(default)]
    pub progress: Option<u64>,
    #[serde(default)]
    pub item_outcome: Option<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserOrganizedCrimeError {
    #[serde(default)]
    pub code: Option<u64>,
    #[serde(default)]
    pub error: Option<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
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

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserTrade {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub timestamp: UserTradeTimestamp,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub user: Option<UserTradeParticipant>,
    #[serde(default)]
    pub trader: Option<UserTradeParticipant>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserTradeDetailed {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub timestamp: UserTradeTimestamp,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default, rename = "type")]
    pub trade_type: Option<String>,
    #[serde(default)]
    pub user: Option<UserTradeParticipant>,
    #[serde(default)]
    pub trader: Option<UserTradeParticipant>,
    #[serde(default)]
    pub items: Vec<TradeItem>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserTradeParticipant {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(untagged)]
pub enum UserTradeTimestamp {
    #[default]
    Missing,
    Value(u64),
    Unavailable(bool),
    Raw(Value),
}

#[derive(Debug, Clone)]
pub enum TradeItem {
    Money(TradeMoneyItem),
    Inventory(TradeInventoryItem),
    Faction(TradeFactionItem),
    Company(TradeCompanyItem),
    Property(TradePropertyItem),
    Nap(TradeNapItem),
    Unknown(UnknownTradeItem),
}

#[derive(Debug, Clone, Default)]
pub struct TradeMoneyItem {
    pub user_id: Option<u64>,
    pub details: TradeMoneyDetails,
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Default)]
pub struct TradeInventoryItem {
    pub user_id: Option<u64>,
    pub details: TradeInventoryItemDetails,
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Default)]
pub struct TradeFactionItem {
    pub user_id: Option<u64>,
    pub details: TradeFactionDetails,
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Default)]
pub struct TradeCompanyItem {
    pub user_id: Option<u64>,
    pub details: TradeCompanyDetails,
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Default)]
pub struct TradePropertyItem {
    pub user_id: Option<u64>,
    pub details: TradePropertyDetails,
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Default)]
pub struct TradeNapItem {
    pub user_id: Option<u64>,
    pub details: TradeNapDetails,
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Default)]
pub struct UnknownTradeItem {
    pub user_id: Option<u64>,
    pub item_type: Option<String>,
    pub details: Option<Value>,
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TradeMoneyDetails {
    #[serde(default)]
    pub amount: Option<i64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TradeInventoryItemDetails {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub uid: Option<u64>,
    #[serde(default)]
    pub amount: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TradeFactionDetails {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub respect: Option<i64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TradeCompanyDetails {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub value: Option<i64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TradePropertyDetails {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub happiness: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TradeNapDetails {
    #[serde(default)]
    pub days: Option<u64>,
    #[serde(default)]
    pub factions: Vec<TradeNapFaction>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TradeNapFaction {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
struct RawTradeItemEnvelope {
    #[serde(default)]
    user_id: Option<u64>,
    #[serde(default, rename = "type")]
    item_type: Option<String>,
    #[serde(default)]
    details: Option<Value>,
    #[serde(flatten, default)]
    extra: BTreeMap<String, Value>,
}

impl RawTradeItemEnvelope {
    fn into_unknown(self) -> UnknownTradeItem {
        UnknownTradeItem {
            user_id: self.user_id,
            item_type: self.item_type,
            details: self.details,
            extra: self.extra,
        }
    }
}

impl<'de> serde::Deserialize<'de> for TradeItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw = RawTradeItemEnvelope::deserialize(deserializer)?;

        match raw.item_type.as_deref() {
            Some("Money") => Ok(decode_trade_item(raw, |value| {
                TradeItem::Money(value.into())
            })),
            Some("Item") => Ok(decode_trade_item(raw, |value| {
                TradeItem::Inventory(value.into())
            })),
            Some("Faction") => Ok(decode_trade_item(raw, |value| {
                TradeItem::Faction(value.into())
            })),
            Some("Company") => Ok(decode_trade_item(raw, |value| {
                TradeItem::Company(value.into())
            })),
            Some("Property") => Ok(decode_trade_item(raw, |value| {
                TradeItem::Property(value.into())
            })),
            Some("NAP") => Ok(decode_trade_item(raw, |value| TradeItem::Nap(value.into()))),
            _ => Ok(TradeItem::Unknown(raw.into_unknown())),
        }
    }
}

impl<'de> serde::Deserialize<'de> for UserEmployment {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        let job_type = value
            .as_object()
            .and_then(|object| object.get("type"))
            .and_then(Value::as_str);

        match job_type {
            Some("job") => decode_user_union(value, UserEmployment::Job),
            Some("company") => decode_user_union(value, UserEmployment::Company),
            _ => Ok(UserEmployment::Unknown(unknown_user_employment(value))),
        }
    }
}

impl<'de> serde::Deserialize<'de> for UserCompetition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        let name = value
            .as_object()
            .and_then(|object| object.get("name"))
            .and_then(Value::as_str);

        match name {
            Some("Halloween") => decode_user_union(value, UserCompetition::Halloween),
            Some("Easter Egg Hunt") => decode_user_union(value, UserCompetition::EasterEggs),
            Some("Rock, Paper, Scissors") => {
                decode_user_union(value, UserCompetition::RockPaperScissors)
            }
            Some("Elimination") => decode_user_union(value, UserCompetition::Elimination),
            _ => Ok(UserCompetition::Unknown(unknown_user_competition(value))),
        }
    }
}

impl<'de> serde::Deserialize<'de> for UserOrganizedCrimeSelection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        if value.get("code").is_some() && value.get("error").is_some() {
            decode_user_union(value, UserOrganizedCrimeSelection::Error)
        } else {
            decode_user_union(value, UserOrganizedCrimeSelection::Crime)
        }
    }
}

fn decode_trade_item<T, F>(raw: RawTradeItemEnvelope, wrap: F) -> TradeItem
where
    T: serde::de::DeserializeOwned,
    F: FnOnce(TradeItemCore<T>) -> TradeItem,
{
    let details = raw.details.clone().unwrap_or(Value::Null);
    match serde_json::from_value::<T>(details) {
        Ok(details) => wrap(TradeItemCore {
            user_id: raw.user_id,
            details,
            extra: raw.extra,
        }),
        Err(_) => TradeItem::Unknown(raw.into_unknown()),
    }
}

fn decode_user_union<T, U, F, E>(value: Value, wrap: F) -> Result<U, E>
where
    T: serde::de::DeserializeOwned,
    F: FnOnce(T) -> U,
    E: serde::de::Error,
{
    serde_json::from_value::<T>(value)
        .map(wrap)
        .map_err(serde::de::Error::custom)
}

fn unknown_user_employment(value: Value) -> UnknownUserEmployment {
    let job_type = value
        .as_object()
        .and_then(|object| object.get("type"))
        .and_then(Value::as_str)
        .map(ToOwned::to_owned);

    UnknownUserEmployment {
        job_type,
        extra: value_to_extra_map(value),
    }
}

fn unknown_user_competition(value: Value) -> UnknownUserCompetition {
    let name = value
        .as_object()
        .and_then(|object| object.get("name"))
        .and_then(Value::as_str)
        .map(ToOwned::to_owned);

    UnknownUserCompetition {
        name,
        extra: value_to_extra_map(value),
    }
}

fn value_to_extra_map(value: Value) -> BTreeMap<String, Value> {
    match value {
        Value::Object(object) => object.into_iter().collect(),
        other => {
            let mut extra = BTreeMap::new();
            extra.insert("raw".to_string(), other);
            extra
        }
    }
}

#[derive(Debug, Clone, Default)]
struct TradeItemCore<T> {
    user_id: Option<u64>,
    details: T,
    extra: BTreeMap<String, Value>,
}

impl From<TradeItemCore<TradeMoneyDetails>> for TradeMoneyItem {
    fn from(value: TradeItemCore<TradeMoneyDetails>) -> Self {
        Self {
            user_id: value.user_id,
            details: value.details,
            extra: value.extra,
        }
    }
}

impl From<TradeItemCore<TradeInventoryItemDetails>> for TradeInventoryItem {
    fn from(value: TradeItemCore<TradeInventoryItemDetails>) -> Self {
        Self {
            user_id: value.user_id,
            details: value.details,
            extra: value.extra,
        }
    }
}

impl From<TradeItemCore<TradeFactionDetails>> for TradeFactionItem {
    fn from(value: TradeItemCore<TradeFactionDetails>) -> Self {
        Self {
            user_id: value.user_id,
            details: value.details,
            extra: value.extra,
        }
    }
}

impl From<TradeItemCore<TradeCompanyDetails>> for TradeCompanyItem {
    fn from(value: TradeItemCore<TradeCompanyDetails>) -> Self {
        Self {
            user_id: value.user_id,
            details: value.details,
            extra: value.extra,
        }
    }
}

impl From<TradeItemCore<TradePropertyDetails>> for TradePropertyItem {
    fn from(value: TradeItemCore<TradePropertyDetails>) -> Self {
        Self {
            user_id: value.user_id,
            details: value.details,
            extra: value.extra,
        }
    }
}

impl From<TradeItemCore<TradeNapDetails>> for TradeNapItem {
    fn from(value: TradeItemCore<TradeNapDetails>) -> Self {
        Self {
            user_id: value.user_id,
            details: value.details,
            extra: value.extra,
        }
    }
}
