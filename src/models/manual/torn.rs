use std::collections::BTreeMap;

use serde::Deserialize;

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
