use std::collections::BTreeMap;

use serde::Deserialize;

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
pub struct PaginatedMetadata {
    #[serde(default)]
    pub links: Option<PaginatedLinks>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct PaginatedLinks {
    #[serde(default)]
    pub prev: Option<String>,
    #[serde(default)]
    pub next: Option<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}
