//! Torn resource wrapper APIs.

use crate::client::{DataRequestOptions, TornClient};
use crate::models::generated::RawSelectionBundle;
use crate::models::manual::torn::{
    TornCalendarBundle, TornHonorsBundle, TornItemsBundle, TornMedalsBundle,
};
use crate::transport::HttpTransport;
use crate::wrapper::error::SdkError;
use crate::wrapper::internal::{
    execute_raw, execute_typed, validate_range, validate_required_path_arg, validate_selection,
    validate_selections,
};
use crate::wrapper::options::BaseOptions;

macro_rules! raw_selection_methods {
    ($options:ty; $($method:ident => $selection:literal),* $(,)?) => {
        $(
            #[doc = concat!("Convenience helper for raw `torn.", $selection, "`.")]
            pub async fn $method(&self, options: $options) -> Result<RawSelectionBundle, SdkError> {
                self.raw_selection($selection, options).await
            }
        )*
    };
}

#[derive(Debug, Clone, Default)]
/// Options for `torn` wrapper requests.
pub struct TornOptions {
    /// Shared base options.
    pub base: BaseOptions,
    /// Optional `crimeId`.
    pub crime_id: Option<String>,
    /// Optional ids list for specific typed selections.
    pub ids: Option<String>,
    /// Optional `logCategoryId`.
    pub log_category_id: Option<String>,
    /// Optional `stockId`.
    pub stock_id: Option<String>,
}

impl TornOptions {
    /// Replaces the shared base options.
    pub fn with_base(mut self, base: BaseOptions) -> Self {
        self.base = base;
        self
    }

    /// Sets the generic/direct `id` value.
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.base = self.base.with_id(id);
        self
    }

    /// Sets `crimeId`.
    pub fn with_crime_id(mut self, value: impl Into<String>) -> Self {
        self.crime_id = Some(value.into());
        self
    }

    /// Sets the `ids` path argument used by some typed selections.
    pub fn with_ids(mut self, value: impl Into<String>) -> Self {
        self.ids = Some(value.into());
        self
    }

    /// Sets `logCategoryId`.
    pub fn with_log_category_id(mut self, value: impl Into<String>) -> Self {
        self.log_category_id = Some(value.into());
        self
    }

    /// Sets `stockId`.
    pub fn with_stock_id(mut self, value: impl Into<String>) -> Self {
        self.stock_id = Some(value.into());
        self
    }

    pub(crate) fn into_data_request_options(self) -> DataRequestOptions {
        let mut options = self.base.into_data_request_options();
        if let Some(value) = self.crime_id {
            if options.id.is_none() {
                options = options.with_id(value.clone());
            }
            options = options.with_path_arg("crimeId", value);
        }
        if let Some(value) = self.ids {
            if options.id.is_none() {
                options = options.with_id(value.clone());
            }
            options = options.with_path_arg("ids", value);
        }
        if let Some(value) = self.log_category_id {
            if options.id.is_none() {
                options = options.with_id(value.clone());
            }
            options = options.with_path_arg("logCategoryId", value);
        }
        if let Some(value) = self.stock_id {
            if options.id.is_none() {
                options = options.with_id(value.clone());
            }
            options = options.with_path_arg("stockId", value);
        }
        options
    }
}

#[derive(Debug, Clone, Copy)]
/// Torn resource API entrypoint.
pub struct TornApi<'a, T: HttpTransport> {
    pub(crate) client: &'a TornClient<T>,
}

impl<T: HttpTransport> TornApi<'_, T> {
    /// Supported `torn` selections validated by wrapper helpers.
    pub const SUPPORTED_SELECTIONS: &'static [&'static str] = &[
        "merits",
        "medals",
        "organisedcrimes",
        "pawnshop",
        "organizedcrimes",
        "itemstats",
        "items",
        "logcategories",
        "lookup",
        "logtypes",
        "stocks",
        "stats",
        "subcrimes",
        "timestamp",
        "territory",
        "properties",
        "pokertables",
        "rockpaperscissors",
        "shoplifting",
        "searchforcash",
        "companies",
        "cityshops",
        "competition",
        "education",
        "crimes",
        "bank",
        "attacklog",
        "bounties",
        "cards",
        "calendar",
        "honors",
        "hof",
        "itemammo",
        "itemmods",
        "itemdetails",
        "eliminationteam",
        "elimination",
        "factionhof",
        "gyms",
        "factiontree",
    ];

    /// Returns the underlying low-level client.
    pub fn raw_client(&self) -> &TornClient<T> {
        self.client
    }

    /// Typed helper for `torn.calendar`.
    pub async fn calendar(&self, options: TornOptions) -> Result<TornCalendarBundle, SdkError> {
        validate_options_for_selection("calendar", &options)?;
        execute_typed(
            self.client,
            "torn",
            vec!["calendar".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `torn.items`.
    pub async fn items(&self, options: TornOptions) -> Result<TornItemsBundle, SdkError> {
        validate_options_for_selection("items", &options)?;
        execute_typed(
            self.client,
            "torn",
            vec!["items".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `torn.honors`.
    pub async fn honors(&self, options: TornOptions) -> Result<TornHonorsBundle, SdkError> {
        validate_options_for_selection("honors", &options)?;
        execute_typed(
            self.client,
            "torn",
            vec!["honors".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `torn.medals`.
    pub async fn medals(&self, options: TornOptions) -> Result<TornMedalsBundle, SdkError> {
        validate_options_for_selection("medals", &options)?;
        execute_typed(
            self.client,
            "torn",
            vec!["medals".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Executes a single validated selection and returns a raw JSON bundle.
    pub async fn raw_selection(
        &self,
        selection: &str,
        options: TornOptions,
    ) -> Result<RawSelectionBundle, SdkError> {
        validate_selection("torn", selection, Self::SUPPORTED_SELECTIONS)?;
        validate_options_for_selection(selection, &options)?;
        execute_raw(
            self.client,
            "torn",
            vec![selection.to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Executes multiple validated selections and returns a raw JSON bundle.
    pub async fn raw_selections<S, I>(
        &self,
        selections: I,
        options: TornOptions,
    ) -> Result<RawSelectionBundle, SdkError>
    where
        S: Into<String>,
        I: IntoIterator<Item = S>,
    {
        let selection_values = selections.into_iter().map(Into::into).collect::<Vec<_>>();
        validate_selections("torn", &selection_values, Self::SUPPORTED_SELECTIONS)?;
        for selection in &selection_values {
            validate_options_for_selection(selection, &options)?;
        }
        execute_raw(
            self.client,
            "torn",
            selection_values,
            options.into_data_request_options(),
        )
        .await
    }

    /// Executes one validated selection and deserializes into a caller type.
    pub async fn typed_selection<R>(
        &self,
        selection: &str,
        options: TornOptions,
    ) -> Result<R, SdkError>
    where
        R: serde::de::DeserializeOwned,
    {
        validate_selection("torn", selection, Self::SUPPORTED_SELECTIONS)?;
        validate_options_for_selection(selection, &options)?;
        execute_typed(
            self.client,
            "torn",
            vec![selection.to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    raw_selection_methods!(TornOptions;
        merits_raw => "merits",
        medals_raw => "medals",
        organisedcrimes_raw => "organisedcrimes",
        pawnshop_raw => "pawnshop",
        organizedcrimes_raw => "organizedcrimes",
        itemstats_raw => "itemstats",
        items_raw => "items",
        logcategories_raw => "logcategories",
        lookup_raw => "lookup",
        logtypes_raw => "logtypes",
        stocks_raw => "stocks",
        stats_raw => "stats",
        subcrimes_raw => "subcrimes",
        timestamp_raw => "timestamp",
        territory_raw => "territory",
        properties_raw => "properties",
        pokertables_raw => "pokertables",
        rockpaperscissors_raw => "rockpaperscissors",
        shoplifting_raw => "shoplifting",
        searchforcash_raw => "searchforcash",
        companies_raw => "companies",
        cityshops_raw => "cityshops",
        competition_raw => "competition",
        education_raw => "education",
        crimes_raw => "crimes",
        bank_raw => "bank",
        attacklog_raw => "attacklog",
        bounties_raw => "bounties",
        cards_raw => "cards",
        calendar_raw => "calendar",
        honors_raw => "honors",
        hof_raw => "hof",
        itemammo_raw => "itemammo",
        itemmods_raw => "itemmods",
        itemdetails_raw => "itemdetails",
        eliminationteam_raw => "eliminationteam",
        elimination_raw => "elimination",
        factionhof_raw => "factionhof",
        gyms_raw => "gyms",
        factiontree_raw => "factiontree",
    );
}

fn validate_options_for_selection(selection: &str, options: &TornOptions) -> Result<(), SdkError> {
    validate_range("torn", options.base.from, options.base.to)?;

    match selection {
        "honors" | "items" | "medals" => {
            validate_required_path_arg(
                "torn",
                selection,
                "ids",
                options.ids.is_some() || options.base.id.is_some(),
            )?;
        }
        "logtypes" => {
            validate_required_path_arg(
                "torn",
                selection,
                "logCategoryId",
                options.log_category_id.is_some() || options.base.id.is_some(),
            )?;
        }
        "stocks" => {
            validate_required_path_arg(
                "torn",
                selection,
                "stockId",
                options.stock_id.is_some() || options.base.id.is_some(),
            )?;
        }
        "subcrimes" => {
            validate_required_path_arg(
                "torn",
                selection,
                "crimeId",
                options.crime_id.is_some() || options.base.id.is_some(),
            )?;
        }
        "itemdetails" | "eliminationteam" => {
            validate_required_path_arg("torn", selection, "id", options.base.id.is_some())?;
        }
        _ => {}
    }

    Ok(())
}
