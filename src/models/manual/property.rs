use std::collections::BTreeMap;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct PropertyBundle {
    pub property: PropertyDetails,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct PropertyDetails {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub happy: Option<u64>,
    #[serde(default)]
    pub market_price: Option<u64>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub owner: Option<PropertyActor>,
    #[serde(default)]
    pub rented_by: Option<PropertyActor>,
    #[serde(default)]
    pub used_by: Vec<PropertyActor>,
    #[serde(default)]
    pub staff: Vec<PropertyStaffSummary>,
    #[serde(default)]
    pub property: Option<PropertyTypeSummary>,
    #[serde(default)]
    pub upkeep: Option<PropertyUpkeep>,
    #[serde(default)]
    pub modifications: Vec<serde_json::Value>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct PropertyActor {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct PropertyStaffSummary {
    #[serde(default, rename = "type")]
    pub staff_type: Option<String>,
    #[serde(default)]
    pub amount: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct PropertyTypeSummary {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct PropertyUpkeep {
    #[serde(default)]
    pub property: Option<u64>,
    #[serde(default)]
    pub staff: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}
