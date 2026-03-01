//! Faction resource wrapper APIs.

use crate::client::{DataRequestOptions, TornClient};
use crate::models::generated::RawSelectionBundle;
use crate::models::manual::faction::{
    FactionBasicBundle, FactionMembersBundle, FactionRankedWarsBundle, FactionWarsBundle,
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
            #[doc = concat!("Convenience helper for raw `faction.", $selection, "`.")]
            pub async fn $method(&self, options: $options) -> Result<RawSelectionBundle, SdkError> {
                self.raw_selection($selection, options).await
            }
        )*
    };
}

#[derive(Debug, Clone, Default)]
/// Options for `faction` wrapper requests.
pub struct FactionOptions {
    /// Shared base options.
    pub base: BaseOptions,
    /// Optional `chainId`.
    pub chain_id: Option<String>,
    /// Optional `crimeId`.
    pub crime_id: Option<String>,
    /// Optional `raidWarId`.
    pub raid_war_id: Option<String>,
    /// Optional `rankedWarId`.
    pub ranked_war_id: Option<String>,
    /// Optional `territoryWarId`.
    pub territory_war_id: Option<String>,
}

impl FactionOptions {
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

    /// Sets `chainId`.
    pub fn with_chain_id(mut self, value: impl Into<String>) -> Self {
        self.chain_id = Some(value.into());
        self
    }

    /// Sets `crimeId`.
    pub fn with_crime_id(mut self, value: impl Into<String>) -> Self {
        self.crime_id = Some(value.into());
        self
    }

    /// Sets `raidWarId`.
    pub fn with_raid_war_id(mut self, value: impl Into<String>) -> Self {
        self.raid_war_id = Some(value.into());
        self
    }

    /// Sets `rankedWarId`.
    pub fn with_ranked_war_id(mut self, value: impl Into<String>) -> Self {
        self.ranked_war_id = Some(value.into());
        self
    }

    /// Sets `territoryWarId`.
    pub fn with_territory_war_id(mut self, value: impl Into<String>) -> Self {
        self.territory_war_id = Some(value.into());
        self
    }

    pub(crate) fn into_data_request_options(self) -> DataRequestOptions {
        let mut options = self.base.into_data_request_options();
        if let Some(value) = self.chain_id {
            if options.id.is_none() {
                options = options.with_id(value.clone());
            }
            options = options.with_path_arg("chainId", value);
        }
        if let Some(value) = self.crime_id {
            if options.id.is_none() {
                options = options.with_id(value.clone());
            }
            options = options.with_path_arg("crimeId", value);
        }
        if let Some(value) = self.raid_war_id {
            if options.id.is_none() {
                options = options.with_id(value.clone());
            }
            options = options.with_path_arg("raidWarId", value);
        }
        if let Some(value) = self.ranked_war_id {
            if options.id.is_none() {
                options = options.with_id(value.clone());
            }
            options = options.with_path_arg("rankedWarId", value);
        }
        if let Some(value) = self.territory_war_id {
            if options.id.is_none() {
                options = options.with_id(value.clone());
            }
            options = options.with_path_arg("territoryWarId", value);
        }
        options
    }
}

#[derive(Debug, Clone, Copy)]
/// Faction resource API entrypoint.
pub struct FactionApi<'a, T: HttpTransport> {
    pub(crate) client: &'a TornClient<T>,
}

impl<T: HttpTransport> FactionApi<'_, T> {
    /// Supported `faction` selections validated by wrapper helpers.
    pub const SUPPORTED_SELECTIONS: &'static [&'static str] = &[
        "revives",
        "reports",
        "revivesfull",
        "stats",
        "search",
        "rankedwars",
        "rackets",
        "positions",
        "raidreport",
        "rankedwarreport",
        "raids",
        "utilities",
        "upgrades",
        "warfare",
        "weapons",
        "wars",
        "timestamp",
        "territory",
        "temporary",
        "territoryownership",
        "territorywars",
        "territorywarreport",
        "caches",
        "boosters",
        "cesium",
        "chainreport",
        "chain",
        "basic",
        "armor",
        "applications",
        "attacks",
        "balance",
        "attacksfull",
        "lookup",
        "hof",
        "medical",
        "news",
        "members",
        "drugs",
        "contributors",
        "chains",
        "crime",
        "crimes",
        "crimeexp",
    ];

    /// Returns the underlying low-level client.
    pub fn raw_client(&self) -> &TornClient<T> {
        self.client
    }

    /// Typed helper for `faction.basic`.
    pub async fn basic(&self, options: FactionOptions) -> Result<FactionBasicBundle, SdkError> {
        validate_options_for_selection("basic", &options)?;
        execute_typed(
            self.client,
            "faction",
            vec!["basic".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `faction.members`.
    pub async fn members(&self, options: FactionOptions) -> Result<FactionMembersBundle, SdkError> {
        validate_options_for_selection("members", &options)?;
        execute_typed(
            self.client,
            "faction",
            vec!["members".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `faction.wars`.
    pub async fn wars(&self, options: FactionOptions) -> Result<FactionWarsBundle, SdkError> {
        validate_options_for_selection("wars", &options)?;
        execute_typed(
            self.client,
            "faction",
            vec!["wars".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `faction.rankedwars`.
    pub async fn rankedwars(
        &self,
        options: FactionOptions,
    ) -> Result<FactionRankedWarsBundle, SdkError> {
        validate_options_for_selection("rankedwars", &options)?;
        execute_typed(
            self.client,
            "faction",
            vec!["rankedwars".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Executes a single validated selection and returns a raw JSON bundle.
    pub async fn raw_selection(
        &self,
        selection: &str,
        options: FactionOptions,
    ) -> Result<RawSelectionBundle, SdkError> {
        validate_selection("faction", selection, Self::SUPPORTED_SELECTIONS)?;
        validate_options_for_selection(selection, &options)?;
        execute_raw(
            self.client,
            "faction",
            vec![selection.to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Executes multiple validated selections and returns a raw JSON bundle.
    pub async fn raw_selections<S, I>(
        &self,
        selections: I,
        options: FactionOptions,
    ) -> Result<RawSelectionBundle, SdkError>
    where
        S: Into<String>,
        I: IntoIterator<Item = S>,
    {
        let selection_values = selections.into_iter().map(Into::into).collect::<Vec<_>>();
        validate_selections("faction", &selection_values, Self::SUPPORTED_SELECTIONS)?;
        for selection in &selection_values {
            validate_options_for_selection(selection, &options)?;
        }
        execute_raw(
            self.client,
            "faction",
            selection_values,
            options.into_data_request_options(),
        )
        .await
    }

    /// Executes one validated selection and deserializes into a caller type.
    pub async fn typed_selection<R>(
        &self,
        selection: &str,
        options: FactionOptions,
    ) -> Result<R, SdkError>
    where
        R: serde::de::DeserializeOwned,
    {
        validate_selection("faction", selection, Self::SUPPORTED_SELECTIONS)?;
        validate_options_for_selection(selection, &options)?;
        execute_typed(
            self.client,
            "faction",
            vec![selection.to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    raw_selection_methods!(FactionOptions;
        revives_raw => "revives",
        reports_raw => "reports",
        revivesfull_raw => "revivesfull",
        stats_raw => "stats",
        search_raw => "search",
        rankedwars_raw => "rankedwars",
        rackets_raw => "rackets",
        positions_raw => "positions",
        raidreport_raw => "raidreport",
        rankedwarreport_raw => "rankedwarreport",
        raids_raw => "raids",
        utilities_raw => "utilities",
        upgrades_raw => "upgrades",
        warfare_raw => "warfare",
        weapons_raw => "weapons",
        wars_raw => "wars",
        timestamp_raw => "timestamp",
        territory_raw => "territory",
        temporary_raw => "temporary",
        territoryownership_raw => "territoryownership",
        territorywars_raw => "territorywars",
        territorywarreport_raw => "territorywarreport",
        caches_raw => "caches",
        boosters_raw => "boosters",
        cesium_raw => "cesium",
        chainreport_raw => "chainreport",
        chain_raw => "chain",
        basic_raw => "basic",
        armor_raw => "armor",
        applications_raw => "applications",
        attacks_raw => "attacks",
        balance_raw => "balance",
        attacksfull_raw => "attacksfull",
        lookup_raw => "lookup",
        hof_raw => "hof",
        medical_raw => "medical",
        news_raw => "news",
        members_raw => "members",
        drugs_raw => "drugs",
        contributors_raw => "contributors",
        chains_raw => "chains",
        crime_raw => "crime",
        crimes_raw => "crimes",
        crimeexp_raw => "crimeexp",
    );
}

fn validate_options_for_selection(
    selection: &str,
    options: &FactionOptions,
) -> Result<(), SdkError> {
    validate_range("faction", options.base.from, options.base.to)?;

    if requires_faction_id(selection) {
        validate_required_path_arg("faction", selection, "id", options.base.id.is_some())?;
    }

    match selection {
        "chainreport" => {
            validate_required_path_arg(
                "faction",
                selection,
                "chainId",
                options.chain_id.is_some() || options.base.id.is_some(),
            )?;
        }
        "crime" => {
            validate_required_path_arg(
                "faction",
                selection,
                "crimeId",
                options.crime_id.is_some() || options.base.id.is_some(),
            )?;
        }
        "raidreport" => {
            validate_required_path_arg(
                "faction",
                selection,
                "raidWarId",
                options.raid_war_id.is_some() || options.base.id.is_some(),
            )?;
        }
        "rankedwarreport" => {
            validate_required_path_arg(
                "faction",
                selection,
                "rankedWarId",
                options.ranked_war_id.is_some() || options.base.id.is_some(),
            )?;
        }
        "territorywarreport" => {
            validate_required_path_arg(
                "faction",
                selection,
                "territoryWarId",
                options.territory_war_id.is_some() || options.base.id.is_some(),
            )?;
        }
        "search" if options.base.name.is_none() => {
            return Err(SdkError::Validation(
                "resource 'faction' selection 'search' requires 'name' filter to be provided"
                    .to_string(),
            ));
        }
        _ => {}
    }

    Ok(())
}

fn requires_faction_id(selection: &str) -> bool {
    matches!(
        selection,
        "basic"
            | "chain"
            | "chains"
            | "hof"
            | "members"
            | "raids"
            | "rankedwars"
            | "territory"
            | "territorywars"
            | "wars"
    )
}
