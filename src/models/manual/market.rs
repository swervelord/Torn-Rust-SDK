use std::collections::BTreeMap;

use serde::Deserialize;

use super::common::PaginatedMetadata;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct MarketLookupBundle {
    #[serde(default)]
    pub selections: Vec<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct MarketTimestampBundle {
    #[serde(default)]
    pub timestamp: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct MarketPointsMarketBundle {
    #[serde(default)]
    pub pointsmarket: BTreeMap<String, MarketPointsMarketListing>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct MarketPointsMarketListing {
    #[serde(default)]
    pub cost: Option<u64>,
    #[serde(default)]
    pub quantity: Option<u64>,
    #[serde(default)]
    pub total_cost: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

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

#[derive(Debug, Clone, Deserialize)]
pub struct MarketPropertiesBundle {
    pub properties: MarketPropertyCatalog,
    #[serde(default)]
    pub properties_timestamp: Option<u64>,
    #[serde(default)]
    pub properties_delay: Option<u64>,
    #[serde(default)]
    pub _metadata: Option<MarketMetadata>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MarketRentalsBundle {
    pub rentals: MarketRentalCatalog,
    #[serde(default)]
    pub rentals_timestamp: Option<u64>,
    #[serde(default)]
    pub rentals_delay: Option<u64>,
    #[serde(default)]
    pub _metadata: Option<MarketMetadata>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MarketAuctionHouseBundle {
    #[serde(default)]
    pub auctionhouse: Vec<MarketAuctionHouseListing>,
    #[serde(default)]
    pub _metadata: Option<MarketMetadata>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MarketAuctionHouseListingBundle {
    pub auctionhouselisting: MarketAuctionHouseListing,
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
    pub cache_delay: Option<u64>,
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
pub struct MarketPropertyCatalog {
    #[serde(default)]
    pub listings: Vec<MarketPropertyListing>,
    #[serde(default)]
    pub property: Option<MarketPropertyTypeSummary>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct MarketRentalCatalog {
    #[serde(default)]
    pub listings: Vec<MarketRentalListing>,
    #[serde(default)]
    pub property: Option<MarketPropertyTypeSummary>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct MarketPropertyListing {
    #[serde(default)]
    pub happy: Option<u64>,
    #[serde(default)]
    pub cost: Option<u64>,
    #[serde(default)]
    pub market_price: Option<u64>,
    #[serde(default)]
    pub upkeep: Option<u64>,
    #[serde(default)]
    pub modifications: Vec<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct MarketRentalListing {
    #[serde(default)]
    pub happy: Option<u64>,
    #[serde(default)]
    pub cost: Option<u64>,
    #[serde(default)]
    pub cost_per_day: Option<u64>,
    #[serde(default)]
    pub rental_period: Option<u64>,
    #[serde(default)]
    pub market_price: Option<u64>,
    #[serde(default)]
    pub upkeep: Option<u64>,
    #[serde(default)]
    pub modifications: Vec<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct MarketPropertyTypeSummary {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct MarketAuctionHouseListing {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub seller: Option<MarketUserSummary>,
    #[serde(default)]
    pub buyer: Option<MarketUserSummary>,
    #[serde(default)]
    pub timestamp: Option<u64>,
    #[serde(default)]
    pub price: Option<u64>,
    #[serde(default)]
    pub bids: Option<u64>,
    #[serde(default)]
    pub item: Option<MarketAuctionItem>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct MarketUserSummary {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct MarketAuctionItem {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub uid: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default, rename = "type")]
    pub item_type: Option<String>,
    #[serde(default)]
    pub sub_type: Option<String>,
    #[serde(default)]
    pub stats: Option<MarketItemStats>,
    #[serde(default)]
    pub bonuses: Vec<MarketItemBonus>,
    #[serde(default)]
    pub rarity: Option<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

pub type MarketMetadata = PaginatedMetadata;
pub type MarketPaginationLinks = super::common::PaginatedLinks;
