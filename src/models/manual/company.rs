use std::collections::BTreeMap;

use serde::Deserialize;

use super::user::UserStatus;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct CompanyApplicationsBundle {
    #[serde(default)]
    pub applications: Vec<CompanyApplicationSummary>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct CompanyCompaniesBundle {
    #[serde(default)]
    pub company: Option<serde_json::Value>,
    #[serde(default)]
    pub company_timestamp: Option<u64>,
    #[serde(default)]
    pub company_delay: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct CompanyDetailedBundle {
    #[serde(default)]
    pub company_detailed: Option<CompanyDetailedProfile>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct CompanyLookupBundle {
    #[serde(default)]
    pub selections: Vec<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct CompanyNewsBundle {
    #[serde(default)]
    pub news: BTreeMap<String, CompanyNewsEntry>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct CompanyTimestampBundle {
    #[serde(default)]
    pub timestamp: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CompanyProfileBundle {
    pub company: CompanyProfile,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct CompanyProfile {
    #[serde(default, alias = "ID")]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub company_type: Option<u64>,
    #[serde(default)]
    pub rating: Option<i64>,
    #[serde(default)]
    pub age: Option<u64>,
    #[serde(default)]
    pub daily_income: Option<i64>,
    #[serde(default)]
    pub weekly_income: Option<i64>,
    /// The legacy profile payload exposes `employees`, but its nested shape is not stable enough
    /// to model more narrowly here.
    #[serde(default)]
    pub employees: Option<serde_json::Value>,
    #[serde(default)]
    pub status: Option<UserStatus>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct CompanyApplicationSummary {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub user: Option<CompanyApplicationUser>,
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
pub struct CompanyApplicationUser {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub level: Option<u32>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct CompanyDetailedProfile {
    #[serde(default, alias = "ID")]
    pub id: Option<u64>,
    #[serde(default)]
    pub company_funds: Option<i64>,
    #[serde(default)]
    pub company_bank: Option<i64>,
    #[serde(default)]
    pub popularity: Option<i64>,
    #[serde(default)]
    pub efficiency: Option<i64>,
    #[serde(default)]
    pub environment: Option<i64>,
    #[serde(default)]
    pub trains_available: Option<u32>,
    #[serde(default)]
    pub advertising_budget: Option<i64>,
    #[serde(default)]
    pub upgrades: Option<CompanyDetailedUpgrades>,
    #[serde(default)]
    pub value: Option<i64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct CompanyDetailedUpgrades {
    #[serde(default)]
    pub company_size: Option<u32>,
    #[serde(default)]
    pub staffroom_size: Option<String>,
    #[serde(default)]
    pub storage_size: Option<String>,
    #[serde(default)]
    pub storage_space: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct CompanyEmployeesBundle {
    #[serde(default)]
    pub company_employees: Option<BTreeMap<String, CompanyEmployee>>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct CompanyEmployee {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub position: Option<String>,
    #[serde(default)]
    pub days_in_company: Option<u64>,
    #[serde(default)]
    pub wage: Option<i64>,
    #[serde(default)]
    pub manual_labor: Option<i64>,
    #[serde(default)]
    pub intelligence: Option<i64>,
    #[serde(default)]
    pub endurance: Option<i64>,
    #[serde(default)]
    pub effectiveness: Option<CompanyEmployeeEffectiveness>,
    #[serde(default)]
    pub status: Option<UserStatus>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct CompanyEmployeeEffectiveness {
    #[serde(default)]
    pub working_stats: Option<i64>,
    #[serde(default)]
    pub settled_in: Option<i64>,
    #[serde(default)]
    pub director_education: Option<i64>,
    #[serde(default)]
    pub total: Option<i64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct CompanyStockBundle {
    #[serde(default)]
    pub company_stock: BTreeMap<String, CompanyStockEntry>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct CompanyStockEntry {
    #[serde(default)]
    pub cost: Option<i64>,
    #[serde(default)]
    pub price: Option<i64>,
    #[serde(default)]
    pub in_stock: Option<u64>,
    #[serde(default)]
    pub on_order: Option<u64>,
    #[serde(default)]
    pub sold_amount: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct CompanyNewsEntry {
    #[serde(default)]
    pub news: Option<String>,
    #[serde(default)]
    pub timestamp: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}
