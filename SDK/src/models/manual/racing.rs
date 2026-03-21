use std::collections::BTreeMap;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct RacingRaceBundle {
    pub race: RacingRace,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RacingRace {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub creator_id: Option<u64>,
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
pub struct RacingSchedule {
    #[serde(default)]
    pub start: Option<u64>,
    #[serde(default)]
    pub end: Option<u64>,
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
    pub race_time: Option<u64>,
    #[serde(default)]
    pub best_lap_time: Option<u64>,
    #[serde(default)]
    pub has_crashed: Option<bool>,
    #[serde(default)]
    pub time_ended: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}
