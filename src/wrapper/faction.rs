//! Faction resource wrapper APIs.

use crate::client::{DataRequestOptions, TornClient};
use crate::models::generated::RawSelectionBundle;
use crate::models::manual::faction::{
    FactionApplicationsBundle, FactionArmorBundle, FactionAttacksBundle, FactionAttacksFullBundle,
    FactionBalanceBundle, FactionBasicBundle, FactionBoostersBundle, FactionCachesBundle,
    FactionCesiumBundle, FactionChainBundle, FactionChainReportBundle, FactionChainsBundle,
    FactionContributorsBundle, FactionCrimeBundle, FactionCrimeExpBundle, FactionCrimesBundle,
    FactionDrugsBundle, FactionHofBundle, FactionLookupBundle, FactionMedicalBundle,
    FactionMembersBundle, FactionNewsBundle, FactionPositionsBundle, FactionRacketsBundle,
    FactionRaidReportBundle, FactionRaidsBundle, FactionRankedWarReportBundle,
    FactionRankedWarsBundle, FactionReportsBundle, FactionRevivesBundle, FactionRevivesFullBundle,
    FactionSearchBundle, FactionStatsBundle, FactionTemporaryBundle, FactionTerritoryBundle,
    FactionTerritoryOwnershipBundle, FactionTerritoryWarReportBundle, FactionTerritoryWarsBundle,
    FactionTimestampBundle, FactionUpgradesBundle, FactionUtilitiesBundle, FactionWarfareBundle,
    FactionWarsBundle, FactionWeaponsBundle,
};
use crate::transport::HttpTransport;
use crate::wrapper::error::SdkError;
use crate::wrapper::internal::{
    execute_raw, execute_typed, validate_range, validate_required_path_arg,
    validate_required_query_arg, validate_selection, validate_selections,
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

    /// Typed helper for `faction.attacks`.
    pub async fn attacks(&self, options: FactionOptions) -> Result<FactionAttacksBundle, SdkError> {
        validate_options_for_selection("attacks", &options)?;
        execute_typed(
            self.client,
            "faction",
            vec!["attacks".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `faction.attacksfull`.
    pub async fn attacksfull(
        &self,
        options: FactionOptions,
    ) -> Result<FactionAttacksFullBundle, SdkError> {
        validate_options_for_selection("attacksfull", &options)?;
        execute_typed(
            self.client,
            "faction",
            vec!["attacksfull".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `faction.applications`.
    pub async fn applications(
        &self,
        options: FactionOptions,
    ) -> Result<FactionApplicationsBundle, SdkError> {
        validate_options_for_selection("applications", &options)?;
        execute_typed(
            self.client,
            "faction",
            vec!["applications".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `faction.armor`.
    pub async fn armor(&self, options: FactionOptions) -> Result<FactionArmorBundle, SdkError> {
        validate_options_for_selection("armor", &options)?;
        execute_typed(
            self.client,
            "faction",
            vec!["armor".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `faction.boosters`.
    pub async fn boosters(
        &self,
        options: FactionOptions,
    ) -> Result<FactionBoostersBundle, SdkError> {
        validate_options_for_selection("boosters", &options)?;
        execute_typed(
            self.client,
            "faction",
            vec!["boosters".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `faction.caches`.
    pub async fn caches(&self, options: FactionOptions) -> Result<FactionCachesBundle, SdkError> {
        validate_options_for_selection("caches", &options)?;
        execute_typed(
            self.client,
            "faction",
            vec!["caches".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `faction.cesium`.
    pub async fn cesium(&self, options: FactionOptions) -> Result<FactionCesiumBundle, SdkError> {
        validate_options_for_selection("cesium", &options)?;
        execute_typed(
            self.client,
            "faction",
            vec!["cesium".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `faction.lookup`.
    pub async fn lookup(&self, options: FactionOptions) -> Result<FactionLookupBundle, SdkError> {
        validate_options_for_selection("lookup", &options)?;
        execute_typed(
            self.client,
            "faction",
            vec!["lookup".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `faction.chain`.
    pub async fn chain(&self, options: FactionOptions) -> Result<FactionChainBundle, SdkError> {
        validate_options_for_selection("chain", &options)?;
        execute_typed(
            self.client,
            "faction",
            vec!["chain".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `faction.chains`.
    pub async fn chains(&self, options: FactionOptions) -> Result<FactionChainsBundle, SdkError> {
        validate_options_for_selection("chains", &options)?;
        execute_typed(
            self.client,
            "faction",
            vec!["chains".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `faction.chainreport`.
    pub async fn chainreport(
        &self,
        options: FactionOptions,
    ) -> Result<FactionChainReportBundle, SdkError> {
        validate_options_for_selection("chainreport", &options)?;
        execute_typed(
            self.client,
            "faction",
            vec!["chainreport".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `faction.balance`.
    pub async fn balance(&self, options: FactionOptions) -> Result<FactionBalanceBundle, SdkError> {
        validate_options_for_selection("balance", &options)?;
        execute_typed(
            self.client,
            "faction",
            vec!["balance".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `faction.contributors`.
    pub async fn contributors(
        &self,
        options: FactionOptions,
    ) -> Result<FactionContributorsBundle, SdkError> {
        validate_options_for_selection("contributors", &options)?;
        execute_typed(
            self.client,
            "faction",
            vec!["contributors".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `faction.crime`.
    pub async fn crime(&self, options: FactionOptions) -> Result<FactionCrimeBundle, SdkError> {
        validate_options_for_selection("crime", &options)?;
        execute_typed(
            self.client,
            "faction",
            vec!["crime".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `faction.crimeexp`.
    pub async fn crimeexp(
        &self,
        options: FactionOptions,
    ) -> Result<FactionCrimeExpBundle, SdkError> {
        validate_options_for_selection("crimeexp", &options)?;
        execute_typed(
            self.client,
            "faction",
            vec!["crimeexp".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `faction.crimes`.
    pub async fn crimes(&self, options: FactionOptions) -> Result<FactionCrimesBundle, SdkError> {
        validate_options_for_selection("crimes", &options)?;
        execute_typed(
            self.client,
            "faction",
            vec!["crimes".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `faction.drugs`.
    pub async fn drugs(&self, options: FactionOptions) -> Result<FactionDrugsBundle, SdkError> {
        validate_options_for_selection("drugs", &options)?;
        execute_typed(
            self.client,
            "faction",
            vec!["drugs".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `faction.rackets`.
    pub async fn rackets(&self, options: FactionOptions) -> Result<FactionRacketsBundle, SdkError> {
        validate_options_for_selection("rackets", &options)?;
        execute_typed(
            self.client,
            "faction",
            vec!["rackets".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `faction.medical`.
    pub async fn medical(&self, options: FactionOptions) -> Result<FactionMedicalBundle, SdkError> {
        validate_options_for_selection("medical", &options)?;
        execute_typed(
            self.client,
            "faction",
            vec!["medical".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `faction.hof`.
    pub async fn hof(&self, options: FactionOptions) -> Result<FactionHofBundle, SdkError> {
        validate_options_for_selection("hof", &options)?;
        execute_typed(
            self.client,
            "faction",
            vec!["hof".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `faction.positions`.
    pub async fn positions(
        &self,
        options: FactionOptions,
    ) -> Result<FactionPositionsBundle, SdkError> {
        validate_options_for_selection("positions", &options)?;
        execute_typed(
            self.client,
            "faction",
            vec!["positions".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `faction.search`.
    pub async fn search(&self, options: FactionOptions) -> Result<FactionSearchBundle, SdkError> {
        validate_options_for_selection("search", &options)?;
        execute_typed(
            self.client,
            "faction",
            vec!["search".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `faction.stats`.
    pub async fn stats(&self, options: FactionOptions) -> Result<FactionStatsBundle, SdkError> {
        validate_options_for_selection("stats", &options)?;
        execute_typed(
            self.client,
            "faction",
            vec!["stats".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `faction.raids`.
    pub async fn raids(&self, options: FactionOptions) -> Result<FactionRaidsBundle, SdkError> {
        validate_options_for_selection("raids", &options)?;
        execute_typed(
            self.client,
            "faction",
            vec!["raids".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `faction.raidreport`.
    pub async fn raidreport(
        &self,
        options: FactionOptions,
    ) -> Result<FactionRaidReportBundle, SdkError> {
        validate_options_for_selection("raidreport", &options)?;
        execute_typed(
            self.client,
            "faction",
            vec!["raidreport".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `faction.rankedwarreport`.
    pub async fn rankedwarreport(
        &self,
        options: FactionOptions,
    ) -> Result<FactionRankedWarReportBundle, SdkError> {
        validate_options_for_selection("rankedwarreport", &options)?;
        execute_typed(
            self.client,
            "faction",
            vec!["rankedwarreport".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `faction.reports`.
    pub async fn reports(&self, options: FactionOptions) -> Result<FactionReportsBundle, SdkError> {
        validate_options_for_selection("reports", &options)?;
        execute_typed(
            self.client,
            "faction",
            vec!["reports".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `faction.revives`.
    pub async fn revives(&self, options: FactionOptions) -> Result<FactionRevivesBundle, SdkError> {
        validate_options_for_selection("revives", &options)?;
        execute_typed(
            self.client,
            "faction",
            vec!["revives".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `faction.revivesfull`.
    pub async fn revivesfull(
        &self,
        options: FactionOptions,
    ) -> Result<FactionRevivesFullBundle, SdkError> {
        validate_options_for_selection("revivesfull", &options)?;
        execute_typed(
            self.client,
            "faction",
            vec!["revivesfull".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `faction.timestamp`.
    pub async fn timestamp(
        &self,
        options: FactionOptions,
    ) -> Result<FactionTimestampBundle, SdkError> {
        validate_options_for_selection("timestamp", &options)?;
        execute_typed(
            self.client,
            "faction",
            vec!["timestamp".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `faction.temporary`.
    pub async fn temporary(
        &self,
        options: FactionOptions,
    ) -> Result<FactionTemporaryBundle, SdkError> {
        validate_options_for_selection("temporary", &options)?;
        execute_typed(
            self.client,
            "faction",
            vec!["temporary".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `faction.territory`.
    pub async fn territory(
        &self,
        options: FactionOptions,
    ) -> Result<FactionTerritoryBundle, SdkError> {
        validate_options_for_selection("territory", &options)?;
        execute_typed(
            self.client,
            "faction",
            vec!["territory".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `faction.territoryownership`.
    pub async fn territoryownership(
        &self,
        options: FactionOptions,
    ) -> Result<FactionTerritoryOwnershipBundle, SdkError> {
        validate_options_for_selection("territoryownership", &options)?;
        execute_typed(
            self.client,
            "faction",
            vec!["territoryownership".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `faction.territorywars`.
    pub async fn territorywars(
        &self,
        options: FactionOptions,
    ) -> Result<FactionTerritoryWarsBundle, SdkError> {
        validate_options_for_selection("territorywars", &options)?;
        execute_typed(
            self.client,
            "faction",
            vec!["territorywars".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `faction.territorywarreport`.
    pub async fn territorywarreport(
        &self,
        options: FactionOptions,
    ) -> Result<FactionTerritoryWarReportBundle, SdkError> {
        validate_options_for_selection("territorywarreport", &options)?;
        execute_typed(
            self.client,
            "faction",
            vec!["territorywarreport".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `faction.utilities`.
    pub async fn utilities(
        &self,
        options: FactionOptions,
    ) -> Result<FactionUtilitiesBundle, SdkError> {
        validate_options_for_selection("utilities", &options)?;
        execute_typed(
            self.client,
            "faction",
            vec!["utilities".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `faction.warfare`.
    pub async fn warfare(&self, options: FactionOptions) -> Result<FactionWarfareBundle, SdkError> {
        validate_options_for_selection("warfare", &options)?;
        execute_typed(
            self.client,
            "faction",
            vec!["warfare".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `faction.weapons`.
    pub async fn weapons(&self, options: FactionOptions) -> Result<FactionWeaponsBundle, SdkError> {
        validate_options_for_selection("weapons", &options)?;
        execute_typed(
            self.client,
            "faction",
            vec!["weapons".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `faction.news`.
    pub async fn news(&self, options: FactionOptions) -> Result<FactionNewsBundle, SdkError> {
        validate_options_for_selection("news", &options)?;
        execute_typed(
            self.client,
            "faction",
            vec!["news".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `faction.upgrades`.
    pub async fn upgrades(
        &self,
        options: FactionOptions,
    ) -> Result<FactionUpgradesBundle, SdkError> {
        validate_options_for_selection("upgrades", &options)?;
        execute_typed(
            self.client,
            "faction",
            vec!["upgrades".to_string()],
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
        "contributors" => {
            validate_required_query_arg("faction", selection, "stat", options.base.stat.is_some())?;
        }
        "news" | "warfare" => {
            validate_required_query_arg("faction", selection, "cat", options.base.cat.is_some())?;
        }
        _ => {}
    }

    Ok(())
}

fn requires_faction_id(selection: &str) -> bool {
    matches!(
        selection,
        "basic" | "hof" | "members" | "rankedwars" | "wars"
    )
}
