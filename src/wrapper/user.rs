//! User resource wrapper APIs.

use crate::client::{DataRequestOptions, TornClient};
use crate::models::generated::RawSelectionBundle;
use crate::models::manual::user::{
    UserBarsBundle, UserBasicBundle, UserCooldownsBundle, UserDiscordBundle, UserFactionBundle,
    UserMoneyBundle, UserProfileBundle, UserTravelBundle,
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
            #[doc = concat!("Convenience helper for raw `user.", $selection, "`.")]
            pub async fn $method(&self, options: $options) -> Result<RawSelectionBundle, SdkError> {
                self.raw_selection($selection, options).await
            }
        )*
    };
}

#[derive(Debug, Clone, Default)]
/// Options for `user` wrapper requests.
pub struct UserOptions {
    /// Shared base options.
    pub base: BaseOptions,
    /// Optional crime identifier for `crimes` endpoint variants.
    pub crime_id: Option<String>,
}

impl UserOptions {
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

    /// Sets `crimeId` for selections that require it.
    pub fn with_crime_id(mut self, value: impl Into<String>) -> Self {
        self.crime_id = Some(value.into());
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
        options
    }
}

#[derive(Debug, Clone, Copy)]
/// User resource API entrypoint.
pub struct UserApi<'a, T: HttpTransport> {
    pub(crate) client: &'a TornClient<T>,
}

impl<'a, T: HttpTransport> UserApi<'a, T> {
    /// Supported `user` selections validated by wrapper helpers.
    pub const SUPPORTED_SELECTIONS: &'static [&'static str] = &[
        "networth",
        "newevents",
        "missions",
        "money",
        "organizedcrime",
        "perks",
        "newmessages",
        "notifications",
        "list",
        "log",
        "jobpoints",
        "jobranks",
        "merits",
        "messages",
        "lookup",
        "medals",
        "personalstats",
        "stocks",
        "timestamp",
        "revivesfull",
        "skills",
        "workstats",
        "virus",
        "travel",
        "weaponexp",
        "property",
        "races",
        "profile",
        "properties",
        "reports",
        "revives",
        "racingrecords",
        "refills",
        "cooldowns",
        "crimes",
        "calendar",
        "competition",
        "display",
        "education",
        "criminalrecord",
        "discord",
        "attacksfull",
        "bars",
        "ammo",
        "attacks",
        "bazaar",
        "bounties",
        "basic",
        "battlestats",
        "enlistedcars",
        "hof",
        "honors",
        "forumthreads",
        "gym",
        "itemmarket",
        "job",
        "icons",
        "inventory",
        "faction",
        "factionbalance",
        "equipment",
        "events",
        "forumposts",
        "forumsubscribedthreads",
        "forumfeed",
        "forumfriends",
    ];

    /// Returns the underlying low-level client.
    pub fn raw_client(&self) -> &TornClient<T> {
        self.client
    }

    /// Typed helper for `user.profile`.
    pub async fn profile(&self, user_id: impl Into<String>) -> Result<UserProfileBundle, SdkError> {
        let options = UserOptions::default().with_id(user_id.into());
        execute_typed(
            self.client,
            "user",
            vec!["profile".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.basic`.
    pub async fn basic(&self, user_id: impl Into<String>) -> Result<UserBasicBundle, SdkError> {
        let options = UserOptions::default().with_id(user_id.into());
        execute_typed(
            self.client,
            "user",
            vec!["basic".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.discord`.
    pub async fn discord(&self, user_id: impl Into<String>) -> Result<UserDiscordBundle, SdkError> {
        let options = UserOptions::default().with_id(user_id.into());
        execute_typed(
            self.client,
            "user",
            vec!["discord".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.faction`.
    pub async fn faction(&self, user_id: impl Into<String>) -> Result<UserFactionBundle, SdkError> {
        let options = UserOptions::default().with_id(user_id.into());
        execute_typed(
            self.client,
            "user",
            vec!["faction".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.money`.
    pub async fn money(&self, options: UserOptions) -> Result<UserMoneyBundle, SdkError> {
        validate_options_for_selection("money", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["money".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.bars`.
    pub async fn bars(&self, options: UserOptions) -> Result<UserBarsBundle, SdkError> {
        validate_options_for_selection("bars", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["bars".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.cooldowns`.
    pub async fn cooldowns(&self, options: UserOptions) -> Result<UserCooldownsBundle, SdkError> {
        validate_options_for_selection("cooldowns", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["cooldowns".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.travel`.
    pub async fn travel(&self, options: UserOptions) -> Result<UserTravelBundle, SdkError> {
        validate_options_for_selection("travel", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["travel".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Executes a single validated selection and returns a raw JSON bundle.
    pub async fn raw_selection(
        &self,
        selection: &str,
        options: UserOptions,
    ) -> Result<RawSelectionBundle, SdkError> {
        validate_selection("user", selection, Self::SUPPORTED_SELECTIONS)?;
        validate_options_for_selection(selection, &options)?;
        execute_raw(
            self.client,
            "user",
            vec![selection.to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Executes multiple validated selections and returns a raw JSON bundle.
    pub async fn raw_selections<S, I>(
        &self,
        selections: I,
        options: UserOptions,
    ) -> Result<RawSelectionBundle, SdkError>
    where
        S: Into<String>,
        I: IntoIterator<Item = S>,
    {
        let selection_values = selections.into_iter().map(Into::into).collect::<Vec<_>>();
        validate_selections("user", &selection_values, Self::SUPPORTED_SELECTIONS)?;
        for selection in &selection_values {
            validate_options_for_selection(selection, &options)?;
        }
        execute_raw(
            self.client,
            "user",
            selection_values,
            options.into_data_request_options(),
        )
        .await
    }

    /// Executes one validated selection and deserializes into a caller type.
    pub async fn typed_selection<R>(
        &self,
        selection: &str,
        options: UserOptions,
    ) -> Result<R, SdkError>
    where
        R: serde::de::DeserializeOwned,
    {
        validate_selection("user", selection, Self::SUPPORTED_SELECTIONS)?;
        validate_options_for_selection(selection, &options)?;
        execute_typed(
            self.client,
            "user",
            vec![selection.to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    raw_selection_methods!(UserOptions;
        networth_raw => "networth",
        newevents_raw => "newevents",
        missions_raw => "missions",
        money_raw => "money",
        organizedcrime_raw => "organizedcrime",
        perks_raw => "perks",
        newmessages_raw => "newmessages",
        notifications_raw => "notifications",
        list_raw => "list",
        log_raw => "log",
        jobpoints_raw => "jobpoints",
        jobranks_raw => "jobranks",
        merits_raw => "merits",
        messages_raw => "messages",
        lookup_raw => "lookup",
        medals_raw => "medals",
        personalstats_raw => "personalstats",
        stocks_raw => "stocks",
        timestamp_raw => "timestamp",
        revivesfull_raw => "revivesfull",
        skills_raw => "skills",
        workstats_raw => "workstats",
        virus_raw => "virus",
        travel_raw => "travel",
        weaponexp_raw => "weaponexp",
        property_raw => "property",
        races_raw => "races",
        profile_raw => "profile",
        properties_raw => "properties",
        reports_raw => "reports",
        revives_raw => "revives",
        racingrecords_raw => "racingrecords",
        refills_raw => "refills",
        cooldowns_raw => "cooldowns",
        crimes_raw => "crimes",
        calendar_raw => "calendar",
        competition_raw => "competition",
        display_raw => "display",
        education_raw => "education",
        criminalrecord_raw => "criminalrecord",
        discord_raw => "discord",
        attacksfull_raw => "attacksfull",
        bars_raw => "bars",
        ammo_raw => "ammo",
        attacks_raw => "attacks",
        bazaar_raw => "bazaar",
        bounties_raw => "bounties",
        basic_raw => "basic",
        battlestats_raw => "battlestats",
        enlistedcars_raw => "enlistedcars",
        hof_raw => "hof",
        honors_raw => "honors",
        forumthreads_raw => "forumthreads",
        gym_raw => "gym",
        itemmarket_raw => "itemmarket",
        job_raw => "job",
        icons_raw => "icons",
        inventory_raw => "inventory",
        faction_raw => "faction",
        factionbalance_raw => "factionbalance",
        equipment_raw => "equipment",
        events_raw => "events",
        forumposts_raw => "forumposts",
        forumsubscribedthreads_raw => "forumsubscribedthreads",
        forumfeed_raw => "forumfeed",
        forumfriends_raw => "forumfriends",
    );
}

fn validate_options_for_selection(selection: &str, options: &UserOptions) -> Result<(), SdkError> {
    validate_range("user", options.base.from, options.base.to)?;

    if requires_user_id(selection) {
        validate_required_path_arg("user", selection, "id", options.base.id.is_some())?;
    }

    if selection == "crimes" {
        validate_required_path_arg(
            "user",
            selection,
            "crimeId",
            options.crime_id.is_some() || options.base.id.is_some(),
        )?;
    }

    Ok(())
}

fn requires_user_id(selection: &str) -> bool {
    matches!(
        selection,
        "basic"
            | "bounties"
            | "competition"
            | "discord"
            | "faction"
            | "forumposts"
            | "forumthreads"
            | "hof"
            | "icons"
            | "job"
            | "personalstats"
            | "profile"
            | "properties"
            | "property"
    )
}
