//! Torn resource wrapper APIs.

use crate::client::{DataRequestOptions, TornClient};
use crate::models::generated::RawSelectionBundle;
use crate::models::manual::torn::{
    TornAttackLogBundle, TornBankBundle, TornBountiesBundle, TornCalendarBundle, TornCardsBundle,
    TornCityShopsBundle, TornCompaniesBundle, TornCompetitionBundle, TornCrimesBundle,
    TornEducationBundle, TornEliminationBundle, TornEliminationTeamBundle, TornFactionHofBundle,
    TornFactionTreeBundle, TornGymsBundle, TornHofBundle, TornHonorsBundle, TornItemAmmoBundle,
    TornItemDetailsBundle, TornItemModsBundle, TornItemStatsBundle, TornItemsBundle,
    TornLogCategoriesBundle, TornLogTypesBundle, TornLookupBundle, TornMedalsBundle,
    TornMeritsBundle, TornOrganisedCrimesBundle, TornOrganizedCrimesBundle, TornPawnshopBundle,
    TornPokerTablesBundle, TornPropertiesBundle, TornRockPaperScissorsBundle,
    TornSearchForCashBundle, TornShopliftingBundle, TornStatsBundle, TornStockBundle,
    TornStocksBundle, TornSubcrimesBundle, TornTerritoryBundle, TornTimestampBundle,
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

    /// Typed helper for `torn.logcategories`.
    pub async fn logcategories(
        &self,
        options: TornOptions,
    ) -> Result<TornLogCategoriesBundle, SdkError> {
        validate_options_for_selection("logcategories", &options)?;
        execute_typed(
            self.client,
            "torn",
            vec!["logcategories".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `torn.logtypes`.
    pub async fn logtypes(&self, options: TornOptions) -> Result<TornLogTypesBundle, SdkError> {
        validate_options_for_selection("logtypes", &options)?;
        execute_typed(
            self.client,
            "torn",
            vec!["logtypes".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `torn.merits`.
    pub async fn merits(&self, options: TornOptions) -> Result<TornMeritsBundle, SdkError> {
        validate_options_for_selection("merits", &options)?;
        execute_typed(
            self.client,
            "torn",
            vec!["merits".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `torn.itemammo`.
    pub async fn itemammo(&self, options: TornOptions) -> Result<TornItemAmmoBundle, SdkError> {
        validate_options_for_selection("itemammo", &options)?;
        execute_typed(
            self.client,
            "torn",
            vec!["itemammo".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `torn.itemmods`.
    pub async fn itemmods(&self, options: TornOptions) -> Result<TornItemModsBundle, SdkError> {
        validate_options_for_selection("itemmods", &options)?;
        execute_typed(
            self.client,
            "torn",
            vec!["itemmods".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `torn.properties`.
    pub async fn properties(&self, options: TornOptions) -> Result<TornPropertiesBundle, SdkError> {
        validate_options_for_selection("properties", &options)?;
        execute_typed(
            self.client,
            "torn",
            vec!["properties".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for the `torn.stocks` collection endpoint.
    pub async fn stocks(&self, options: TornOptions) -> Result<TornStocksBundle, SdkError> {
        validate_options_for_selection("stocks", &options)?;
        if options.stock_id.is_some() || options.base.id.is_some() {
            return Err(SdkError::Validation(
                "torn.stocks list helper does not accept stockId; use torn.stock() instead"
                    .to_string(),
            ));
        }
        execute_typed(
            self.client,
            "torn",
            vec!["stocks".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for the single-stock `torn.stocks` detail endpoint.
    pub async fn stock(&self, options: TornOptions) -> Result<TornStockBundle, SdkError> {
        validate_options_for_selection("stocks", &options)?;
        validate_required_path_arg(
            "torn",
            "stocks",
            "stockId",
            options.stock_id.is_some() || options.base.id.is_some(),
        )?;
        execute_typed(
            self.client,
            "torn",
            vec!["stocks".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `torn.bounties`.
    pub async fn bounties(&self, options: TornOptions) -> Result<TornBountiesBundle, SdkError> {
        validate_options_for_selection("bounties", &options)?;
        execute_typed(
            self.client,
            "torn",
            vec!["bounties".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for fallback `torn.bank`.
    pub async fn bank(&self, options: TornOptions) -> Result<TornBankBundle, SdkError> {
        validate_options_for_selection("bank", &options)?;
        execute_typed(
            self.client,
            "torn",
            vec!["bank".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for stable `torn.attacklog`.
    pub async fn attacklog(&self, options: TornOptions) -> Result<TornAttackLogBundle, SdkError> {
        validate_options_for_selection("attacklog", &options)?;
        execute_typed(
            self.client,
            "torn",
            vec!["attacklog".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for fallback `torn.cards`.
    pub async fn cards(&self, options: TornOptions) -> Result<TornCardsBundle, SdkError> {
        validate_options_for_selection("cards", &options)?;
        execute_typed(
            self.client,
            "torn",
            vec!["cards".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for fallback `torn.cityshops`.
    pub async fn cityshops(&self, options: TornOptions) -> Result<TornCityShopsBundle, SdkError> {
        validate_options_for_selection("cityshops", &options)?;
        execute_typed(
            self.client,
            "torn",
            vec!["cityshops".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for fallback `torn.companies`.
    pub async fn companies(&self, options: TornOptions) -> Result<TornCompaniesBundle, SdkError> {
        validate_options_for_selection("companies", &options)?;
        execute_typed(
            self.client,
            "torn",
            vec!["companies".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for fallback `torn.gyms`.
    pub async fn gyms(&self, options: TornOptions) -> Result<TornGymsBundle, SdkError> {
        validate_options_for_selection("gyms", &options)?;
        execute_typed(
            self.client,
            "torn",
            vec!["gyms".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for fallback `torn.itemstats`.
    pub async fn itemstats(&self, options: TornOptions) -> Result<TornItemStatsBundle, SdkError> {
        validate_options_for_selection("itemstats", &options)?;
        execute_typed(
            self.client,
            "torn",
            vec!["itemstats".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for conservative `torn.itemdetails`.
    pub async fn itemdetails(
        &self,
        options: TornOptions,
    ) -> Result<TornItemDetailsBundle, SdkError> {
        validate_options_for_selection("itemdetails", &options)?;
        execute_typed(
            self.client,
            "torn",
            vec!["itemdetails".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for fallback `torn.pawnshop`.
    pub async fn pawnshop(&self, options: TornOptions) -> Result<TornPawnshopBundle, SdkError> {
        validate_options_for_selection("pawnshop", &options)?;
        execute_typed(
            self.client,
            "torn",
            vec!["pawnshop".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for fallback `torn.pokertables`.
    pub async fn pokertables(
        &self,
        options: TornOptions,
    ) -> Result<TornPokerTablesBundle, SdkError> {
        validate_options_for_selection("pokertables", &options)?;
        execute_typed(
            self.client,
            "torn",
            vec!["pokertables".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `torn.lookup`.
    pub async fn lookup(&self, options: TornOptions) -> Result<TornLookupBundle, SdkError> {
        validate_options_for_selection("lookup", &options)?;
        execute_typed(
            self.client,
            "torn",
            vec!["lookup".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `torn.timestamp`.
    pub async fn timestamp(&self, options: TornOptions) -> Result<TornTimestampBundle, SdkError> {
        validate_options_for_selection("timestamp", &options)?;
        execute_typed(
            self.client,
            "torn",
            vec!["timestamp".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for fallback `torn.competition`.
    pub async fn competition(
        &self,
        options: TornOptions,
    ) -> Result<TornCompetitionBundle, SdkError> {
        validate_options_for_selection("competition", &options)?;
        execute_typed(
            self.client,
            "torn",
            vec!["competition".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `torn.crimes`.
    pub async fn crimes(&self, options: TornOptions) -> Result<TornCrimesBundle, SdkError> {
        validate_options_for_selection("crimes", &options)?;
        execute_typed(
            self.client,
            "torn",
            vec!["crimes".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `torn.education`.
    pub async fn education(&self, options: TornOptions) -> Result<TornEducationBundle, SdkError> {
        validate_options_for_selection("education", &options)?;
        execute_typed(
            self.client,
            "torn",
            vec!["education".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `torn.elimination`.
    pub async fn elimination(
        &self,
        options: TornOptions,
    ) -> Result<TornEliminationBundle, SdkError> {
        validate_options_for_selection("elimination", &options)?;
        execute_typed(
            self.client,
            "torn",
            vec!["elimination".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for conservative `torn.eliminationteam`.
    pub async fn eliminationteam(
        &self,
        options: TornOptions,
    ) -> Result<TornEliminationTeamBundle, SdkError> {
        validate_options_for_selection("eliminationteam", &options)?;
        execute_typed(
            self.client,
            "torn",
            vec!["eliminationteam".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `torn.factionhof`.
    pub async fn factionhof(&self, options: TornOptions) -> Result<TornFactionHofBundle, SdkError> {
        validate_options_for_selection("factionhof", &options)?;
        execute_typed(
            self.client,
            "torn",
            vec!["factionhof".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `torn.factiontree`.
    pub async fn factiontree(
        &self,
        options: TornOptions,
    ) -> Result<TornFactionTreeBundle, SdkError> {
        validate_options_for_selection("factiontree", &options)?;
        execute_typed(
            self.client,
            "torn",
            vec!["factiontree".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `torn.hof`.
    pub async fn hof(&self, options: TornOptions) -> Result<TornHofBundle, SdkError> {
        validate_options_for_selection("hof", &options)?;
        execute_typed(
            self.client,
            "torn",
            vec!["hof".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for fallback `torn.organisedcrimes`.
    pub async fn organisedcrimes(
        &self,
        options: TornOptions,
    ) -> Result<TornOrganisedCrimesBundle, SdkError> {
        validate_options_for_selection("organisedcrimes", &options)?;
        execute_typed(
            self.client,
            "torn",
            vec!["organisedcrimes".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `torn.organizedcrimes`.
    pub async fn organizedcrimes(
        &self,
        options: TornOptions,
    ) -> Result<TornOrganizedCrimesBundle, SdkError> {
        validate_options_for_selection("organizedcrimes", &options)?;
        execute_typed(
            self.client,
            "torn",
            vec!["organizedcrimes".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for fallback `torn.rockpaperscissors`.
    pub async fn rockpaperscissors(
        &self,
        options: TornOptions,
    ) -> Result<TornRockPaperScissorsBundle, SdkError> {
        validate_options_for_selection("rockpaperscissors", &options)?;
        execute_typed(
            self.client,
            "torn",
            vec!["rockpaperscissors".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for fallback `torn.searchforcash`.
    pub async fn searchforcash(
        &self,
        options: TornOptions,
    ) -> Result<TornSearchForCashBundle, SdkError> {
        validate_options_for_selection("searchforcash", &options)?;
        execute_typed(
            self.client,
            "torn",
            vec!["searchforcash".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for fallback `torn.shoplifting`.
    pub async fn shoplifting(
        &self,
        options: TornOptions,
    ) -> Result<TornShopliftingBundle, SdkError> {
        validate_options_for_selection("shoplifting", &options)?;
        execute_typed(
            self.client,
            "torn",
            vec!["shoplifting".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for fallback `torn.stats`.
    pub async fn stats(&self, options: TornOptions) -> Result<TornStatsBundle, SdkError> {
        validate_options_for_selection("stats", &options)?;
        execute_typed(
            self.client,
            "torn",
            vec!["stats".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `torn.subcrimes`.
    pub async fn subcrimes(&self, options: TornOptions) -> Result<TornSubcrimesBundle, SdkError> {
        validate_options_for_selection("subcrimes", &options)?;
        execute_typed(
            self.client,
            "torn",
            vec!["subcrimes".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `torn.territory`.
    pub async fn territory(&self, options: TornOptions) -> Result<TornTerritoryBundle, SdkError> {
        validate_options_for_selection("territory", &options)?;
        execute_typed(
            self.client,
            "torn",
            vec!["territory".to_string()],
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
        "attacklog" => {
            validate_required_query_arg("torn", selection, "log", options.base.log.is_some())?;
        }
        "honors" | "items" | "medals" => {
            validate_required_path_arg(
                "torn",
                selection,
                "ids",
                options.ids.is_some() || options.base.id.is_some(),
            )?;
        }
        "itemstats" => {
            validate_required_path_arg("torn", selection, "id", options.base.id.is_some())?;
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
