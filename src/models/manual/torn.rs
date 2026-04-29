use std::collections::BTreeMap;

use serde::Deserialize;
use serde_json::Value;

use crate::models::manual::common::{PaginatedMetadata, PaginatedMetadataWithTotal};

#[derive(Debug, Clone, Deserialize)]
pub struct TornCalendarBundle {
    pub calendar: TornCalendar,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TornItemsBundle {
    #[serde(default)]
    pub items: Vec<TornItemSummary>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TornHonorsBundle {
    #[serde(default)]
    pub honors: Vec<TornHonorSummary>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TornMedalsBundle {
    #[serde(default)]
    pub medals: Vec<TornMedalSummary>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TornLogCategoriesBundle {
    #[serde(default)]
    pub logcategories: Vec<TornLogCategory>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TornLogTypesBundle {
    #[serde(default)]
    pub logtypes: Vec<TornLogType>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TornMeritsBundle {
    #[serde(default)]
    pub merits: Vec<TornMerit>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TornItemAmmoBundle {
    #[serde(default)]
    pub itemammo: Vec<TornAmmoItem>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TornItemModsBundle {
    #[serde(default)]
    pub itemmods: Vec<TornItemMod>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TornPropertiesBundle {
    #[serde(default)]
    pub properties: Vec<TornPropertyCatalogEntry>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TornStocksBundle {
    #[serde(default)]
    pub stocks: Vec<TornStockSummary>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TornStockBundle {
    #[serde(rename = "stocks")]
    pub stock: TornStockDetailed,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TornBountiesBundle {
    #[serde(default)]
    pub bounties: Vec<TornBounty>,
    #[serde(default)]
    pub bounties_timestamp: Option<u64>,
    #[serde(default)]
    pub bounties_delay: Option<u64>,
    #[serde(default)]
    pub _metadata: Option<PaginatedMetadataWithTotal>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TornAttackLogBundle {
    pub attacklog: TornAttackLog,
    #[serde(default)]
    pub _metadata: Option<PaginatedMetadata>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TornBankBundle {
    pub bank: TornBankRates,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TornCardsBundle {
    #[serde(default)]
    pub cards: BTreeMap<String, TornCard>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TornCityShopsBundle {
    #[serde(default)]
    pub cityshops: BTreeMap<String, TornCityShop>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TornCompaniesBundle {
    #[serde(default)]
    pub companies: BTreeMap<String, TornCompany>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TornGymsBundle {
    #[serde(default)]
    pub gyms: BTreeMap<String, TornGym>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TornPawnshopBundle {
    pub pawnshop: TornPawnshop,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TornPokerTablesBundle {
    #[serde(default)]
    pub pokertables: BTreeMap<String, TornPokerTable>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TornLookupBundle {
    #[serde(default)]
    pub selections: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TornTimestampBundle {
    #[serde(default)]
    pub timestamp: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TornCompetitionBundle {
    #[serde(default)]
    pub competition: Vec<Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TornCrimesBundle {
    #[serde(default)]
    pub crimes: Vec<Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TornEducationBundle {
    #[serde(default)]
    pub education: Vec<Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TornEliminationBundle {
    #[serde(default)]
    pub elimination: Option<Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TornEliminationTeamBundle {
    #[serde(default)]
    pub eliminationteam: Vec<TornEliminationTeamEntry>,
    #[serde(default)]
    pub _metadata: Option<PaginatedMetadata>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TornFactionHofBundle {
    #[serde(default)]
    pub factionhof: Option<Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TornFactionTreeBundle {
    #[serde(default)]
    pub factiontree: Option<Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TornHofBundle {
    #[serde(default)]
    pub hof: Option<Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TornOrganisedCrimesBundle {
    #[serde(default)]
    pub organisedcrimes: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TornOrganizedCrimesBundle {
    #[serde(default)]
    pub organizedcrimes: Vec<Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TornRockPaperScissorsBundle {
    #[serde(default)]
    pub rockpaperscissors: Vec<TornCountEntry>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TornSearchForCashBundle {
    #[serde(default)]
    pub searchforcash: BTreeMap<String, TornPercentageEntry>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TornShopliftingBundle {
    #[serde(default)]
    pub shoplifting: BTreeMap<String, Vec<TornShopliftingEntry>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TornStatsBundle {
    #[serde(default)]
    pub stats: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TornSubcrimesBundle {
    #[serde(default)]
    pub subcrimes: Option<Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TornTerritoryBundle {
    #[serde(default)]
    pub territory: Option<Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TornItemDetailsBundle {
    #[serde(default)]
    pub itemdetails: Option<Value>,
    #[serde(default)]
    pub _metadata: Option<PaginatedMetadata>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TornItemStatsBundle {
    pub itemstats: TornItemStats,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TornEliminationTeamEntry {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub rank: Option<u64>,
    #[serde(default)]
    pub score: Option<f64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TornCalendar {
    #[serde(default)]
    pub competitions: Vec<TornCalendarEntry>,
    #[serde(default)]
    pub events: Vec<TornCalendarEntry>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TornCalendarEntry {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub start: Option<u64>,
    #[serde(default)]
    pub end: Option<u64>,
    #[serde(default)]
    pub fixed_start_time: Option<bool>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TornItemSummary {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default, rename = "type")]
    pub item_type: Option<String>,
    #[serde(default)]
    pub circulation: Option<u64>,
    #[serde(default)]
    pub is_tradable: Option<bool>,
    #[serde(default)]
    pub is_found_in_city: Option<bool>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub details: Option<TornItemDetails>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TornItemDetails {
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default)]
    pub base_stats: Option<TornItemBaseStats>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TornItemBaseStats {
    #[serde(default)]
    pub damage: Option<f64>,
    #[serde(default)]
    pub accuracy: Option<f64>,
    #[serde(default)]
    pub armor: Option<f64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TornHonorSummary {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub rarity: Option<String>,
    #[serde(default)]
    pub circulation: Option<u64>,
    #[serde(default)]
    pub equipped: Option<bool>,
    #[serde(default, rename = "type")]
    pub honor_type: Option<TornAwardType>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TornMedalSummary {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub rarity: Option<String>,
    #[serde(default)]
    pub circulation: Option<u64>,
    #[serde(default, rename = "type")]
    pub medal_type: Option<TornAwardType>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TornAwardType {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TornLogCategory {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TornLogType {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TornMerit {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TornAmmoItem {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub price: Option<u64>,
    #[serde(default)]
    pub types: Vec<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TornItemMod {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub dual_fit: Option<bool>,
    #[serde(default)]
    pub weapons: Vec<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TornPropertyCatalogEntry {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub cost: Option<u64>,
    #[serde(default)]
    pub happy: Option<u64>,
    #[serde(default)]
    pub upkeep: Option<u64>,
    #[serde(default, alias = "upgrades_available")]
    pub modifications: Vec<String>,
    #[serde(default, alias = "staff_available")]
    pub staff: Vec<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TornStockSummary {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub acronym: Option<String>,
    #[serde(default)]
    pub images: Option<TornStockImages>,
    #[serde(default)]
    pub market: Option<TornStockMarket>,
    #[serde(default)]
    pub bonus: Option<TornStockBonus>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TornStockDetailed {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub acronym: Option<String>,
    #[serde(default)]
    pub images: Option<TornStockImages>,
    #[serde(default)]
    pub market: Option<TornStockMarket>,
    #[serde(default)]
    pub bonus: Option<TornStockBonus>,
    #[serde(default)]
    pub chart: Option<TornStockChart>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TornStockImages {
    #[serde(default)]
    pub logo: Option<String>,
    #[serde(default)]
    pub full: Option<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TornStockMarket {
    #[serde(default)]
    pub price: Option<f64>,
    #[serde(default)]
    pub cap: Option<u64>,
    #[serde(default)]
    pub shares: Option<u64>,
    #[serde(default)]
    pub investors: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TornStockBonus {
    #[serde(default)]
    pub passive: Option<bool>,
    #[serde(default)]
    pub frequency: Option<u64>,
    #[serde(default)]
    pub requirement: Option<u64>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TornStockChart {
    #[serde(default)]
    pub performance: Option<TornStockPerformance>,
    #[serde(default)]
    pub history: Vec<TornStockHistoryEntry>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TornStockPerformance {
    #[serde(default)]
    pub last_hour: Option<TornStockPerformanceWindow>,
    #[serde(default)]
    pub last_day: Option<TornStockPerformanceWindow>,
    #[serde(default)]
    pub last_week: Option<TornStockPerformanceWindow>,
    #[serde(default)]
    pub last_month: Option<TornStockPerformanceWindow>,
    #[serde(default)]
    pub last_year: Option<TornStockPerformanceWindow>,
    #[serde(default)]
    pub all_time: Option<TornStockPerformanceWindow>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TornStockPerformanceWindow {
    #[serde(default)]
    pub change: Option<f64>,
    #[serde(default)]
    pub change_percentage: Option<f64>,
    #[serde(default)]
    pub start: Option<f64>,
    #[serde(default)]
    pub end: Option<f64>,
    #[serde(default)]
    pub high: Option<f64>,
    #[serde(default)]
    pub low: Option<f64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TornStockHistoryEntry {
    #[serde(default)]
    pub timestamp: Option<u64>,
    #[serde(default)]
    pub price: Option<f64>,
    #[serde(default)]
    pub change: Option<f64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TornBounty {
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
    pub reward: Option<u64>,
    #[serde(default)]
    pub reason: Option<String>,
    #[serde(default)]
    pub quantity: Option<u64>,
    #[serde(default)]
    pub is_anonymous: Option<bool>,
    #[serde(default)]
    pub valid_until: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TornAttackLog {
    #[serde(default)]
    pub log: Vec<TornAttackLogEntry>,
    #[serde(default)]
    pub summary: Vec<TornAttackLogSummary>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TornAttackLogSummary {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub hits: Option<u64>,
    #[serde(default)]
    pub misses: Option<u64>,
    #[serde(default)]
    pub damage: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TornAttackLogEntry {
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub timestamp: Option<u64>,
    #[serde(default)]
    pub action: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub attacker: Option<TornAttackParticipant>,
    #[serde(default)]
    pub defender: Option<TornAttackParticipant>,
    #[serde(default)]
    pub attacker_item: Option<TornAttackItem>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TornAttackParticipant {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub item: Option<TornAttackItem>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TornAttackItem {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TornBankRates {
    #[serde(default, rename = "1w")]
    pub one_week: Option<f64>,
    #[serde(default, rename = "2w")]
    pub two_weeks: Option<f64>,
    #[serde(default, rename = "1m")]
    pub one_month: Option<f64>,
    #[serde(default, rename = "2m")]
    pub two_months: Option<f64>,
    #[serde(default, rename = "3m")]
    pub three_months: Option<f64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TornCard {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub short: Option<TornCardShort>,
    #[serde(default)]
    pub rank: Option<u64>,
    #[serde(default, rename = "class")]
    pub css_class: Option<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum TornCardShort {
    Number(u64),
    Label(String),
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TornCityShop {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub inventory: BTreeMap<String, TornCityShopItem>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TornCityShopItem {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default, rename = "type")]
    pub item_type: Option<String>,
    #[serde(default)]
    pub price: Option<u64>,
    #[serde(default)]
    pub in_stock: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TornCompany {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub cost: Option<u64>,
    #[serde(default)]
    pub default_employees: Option<u64>,
    #[serde(default)]
    pub positions: BTreeMap<String, TornCompanyPosition>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TornCompanyPosition {
    #[serde(default)]
    pub man_required: Option<u64>,
    #[serde(default)]
    pub int_required: Option<u64>,
    #[serde(default)]
    pub end_required: Option<u64>,
    #[serde(default)]
    pub man_gain: Option<i64>,
    #[serde(default)]
    pub int_gain: Option<i64>,
    #[serde(default)]
    pub end_gain: Option<i64>,
    #[serde(default)]
    pub special_ability: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TornGym {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub stage: Option<u64>,
    #[serde(default)]
    pub cost: Option<u64>,
    #[serde(default)]
    pub energy: Option<u64>,
    #[serde(default)]
    pub strength: Option<i64>,
    #[serde(default)]
    pub speed: Option<i64>,
    #[serde(default)]
    pub defense: Option<i64>,
    #[serde(default)]
    pub dexterity: Option<i64>,
    #[serde(default)]
    pub note: Option<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TornPawnshop {
    #[serde(default)]
    pub points_value: Option<u64>,
    #[serde(default)]
    pub donatorpack_value: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TornPokerTable {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub big_blind: Option<u64>,
    #[serde(default)]
    pub small_blind: Option<u64>,
    #[serde(default)]
    pub speed: Option<u64>,
    #[serde(default)]
    pub current_players: Option<u64>,
    #[serde(default)]
    pub maximum_players: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TornItemStats {
    #[serde(default, rename = "armoryID", alias = "armory_id")]
    pub armory_id: Option<u64>,
    #[serde(default, rename = "ID", alias = "id")]
    pub id: Option<u64>,
    #[serde(default)]
    pub sell: Option<u64>,
    #[serde(default, rename = "Qty", alias = "qty")]
    pub qty: Option<u64>,
    #[serde(default, rename = "SellTotal", alias = "sell_total")]
    pub sell_total: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default, alias = "sub_type")]
    pub type2: Option<String>,
    #[serde(default)]
    pub dmg: Option<f64>,
    #[serde(default)]
    pub damage: Option<f64>,
    #[serde(default)]
    pub accuracy: Option<f64>,
    #[serde(default)]
    pub acc: Option<f64>,
    #[serde(default)]
    pub arm: Option<f64>,
    #[serde(default, rename = "weptype", alias = "weapon_type")]
    pub weptype: Option<String>,
    #[serde(default, rename = "UID", alias = "uid")]
    pub uid: Option<u64>,
    #[serde(default, rename = "type")]
    pub item_type: Option<String>,
    #[serde(default, rename = "originalType", alias = "original_type")]
    pub original_type: Option<String>,
    #[serde(default)]
    pub market_price: Option<u64>,
    #[serde(default)]
    pub stats: Vec<Value>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TornCountEntry {
    #[serde(default, rename = "type")]
    pub entry_type: Option<String>,
    #[serde(default)]
    pub count: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TornPercentageEntry {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub percentage: Option<f64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TornShopliftingEntry {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub disabled: Option<bool>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}
