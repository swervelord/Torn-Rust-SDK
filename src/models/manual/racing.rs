use std::collections::BTreeMap;

use serde::Deserialize;

use super::common::PaginatedMetadata;

#[derive(Debug, Clone, Deserialize)]
pub struct RacingCarsBundle {
    #[serde(default)]
    pub cars: Vec<RacingCar>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RacingCarUpgradesBundle {
    #[serde(default)]
    pub carupgrades: Vec<RacingCarUpgrade>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RacingLookupBundle {
    #[serde(default)]
    pub selections: Vec<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RacingRaceBundle {
    pub race: RacingRace,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RacingRacesBundle {
    #[serde(default)]
    pub races: Vec<RacingRace>,
    #[serde(default)]
    pub _metadata: Option<PaginatedMetadata>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RacingRecordsBundle {
    #[serde(default)]
    pub records: Vec<RacingRecord>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RacingTimestampBundle {
    #[serde(default)]
    pub timestamp: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RacingTracksBundle {
    #[serde(default)]
    pub tracks: Vec<RacingTrack>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RacingCar {
    #[serde(default)]
    pub car_item_id: Option<u64>,
    #[serde(default)]
    pub car_item_name: Option<String>,
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
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RacingCarUpgrade {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub class_required: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default)]
    pub subcategory: Option<String>,
    #[serde(default)]
    pub effects: Option<RacingCarUpgradeEffects>,
    #[serde(default)]
    pub cost: Option<RacingCarUpgradeCost>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RacingCarUpgradeCost {
    #[serde(default)]
    pub points: Option<i64>,
    #[serde(default)]
    pub cash: Option<i64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RacingCarUpgradeEffects {
    #[serde(default)]
    pub top_speed: Option<i64>,
    #[serde(default)]
    pub acceleration: Option<i64>,
    #[serde(default)]
    pub braking: Option<i64>,
    #[serde(default)]
    pub handling: Option<i64>,
    #[serde(default)]
    pub safety: Option<i64>,
    #[serde(default)]
    pub dirt: Option<i64>,
    #[serde(default)]
    pub tarmac: Option<i64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RacingParticipants {
    #[serde(default)]
    pub minimum: Option<u32>,
    #[serde(default)]
    pub maximum: Option<u32>,
    #[serde(default)]
    pub current: Option<u32>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RacingRace {
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
    pub is_official: Option<bool>,
    #[serde(default)]
    pub laps: Option<u32>,
    #[serde(default)]
    pub participants: Option<RacingParticipants>,
    #[serde(default)]
    pub requirements: Option<RacingRequirements>,
    #[serde(default)]
    pub schedule: Option<RacingSchedule>,
    #[serde(default)]
    pub results: Vec<RacingResult>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RacingRecord {
    #[serde(default)]
    pub driver_id: Option<u64>,
    #[serde(default)]
    pub driver_name: Option<String>,
    #[serde(default)]
    pub car_item_id: Option<u64>,
    #[serde(default)]
    pub lap_time: Option<f64>,
    #[serde(default)]
    pub car_item_name: Option<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RacingRequirements {
    #[serde(default)]
    pub car_class: Option<String>,
    #[serde(default)]
    pub car_item_id: Option<u64>,
    #[serde(default)]
    pub driver_class: Option<String>,
    #[serde(default)]
    pub join_fee: Option<u64>,
    #[serde(default)]
    pub requires_password: Option<bool>,
    #[serde(default)]
    pub requires_stock_car: Option<bool>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RacingResult {
    #[serde(default)]
    pub position: Option<u32>,
    #[serde(default)]
    pub driver_id: Option<u64>,
    #[serde(default)]
    pub car_id: Option<u64>,
    #[serde(default)]
    pub car_item_id: Option<u64>,
    #[serde(default)]
    pub car_item_name: Option<String>,
    #[serde(default)]
    pub car_class: Option<String>,
    #[serde(default)]
    pub race_time: Option<f64>,
    #[serde(default)]
    pub best_lap_time: Option<f64>,
    #[serde(default)]
    pub has_crashed: Option<bool>,
    #[serde(default)]
    pub time_ended: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RacingSchedule {
    #[serde(default)]
    pub join_from: Option<u64>,
    #[serde(default)]
    pub join_until: Option<u64>,
    #[serde(default)]
    pub start: Option<u64>,
    #[serde(default)]
    pub end: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RacingTrack {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}
