use std::collections::BTreeMap;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct KeyInfoBundle {
    pub info: KeyInfo,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct KeyInfo {
    #[serde(default)]
    pub access: Option<KeyAccess>,
    #[serde(default)]
    pub selections: Option<KeySelections>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct KeyAccess {
    #[serde(default, rename = "type")]
    pub access_type: Option<String>,
    #[serde(default)]
    pub level: Option<u32>,
    #[serde(default)]
    pub company: Option<serde_json::Value>,
    #[serde(default)]
    pub faction: Option<serde_json::Value>,
    #[serde(default)]
    pub log: Option<KeyLogAccess>,
    #[serde(default)]
    pub custom_permissions: Option<serde_json::Value>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct KeyLogAccess {
    #[serde(default)]
    pub available: Vec<KeyLogCategoryAccess>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct KeyLogCategoryAccess {
    #[serde(default)]
    pub category_id: Option<u64>,
    #[serde(default)]
    pub log_ids: Vec<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct KeySelections {
    #[serde(default)]
    pub user: Vec<String>,
    #[serde(default)]
    pub faction: Vec<String>,
    #[serde(default)]
    pub forum: Vec<String>,
    #[serde(default)]
    pub key: Vec<String>,
    #[serde(default)]
    pub market: Vec<String>,
    #[serde(default)]
    pub property: Vec<String>,
    #[serde(default)]
    pub racing: Vec<String>,
    #[serde(default)]
    pub torn: Vec<String>,
    #[serde(default)]
    pub company: Vec<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}
