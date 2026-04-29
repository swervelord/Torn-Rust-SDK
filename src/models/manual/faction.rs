use std::collections::BTreeMap;

pub use super::common::{PaginatedLinks, PaginatedMetadata};
use super::user::{UserAttack, UserAttackFull, UserReportSummary, UserRevive, UserReviveFull};
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Clone, Deserialize)]
pub struct FactionBasicBundle {
    pub basic: FactionBasic,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FactionMembersBundle {
    #[serde(default)]
    pub members: Vec<FactionMemberSummary>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FactionWarsBundle {
    pub wars: FactionWarsSummary,
    #[serde(default)]
    pub pacts: Vec<FactionPactSummary>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FactionRankedWarsBundle {
    #[serde(default)]
    pub rankedwars: Vec<FactionRankedWarSummary>,
    #[serde(default)]
    pub _metadata: Option<PaginatedMetadata>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FactionApplicationsBundle {
    #[serde(default)]
    pub applications: Vec<FactionApplicationSummary>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FactionArmorBundle {
    #[serde(default)]
    pub armor: FactionLegacyItemSelection,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FactionBoostersBundle {
    #[serde(default)]
    pub boosters: FactionLegacyItemSelection,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FactionCachesBundle {
    #[serde(default)]
    pub caches: FactionLegacyItemSelection,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FactionCesiumBundle {
    #[serde(default)]
    pub cesium: FactionLegacyItemSelection,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FactionCrimeBundle {
    pub crime: FactionCrime,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FactionCrimeExpBundle {
    #[serde(default)]
    pub crimeexp: FactionCrimeExpSelection,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FactionCrimesBundle {
    #[serde(default)]
    pub crimes: Vec<FactionCrime>,
    #[serde(default)]
    pub _metadata: Option<PaginatedMetadata>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FactionDrugsBundle {
    #[serde(default)]
    pub drugs: FactionLegacyItemSelection,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FactionLookupBundle {
    #[serde(default)]
    pub selections: Vec<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FactionChainBundle {
    pub chain: FactionOngoingChainSummary,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FactionChainsBundle {
    #[serde(default)]
    pub chains: Vec<FactionChainSummary>,
    #[serde(default)]
    pub _metadata: Option<PaginatedMetadata>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FactionChainReportBundle {
    pub chainreport: FactionChainReportSummary,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FactionBalanceBundle {
    pub balance: FactionBalanceSummary,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FactionContributorsBundle {
    #[serde(default)]
    pub contributors: Vec<FactionContributorSummary>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FactionMedicalBundle {
    #[serde(default)]
    pub medical: FactionLegacyItemSelection,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FactionRacketsBundle {
    #[serde(default)]
    pub rackets: Vec<FactionRacketSummary>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FactionHofBundle {
    pub hof: FactionHofSummary,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FactionPositionsBundle {
    #[serde(default)]
    pub positions: Vec<FactionPositionSummary>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FactionSearchBundle {
    #[serde(default)]
    pub search: Vec<FactionSearchSummary>,
    #[serde(default)]
    pub _metadata: Option<PaginatedMetadata>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FactionStatsBundle {
    #[serde(default)]
    pub stats: Vec<FactionStatSummary>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FactionRaidsBundle {
    #[serde(default)]
    pub raids: Vec<FactionRaidSummary>,
    #[serde(default)]
    pub _metadata: Option<PaginatedMetadata>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FactionTimestampBundle {
    #[serde(default)]
    pub timestamp: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FactionTemporaryBundle {
    #[serde(default)]
    pub temporary: FactionLegacyItemSelection,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FactionAttacksBundle {
    #[serde(default)]
    pub attacks: Vec<UserAttack>,
    #[serde(default)]
    pub _metadata: Option<PaginatedMetadata>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FactionAttacksFullBundle {
    #[serde(default)]
    pub attacks: Vec<UserAttackFull>,
    #[serde(default)]
    pub _metadata: Option<PaginatedMetadata>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FactionRevivesBundle {
    #[serde(default)]
    pub revives: Vec<UserRevive>,
    #[serde(default)]
    pub _metadata: Option<PaginatedMetadata>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FactionRevivesFullBundle {
    #[serde(default)]
    pub revives: Vec<UserReviveFull>,
    #[serde(default)]
    pub _metadata: Option<PaginatedMetadata>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FactionReportsBundle {
    #[serde(default)]
    pub reports: Vec<UserReportSummary>,
    #[serde(default)]
    pub _metadata: Option<PaginatedMetadata>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FactionTerritoryBundle {
    #[serde(default)]
    pub territory: Vec<FactionTerritorySummary>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FactionTerritoryOwnershipBundle {
    #[serde(default, rename = "territoryOwnership")]
    pub territory_ownership: Vec<FactionTerritoryOwnershipSummary>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FactionTerritoryWarsBundle {
    #[serde(default)]
    pub territorywars: Vec<FactionTerritoryWarHistoryEntry>,
    #[serde(default)]
    pub _metadata: Option<PaginatedMetadata>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FactionRaidReportBundle {
    pub raidreport: FactionRaidReportSummary,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FactionRankedWarReportBundle {
    pub rankedwarreport: FactionRankedWarReportSummary,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FactionTerritoryWarReportBundle {
    pub territorywarreport: FactionTerritoryWarReportSummary,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FactionWarfareBundle {
    #[serde(default)]
    pub warfare: Vec<FactionWarfareEntry>,
    #[serde(default)]
    pub _metadata: Option<PaginatedMetadata>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FactionUtilitiesBundle {
    #[serde(default)]
    pub utilities: FactionLegacyItemSelection,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FactionWeaponsBundle {
    #[serde(default)]
    pub weapons: FactionLegacyItemSelection,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FactionNewsBundle {
    #[serde(default)]
    pub news: Vec<FactionNewsEntry>,
    #[serde(default)]
    pub _metadata: Option<PaginatedMetadata>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FactionUpgradesBundle {
    #[serde(default)]
    pub state: Option<String>,
    #[serde(default)]
    pub upgrades: Option<FactionUpgradesSummary>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionBasic {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub tag: Option<String>,
    #[serde(default)]
    pub tag_image: Option<String>,
    #[serde(default)]
    pub respect: Option<i64>,
    #[serde(default)]
    pub members: Option<u32>,
    #[serde(default)]
    pub leader_id: Option<u64>,
    #[serde(default)]
    pub co_leader_id: Option<u64>,
    #[serde(default)]
    pub rank: Option<FactionRank>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionRank {
    #[serde(default)]
    pub division: Option<u32>,
    #[serde(default)]
    pub level: Option<u32>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub position: Option<u32>,
    #[serde(default)]
    pub wins: Option<u32>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionMemberSummary {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub level: Option<u32>,
    #[serde(default)]
    pub position: Option<String>,
    #[serde(default)]
    pub days_in_faction: Option<u32>,
    #[serde(default)]
    pub is_revivable: Option<bool>,
    #[serde(default)]
    pub status: Option<FactionMemberStatus>,
    #[serde(default)]
    pub last_action: Option<FactionLastAction>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionMemberStatus {
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
pub struct FactionLastAction {
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
pub struct FactionWarsSummary {
    #[serde(default)]
    pub raids: Vec<FactionRaidWarSummary>,
    #[serde(default)]
    pub ranked: Option<FactionRankedWarDetails>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionRaidWarSummary {
    #[serde(default, alias = "id")]
    pub war_id: Option<u64>,
    #[serde(default)]
    pub start: Option<u64>,
    #[serde(default)]
    pub end: Option<u64>,
    #[serde(default)]
    pub factions: Vec<FactionWarFactionSummary>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionRankedWarDetails {
    #[serde(default, alias = "id")]
    pub war_id: Option<u64>,
    #[serde(default)]
    pub target: Option<u64>,
    #[serde(default)]
    pub start: Option<u64>,
    #[serde(default)]
    pub end: Option<u64>,
    #[serde(default)]
    pub winner: Option<u64>,
    #[serde(default)]
    pub factions: Vec<FactionWarFactionSummary>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionWarFactionSummary {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub score: Option<f64>,
    #[serde(default)]
    pub chain: Option<u64>,
    #[serde(default)]
    pub is_aggressor: Option<bool>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionPactSummary {
    #[serde(default)]
    pub faction_id: Option<u64>,
    #[serde(default)]
    pub faction_name: Option<String>,
    #[serde(default)]
    pub until: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionRankedWarSummary {
    #[serde(default, alias = "war_id")]
    pub id: Option<u64>,
    #[serde(default)]
    pub start: Option<u64>,
    #[serde(default)]
    pub end: Option<u64>,
    #[serde(default)]
    pub target: Option<u64>,
    #[serde(default)]
    pub winner: Option<u64>,
    #[serde(default)]
    pub factions: Vec<FactionWarFactionSummary>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionApplicationSummary {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub user: Option<FactionApplicantSummary>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub valid_until: Option<u64>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionApplicantSummary {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub level: Option<u32>,
    #[serde(default)]
    pub stats: Option<FactionApplicantStats>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionApplicantStats {
    #[serde(default)]
    pub strength: Option<i64>,
    #[serde(default)]
    pub speed: Option<i64>,
    #[serde(default)]
    pub dexterity: Option<i64>,
    #[serde(default)]
    pub defense: Option<i64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionBalanceSummary {
    #[serde(default)]
    pub faction: Option<FactionBalanceFaction>,
    #[serde(default)]
    pub members: Vec<FactionBalanceMember>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionBalanceFaction {
    #[serde(default)]
    pub money: Option<i64>,
    #[serde(default)]
    pub points: Option<i64>,
    #[serde(default)]
    pub scope: Option<u32>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionBalanceMember {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub money: Option<i64>,
    #[serde(default)]
    pub points: Option<i64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionContributorSummary {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub value: Option<i64>,
    #[serde(default)]
    pub in_faction: Option<bool>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum FactionLegacyItemSelection {
    Map(BTreeMap<String, FactionLegacyItemEntry>),
    List(Vec<FactionLegacyItemEntry>),
    Unknown(Value),
}

impl Default for FactionLegacyItemSelection {
    fn default() -> Self {
        Self::Map(BTreeMap::new())
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionLegacyItemEntry {
    #[serde(default, rename = "ID", alias = "id")]
    pub id: Option<u64>,
    #[serde(default)]
    pub item_id: Option<u64>,
    #[serde(default)]
    pub uid: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default, rename = "type")]
    pub item_type: Option<String>,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default)]
    pub quantity: Option<i64>,
    #[serde(default)]
    pub available: Option<i64>,
    #[serde(default)]
    pub loaned: Option<i64>,
    #[serde(default)]
    pub loaned_to: Option<Value>,
    #[serde(default)]
    pub market_price: Option<i64>,
    #[serde(default)]
    pub average_price: Option<i64>,
    #[serde(default)]
    pub damage: Option<f64>,
    #[serde(default)]
    pub accuracy: Option<f64>,
    #[serde(default)]
    pub armor: Option<f64>,
    #[serde(default)]
    pub quality: Option<f64>,
    #[serde(default)]
    pub bonus: Option<String>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum FactionCrimeExpSelection {
    Map(BTreeMap<String, FactionCrimeExpEntry>),
    List(Vec<FactionCrimeExpEntry>),
    Unknown(Value),
}

impl Default for FactionCrimeExpSelection {
    fn default() -> Self {
        Self::Map(BTreeMap::new())
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionCrimeExpEntry {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default, rename = "crimeexp", alias = "crime_exp")]
    pub crime_exp: Option<i64>,
    #[serde(default)]
    pub position: Option<String>,
    #[serde(default)]
    pub rank: Option<i64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionHofSummary {
    #[serde(default)]
    pub rank: Option<FactionHofValueString>,
    #[serde(default)]
    pub respect: Option<FactionHofValue>,
    #[serde(default)]
    pub chain: Option<FactionHofValue>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionHofValueString {
    #[serde(default)]
    pub rank: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionHofValue {
    #[serde(default)]
    pub rank: Option<u64>,
    #[serde(default)]
    pub value: Option<i64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionPositionSummary {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub is_default: Option<bool>,
    #[serde(default)]
    pub abilities: Vec<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionSearchSummary {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub respect: Option<i64>,
    #[serde(default)]
    pub members: Option<u32>,
    #[serde(default)]
    pub leader: Option<FactionSearchLeader>,
    #[serde(default)]
    pub co_leader: Option<FactionSearchLeader>,
    #[serde(default)]
    pub image: Option<String>,
    #[serde(default)]
    pub tag_image: Option<String>,
    #[serde(default)]
    pub tag: Option<String>,
    #[serde(default)]
    pub is_destroyed: Option<bool>,
    #[serde(default)]
    pub is_recruiting: Option<bool>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionSearchLeader {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionStatSummary {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub value: Option<i64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionOngoingChainSummary {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub current: Option<u64>,
    #[serde(default)]
    pub max: Option<u64>,
    #[serde(default)]
    pub timeout: Option<u64>,
    #[serde(default)]
    pub modifier: Option<f64>,
    #[serde(default)]
    pub cooldown: Option<u64>,
    #[serde(default)]
    pub start: Option<u64>,
    #[serde(default)]
    pub end: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionChainSummary {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub chain: Option<u64>,
    #[serde(default)]
    pub respect: Option<f64>,
    #[serde(default)]
    pub start: Option<u64>,
    #[serde(default)]
    pub end: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionChainReportSummary {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub faction_id: Option<u64>,
    #[serde(default)]
    pub start: Option<u64>,
    #[serde(default)]
    pub end: Option<u64>,
    #[serde(default)]
    pub details: Option<FactionChainReportDetailsSummary>,
    #[serde(default)]
    pub bonuses: Vec<FactionChainReportBonusSummary>,
    #[serde(default)]
    pub attackers: Vec<FactionChainReportAttackerSummary>,
    #[serde(default)]
    pub non_attackers: Vec<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionChainReportDetailsSummary {
    #[serde(default)]
    pub chain: Option<u64>,
    #[serde(default)]
    pub respect: Option<f64>,
    #[serde(default)]
    pub members: Option<u64>,
    #[serde(default)]
    pub targets: Option<u64>,
    #[serde(default)]
    pub war: Option<u64>,
    #[serde(default)]
    pub best: Option<f64>,
    #[serde(default)]
    pub leave: Option<u64>,
    #[serde(default)]
    pub mug: Option<u64>,
    #[serde(default)]
    pub hospitalize: Option<u64>,
    #[serde(default)]
    pub assists: Option<u64>,
    #[serde(default)]
    pub retaliations: Option<u64>,
    #[serde(default)]
    pub overseas: Option<u64>,
    #[serde(default)]
    pub draws: Option<u64>,
    #[serde(default)]
    pub escapes: Option<u64>,
    #[serde(default)]
    pub losses: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionChainReportBonusSummary {
    #[serde(default)]
    pub attacker_id: Option<u64>,
    #[serde(default)]
    pub defender_id: Option<u64>,
    #[serde(default)]
    pub chain: Option<u64>,
    #[serde(default)]
    pub respect: Option<i64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionChainReportAttackerSummary {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub respect: Option<FactionChainReportAttackerRespectSummary>,
    #[serde(default)]
    pub attacks: Option<FactionChainReportAttacksSummary>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionChainReportAttackerRespectSummary {
    #[serde(default)]
    pub total: Option<f64>,
    #[serde(default)]
    pub average: Option<f64>,
    #[serde(default)]
    pub best: Option<f64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionChainReportAttacksSummary {
    #[serde(default)]
    pub total: Option<u64>,
    #[serde(default)]
    pub leave: Option<u64>,
    #[serde(default)]
    pub mug: Option<u64>,
    #[serde(default)]
    pub hospitalize: Option<u64>,
    #[serde(default)]
    pub assists: Option<u64>,
    #[serde(default)]
    pub retaliations: Option<u64>,
    #[serde(default)]
    pub overseas: Option<u64>,
    #[serde(default)]
    pub draws: Option<u64>,
    #[serde(default, alias = "escpaces")]
    pub escapes: Option<u64>,
    #[serde(default)]
    pub losses: Option<u64>,
    #[serde(default)]
    pub war: Option<u64>,
    #[serde(default)]
    pub bonuses: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionCrime {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub previous_crime_id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub difficulty: Option<u32>,
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
    pub slots: Vec<FactionCrimeSlot>,
    #[serde(default)]
    pub rewards: Option<FactionCrimeReward>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionCrimeSlot {
    #[serde(default)]
    pub position: Option<String>,
    #[serde(default)]
    pub position_info: Option<FactionCrimePositionInfo>,
    #[serde(default)]
    pub position_id: Option<String>,
    #[serde(default)]
    pub position_number: Option<u32>,
    #[serde(default)]
    pub item_requirement: Option<FactionCrimeItemRequirement>,
    #[serde(default)]
    pub user: Option<FactionCrimeUser>,
    #[serde(default)]
    pub checkpoint_pass_rate: Option<u32>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionCrimePositionInfo {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub number: Option<u32>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionCrimeItemRequirement {
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
pub struct FactionCrimeUser {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub outcome: Option<String>,
    #[serde(default)]
    pub outcome_duration: Option<u32>,
    #[serde(default)]
    pub item_outcome: Option<FactionCrimeUserItemOutcome>,
    #[serde(default)]
    pub joined_at: Option<u64>,
    #[serde(default)]
    pub progress: Option<f64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionCrimeUserItemOutcome {
    #[serde(default)]
    pub owned_by: Option<String>,
    #[serde(default)]
    pub item_id: Option<u64>,
    #[serde(default)]
    pub item_uid: Option<u64>,
    #[serde(default)]
    pub outcome: Option<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionCrimeReward {
    #[serde(default)]
    pub money: Option<i64>,
    #[serde(default)]
    pub items: Vec<FactionCrimeRewardItem>,
    #[serde(default)]
    pub respect: Option<i64>,
    #[serde(default)]
    pub scope: Option<u32>,
    #[serde(default)]
    pub payout: Option<FactionCrimeRewardPayout>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionCrimeRewardItem {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub quantity: Option<u32>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionCrimeRewardPayout {
    #[serde(default, rename = "type")]
    pub payout_type: Option<String>,
    #[serde(default)]
    pub percentage: Option<f64>,
    #[serde(default)]
    pub paid_by: Option<u64>,
    #[serde(default)]
    pub paid_at: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionRacketSummary {
    #[serde(default)]
    pub territory: Option<String>,
    #[serde(default)]
    pub faction_id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub level: Option<u64>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub reward: Option<FactionRacketRewardSummary>,
    #[serde(default)]
    pub created_at: Option<u64>,
    #[serde(default)]
    pub changed_at: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionRacketRewardSummary {
    #[serde(default, rename = "type")]
    pub reward_type: Option<String>,
    #[serde(default)]
    pub quantity: Option<i64>,
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionRaidSummary {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub start: Option<u64>,
    #[serde(default)]
    pub end: Option<u64>,
    #[serde(default)]
    pub aggressor: Option<FactionWarfareFactionSummary>,
    #[serde(default)]
    pub defender: Option<FactionWarfareFactionSummary>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionTerritorySummary {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub acquired_at: Option<u64>,
    #[serde(default)]
    pub sector: Option<u64>,
    #[serde(default)]
    pub size: Option<u64>,
    #[serde(default)]
    pub density: Option<u64>,
    #[serde(default)]
    pub slots: Option<u64>,
    #[serde(default)]
    pub respect: Option<u64>,
    #[serde(default)]
    pub coordinates: Option<FactionTerritoryCoordinatesSummary>,
    #[serde(default)]
    pub racket: Option<FactionRacketSummary>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionTerritoryCoordinatesSummary {
    #[serde(default)]
    pub x: Option<f64>,
    #[serde(default)]
    pub y: Option<f64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionTerritoryOwnershipSummary {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub owned_by: Option<u64>,
    #[serde(default)]
    pub acquired_at: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionTerritoryWarHistoryEntry {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub war_id: Option<u64>,
    #[serde(default)]
    pub territory: Option<String>,
    #[serde(default)]
    pub start: Option<u64>,
    #[serde(default)]
    pub end: Option<u64>,
    #[serde(default)]
    pub target: Option<u64>,
    #[serde(default)]
    pub result: Option<String>,
    #[serde(default)]
    pub factions: Vec<FactionTerritoryWarParticipantSummary>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionTerritoryWarParticipantSummary {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub score: Option<f64>,
    #[serde(default)]
    pub is_aggressor: Option<bool>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionChainWarfareSummary {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub chain: Option<u64>,
    #[serde(default)]
    pub respect: Option<f64>,
    #[serde(default)]
    pub start: Option<u64>,
    #[serde(default)]
    pub end: Option<u64>,
    #[serde(default)]
    pub faction: Option<FactionWarfareFactionSummary>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionTerritoryWarfareSummary {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub territory: Option<String>,
    #[serde(default)]
    pub start: Option<u64>,
    #[serde(default)]
    pub end: Option<u64>,
    #[serde(default)]
    pub target: Option<u64>,
    #[serde(default)]
    pub result: Option<String>,
    #[serde(default)]
    pub aggressor: Option<FactionWarfareFactionSummary>,
    #[serde(default)]
    pub defender: Option<FactionWarfareFactionSummary>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone)]
pub enum FactionWarfareEntry {
    Ranked(FactionRankedWarDetails),
    Territory(FactionTerritoryWarfareSummary),
    Chain(FactionChainWarfareSummary),
    Raid(FactionRaidSummary),
    Unknown(FactionUnknownWarfareEntry),
}

#[derive(Debug, Clone, Default)]
pub struct FactionUnknownWarfareEntry {
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionWarfareFactionSummary {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub score: Option<f64>,
    #[serde(default)]
    pub chain: Option<u64>,
    #[serde(default)]
    pub is_aggressor: Option<bool>,
    #[serde(default)]
    pub respect_lost: Option<i64>,
    #[serde(default)]
    pub players_on_wall: Vec<FactionWarfareWallPlayerSummary>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionWarfareWallPlayerSummary {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionWarfareUserSummary {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionNewsEntry {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub timestamp: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionRaidReportSummary {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub start: Option<u64>,
    #[serde(default)]
    pub end: Option<u64>,
    #[serde(default)]
    pub aggressor: Option<FactionRaidReportSideSummary>,
    #[serde(default)]
    pub defender: Option<FactionRaidReportSideSummary>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionRaidReportSideSummary {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub score: Option<f64>,
    #[serde(default)]
    pub attackers: Vec<FactionRaidReportAttackerSummary>,
    #[serde(default)]
    pub non_attackers: Vec<FactionRaidReportUserSummary>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionRaidReportAttackerSummary {
    #[serde(default)]
    pub user: Option<FactionRaidReportUserSummary>,
    #[serde(default)]
    pub attacks: Option<u64>,
    #[serde(default)]
    pub damage: Option<f64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionRaidReportUserSummary {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionRankedWarReportSummary {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub start: Option<u64>,
    #[serde(default)]
    pub end: Option<u64>,
    #[serde(default)]
    pub winner: Option<u64>,
    #[serde(default)]
    pub forfeit: Option<bool>,
    #[serde(default)]
    pub factions: Vec<FactionRankedWarReportFactionSummary>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionRankedWarReportFactionSummary {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub score: Option<f64>,
    #[serde(default)]
    pub attacks: Option<u64>,
    #[serde(default)]
    pub rank: Option<FactionRankedWarReportRankSummary>,
    #[serde(default)]
    pub rewards: Option<FactionRankedWarReportRewardsSummary>,
    #[serde(default)]
    pub members: Vec<FactionRankedWarReportMemberSummary>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionRankedWarReportRankSummary {
    #[serde(default)]
    pub before: Option<String>,
    #[serde(default)]
    pub after: Option<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionRankedWarReportRewardsSummary {
    #[serde(default)]
    pub respect: Option<f64>,
    #[serde(default)]
    pub points: Option<u64>,
    #[serde(default)]
    pub items: Vec<FactionRankedWarReportRewardItemSummary>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionRankedWarReportRewardItemSummary {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub quantity: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionRankedWarReportMemberSummary {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub level: Option<u64>,
    #[serde(default)]
    pub score: Option<f64>,
    #[serde(default)]
    pub attacks: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionTerritoryWarReportSummary {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub territory: Option<String>,
    #[serde(default)]
    pub started_at: Option<u64>,
    #[serde(default)]
    pub ended_at: Option<u64>,
    #[serde(default)]
    pub winner: Option<u64>,
    #[serde(default)]
    pub result: Option<String>,
    #[serde(default)]
    pub factions: Vec<FactionTerritoryWarReportFactionSummary>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionTerritoryWarReportFactionSummary {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub score: Option<f64>,
    #[serde(default)]
    pub is_aggressor: Option<bool>,
    #[serde(default)]
    pub clears: Option<u64>,
    #[serde(default)]
    pub joins: Option<u64>,
    #[serde(default)]
    pub members: Vec<FactionTerritoryWarReportMemberSummary>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionTerritoryWarReportMemberSummary {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub level: Option<u64>,
    #[serde(default)]
    pub score: Option<f64>,
    #[serde(default)]
    pub joins: Option<u64>,
    #[serde(default)]
    pub clears: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

impl<'de> serde::Deserialize<'de> for FactionWarfareEntry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        let Some(object) = value.as_object() else {
            return Ok(FactionWarfareEntry::Unknown(FactionUnknownWarfareEntry {
                extra: BTreeMap::new(),
            }));
        };

        if object.contains_key("territory") && object.contains_key("result") {
            return decode_faction_union(value, FactionWarfareEntry::Territory);
        }

        if object.contains_key("aggressor") && object.contains_key("defender") {
            return decode_faction_union(value, FactionWarfareEntry::Raid);
        }

        if object.contains_key("target")
            && object.contains_key("winner")
            && object.contains_key("factions")
        {
            return decode_faction_union(value, FactionWarfareEntry::Ranked);
        }

        if object.contains_key("chain") && object.contains_key("faction") {
            return decode_faction_union(value, FactionWarfareEntry::Chain);
        }

        Ok(FactionWarfareEntry::Unknown(FactionUnknownWarfareEntry {
            extra: value_to_extra_map(value),
        }))
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionUpgradesSummary {
    #[serde(default)]
    pub core: Option<FactionUpgradeCore>,
    #[serde(default)]
    pub peace: Vec<FactionUpgradeBranch>,
    #[serde(default)]
    pub war: Vec<FactionUpgradeBranch>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionUpgradeCore {
    #[serde(default)]
    pub upgrades: Vec<FactionUpgradeDetails>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionUpgradeBranch {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub order: Option<u32>,
    #[serde(default)]
    pub multiplier: Option<f64>,
    #[serde(default)]
    pub upgrades: Vec<FactionUpgradeDetails>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FactionUpgradeDetails {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub ability: Option<String>,
    #[serde(default)]
    pub level: Option<u32>,
    #[serde(default)]
    pub cost: Option<i64>,
    #[serde(default)]
    pub unlocked_at: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

fn decode_faction_union<T, U, E, F>(value: serde_json::Value, wrap: F) -> Result<U, E>
where
    T: serde::de::DeserializeOwned,
    F: FnOnce(T) -> U,
    E: serde::de::Error,
{
    serde_json::from_value::<T>(value)
        .map(wrap)
        .map_err(serde::de::Error::custom)
}

fn value_to_extra_map(value: serde_json::Value) -> BTreeMap<String, serde_json::Value> {
    value.as_object().map_or_else(BTreeMap::new, |object| {
        object
            .iter()
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect()
    })
}
