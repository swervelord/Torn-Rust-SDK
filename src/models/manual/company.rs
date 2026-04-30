use std::collections::BTreeMap;

use serde::Deserialize;

use super::user::{UserLastAction, UserStatus};

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

pub type CompanyDetailedBundle = CompanyProfileBundle;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct CompanySearchBundle {
    #[serde(default)]
    pub search: serde_json::Value,
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

#[derive(Debug, Clone, Deserialize, Default)]
pub struct CompanyProfileBundle {
    #[serde(default, alias = "profile")]
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
    #[serde(default, rename = "type")]
    pub company_kind: Option<CompanyType>,
    #[serde(default)]
    pub rating: Option<i64>,
    #[serde(default)]
    pub age: Option<u64>,
    #[serde(default)]
    pub days_old: Option<u64>,
    #[serde(default)]
    pub created_at: Option<u64>,
    #[serde(default)]
    pub image: Option<String>,
    #[serde(default)]
    pub daily_income: Option<i64>,
    #[serde(default)]
    pub weekly_income: Option<i64>,
    #[serde(default)]
    pub income: Option<CompanyIncome>,
    #[serde(default)]
    pub customers: Option<CompanyCustomers>,
    #[serde(default)]
    pub employees: Option<serde_json::Value>,
    #[serde(default)]
    pub director: Option<CompanyDirector>,
    #[serde(default)]
    pub status: Option<UserStatus>,
    #[serde(default)]
    pub funds: Option<i64>,
    #[serde(default)]
    pub popularity: Option<i64>,
    #[serde(default)]
    pub efficiency: Option<i64>,
    #[serde(default)]
    pub environment: Option<i64>,
    #[serde(default)]
    pub trains: Option<u32>,
    #[serde(default)]
    pub applications_allowed: Option<bool>,
    #[serde(default)]
    pub advertisement_budget: Option<i64>,
    #[serde(default)]
    pub upgrades: Option<CompanyUpgrades>,
    #[serde(default)]
    pub value: Option<f64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

impl CompanyProfileBundle {
    /// Accessor matching the API v2 `profile` envelope.
    pub fn profile(&self) -> &CompanyProfile {
        &self.company
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct CompanyType {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct CompanyIncome {
    #[serde(default)]
    pub daily: Option<i64>,
    #[serde(default)]
    pub weekly: Option<i64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct CompanyCustomers {
    #[serde(default)]
    pub daily: Option<i64>,
    #[serde(default)]
    pub weekly: Option<i64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct CompanyDirector {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub status: Option<UserStatus>,
    #[serde(default)]
    pub last_action: Option<UserLastAction>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct CompanyUpgrades {
    #[serde(default)]
    pub staff_room: Option<String>,
    #[serde(default)]
    pub storage: Option<String>,
    #[serde(default)]
    pub storage_capacity: Option<serde_json::Value>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct CompanyApplicationSummary {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default, alias = "player")]
    pub user: Option<CompanyApplicationUser>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default, alias = "expires_at")]
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
    #[serde(default)]
    pub stats: Option<CompanyApplicationStats>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct CompanyApplicationStats {
    #[serde(default)]
    pub manual_labor: Option<i64>,
    #[serde(default)]
    pub intelligence: Option<i64>,
    #[serde(default)]
    pub endurance: Option<i64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct CompanyEmployeesBundle {
    #[serde(default)]
    pub company_employees: Option<BTreeMap<String, CompanyEmployee>>,
    #[serde(default)]
    pub employees: Vec<CompanyEmployee>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct CompanyEmployee {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_position_name")]
    pub position: Option<String>,
    #[serde(default)]
    pub days_in_company: Option<u64>,
    #[serde(default)]
    pub joined_at: Option<u64>,
    #[serde(default)]
    pub wage: Option<i64>,
    #[serde(default)]
    pub manual_labor: Option<i64>,
    #[serde(default)]
    pub intelligence: Option<i64>,
    #[serde(default)]
    pub endurance: Option<i64>,
    #[serde(default)]
    pub stats: Option<CompanyEmployeeStats>,
    #[serde(default)]
    pub effectiveness: Option<CompanyEmployeeEffectiveness>,
    #[serde(default)]
    pub value: Option<f64>,
    #[serde(default)]
    pub status: Option<UserStatus>,
    #[serde(default)]
    pub last_action: Option<UserLastAction>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct CompanyEmployeeStats {
    #[serde(default)]
    pub manual_labor: Option<i64>,
    #[serde(default)]
    pub intelligence: Option<i64>,
    #[serde(default)]
    pub endurance: Option<i64>,
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
    pub book: Option<i64>,
    #[serde(default)]
    pub merits: Option<i64>,
    #[serde(default)]
    pub director_education: Option<i64>,
    #[serde(default)]
    pub management: Option<i64>,
    #[serde(default)]
    pub wrong_gender: Option<i64>,
    #[serde(default)]
    pub addiction: Option<i64>,
    #[serde(default)]
    pub inactivity: Option<i64>,
    #[serde(default)]
    pub total: Option<i64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct CompanyStockBundle {
    #[serde(default)]
    pub company_stock: BTreeMap<String, CompanyStockEntry>,
    #[serde(default)]
    pub stock: Vec<CompanyStockEntry>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct CompanyStockEntry {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub cost: Option<i64>,
    #[serde(default)]
    pub rrp: Option<i64>,
    #[serde(default)]
    pub price: Option<i64>,
    #[serde(default)]
    pub in_stock: Option<u64>,
    #[serde(default)]
    pub on_order: Option<u64>,
    #[serde(default)]
    pub sold_amount: Option<u64>,
    #[serde(default)]
    pub sold_worth: Option<i64>,
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

fn deserialize_optional_position_name<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = Option::<serde_json::Value>::deserialize(deserializer)?;
    Ok(match value {
        Some(serde_json::Value::String(position)) => Some(position),
        Some(serde_json::Value::Object(mut object)) => object
            .remove("name")
            .and_then(|name| name.as_str().map(ToOwned::to_owned)),
        _ => None,
    })
}
