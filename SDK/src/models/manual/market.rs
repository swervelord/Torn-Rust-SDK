use std::collections::BTreeMap;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct MarketBazaarBundle {
    pub bazaar: MarketBazaar,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MarketItemMarketBundle {
    pub itemmarket: MarketItemMarket,
    #[serde(default)]
    pub _metadata: Option<MarketMetadata>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct MarketBazaar {
    #[serde(default)]
    pub advanced_item: Vec<MarketBazaarSellerSummary>,
    #[serde(default)]
    pub bargain: Vec<MarketBazaarSellerSummary>,
    #[serde(default)]
    pub bulk: Vec<MarketBazaarSellerSummary>,
    #[serde(default)]
    pub busiest: Vec<MarketBazaarSellerSummary>,
    #[serde(default)]
    pub dollar_sale: Vec<MarketBazaarSellerSummary>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct MarketBazaarSellerSummary {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub is_open: Option<bool>,
    #[serde(default)]
    pub weekly_customers: Option<u64>,
    #[serde(default)]
    pub advanced_item_sales: Option<u64>,
    #[serde(default)]
    pub bargain_sales: Option<u64>,
    #[serde(default)]
    pub bulk_sales: Option<u64>,
    #[serde(default)]
    pub dollar_sales: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct MarketItemMarket {
    #[serde(default)]
    pub cache_timestamp: Option<u64>,
    #[serde(default)]
    pub item: Option<MarketItemSummary>,
    #[serde(default)]
    pub listings: Vec<MarketListingSummary>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct MarketItemSummary {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default, rename = "type")]
    pub item_type: Option<String>,
    #[serde(default)]
    pub average_price: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct MarketListingSummary {
    #[serde(default)]
    pub amount: Option<u64>,
    #[serde(default)]
    pub price: Option<u64>,
    #[serde(default)]
    pub item_details: Option<MarketItemDetails>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct MarketItemDetails {
    #[serde(default)]
    pub uid: Option<u64>,
    #[serde(default)]
    pub rarity: Option<String>,
    #[serde(default)]
    pub stats: Option<MarketItemStats>,
    #[serde(default)]
    pub bonuses: Vec<MarketItemBonus>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct MarketItemStats {
    #[serde(default)]
    pub damage: Option<f64>,
    #[serde(default)]
    pub accuracy: Option<f64>,
    #[serde(default)]
    pub armor: Option<f64>,
    #[serde(default)]
    pub quality: Option<f64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct MarketItemBonus {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub value: Option<serde_json::Value>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct MarketMetadata {
    #[serde(default)]
    pub links: Option<MarketPaginationLinks>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct MarketPaginationLinks {
    #[serde(default)]
    pub prev: Option<String>,
    #[serde(default)]
    pub next: Option<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}
