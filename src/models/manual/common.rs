use std::collections::BTreeMap;

use serde::Deserialize;

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

#[derive(Debug, Clone, Deserialize, Default)]
pub struct PaginatedMetadataWithTotal {
    #[serde(default)]
    pub links: Option<PaginatedLinks>,
    #[serde(default)]
    pub total: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}
