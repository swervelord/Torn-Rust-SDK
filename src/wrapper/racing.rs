//! Racing resource wrapper APIs.

use crate::client::{DataRequestOptions, TornClient};
use crate::models::generated::RawSelectionBundle;
use crate::models::manual::racing::{
    RacingCarUpgradesBundle, RacingCarsBundle, RacingLookupBundle, RacingRaceBundle,
    RacingRacesBundle, RacingRecordsBundle, RacingTimestampBundle, RacingTracksBundle,
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
            #[doc = concat!("Convenience helper for raw `racing.", $selection, "`.")]
            pub async fn $method(&self, options: $options) -> Result<RawSelectionBundle, SdkError> {
                self.raw_selection($selection, options).await
            }
        )*
    };
}

#[derive(Debug, Clone, Default)]
/// Options for `racing` wrapper requests.
pub struct RacingOptions {
    /// Shared base options.
    pub base: BaseOptions,
    /// Optional `raceId`.
    pub race_id: Option<String>,
    /// Optional `trackId`.
    pub track_id: Option<String>,
}

impl RacingOptions {
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

    /// Sets `raceId`.
    pub fn with_race_id(mut self, value: impl Into<String>) -> Self {
        self.race_id = Some(value.into());
        self
    }

    /// Sets `trackId`.
    pub fn with_track_id(mut self, value: impl Into<String>) -> Self {
        self.track_id = Some(value.into());
        self
    }

    pub(crate) fn into_data_request_options(self) -> DataRequestOptions {
        let mut options = self.base.into_data_request_options();
        if let Some(value) = self.race_id {
            if options.id.is_none() {
                options = options.with_id(value.clone());
            }
            options = options.with_path_arg("raceId", value);
        }
        if let Some(value) = self.track_id {
            if options.id.is_none() {
                options = options.with_id(value.clone());
            }
            options = options.with_path_arg("trackId", value);
        }
        options
    }
}

#[derive(Debug, Clone, Copy)]
/// Racing resource API entrypoint.
pub struct RacingApi<'a, T: HttpTransport> {
    pub(crate) client: &'a TornClient<T>,
}

impl<T: HttpTransport> RacingApi<'_, T> {
    /// Supported `racing` selections validated by wrapper helpers.
    pub const SUPPORTED_SELECTIONS: &'static [&'static str] = &[
        "records",
        "races",
        "tracks",
        "timestamp",
        "carupgrades",
        "cars",
        "race",
        "lookup",
    ];

    /// Returns the underlying low-level client.
    pub fn raw_client(&self) -> &TornClient<T> {
        self.client
    }

    /// Typed helper for `racing.race`.
    pub async fn race(&self, options: RacingOptions) -> Result<RacingRaceBundle, SdkError> {
        validate_options_for_selection("race", &options)?;
        execute_typed(
            self.client,
            "racing",
            vec!["race".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `racing.races`.
    pub async fn races(&self, options: RacingOptions) -> Result<RacingRacesBundle, SdkError> {
        validate_options_for_selection("races", &options)?;
        execute_typed(
            self.client,
            "racing",
            vec!["races".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `racing.records`.
    pub async fn records(&self, options: RacingOptions) -> Result<RacingRecordsBundle, SdkError> {
        validate_options_for_selection("records", &options)?;
        execute_typed(
            self.client,
            "racing",
            vec!["records".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `racing.tracks`.
    pub async fn tracks(&self, options: RacingOptions) -> Result<RacingTracksBundle, SdkError> {
        validate_options_for_selection("tracks", &options)?;
        execute_typed(
            self.client,
            "racing",
            vec!["tracks".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `racing.cars`.
    pub async fn cars(&self, options: RacingOptions) -> Result<RacingCarsBundle, SdkError> {
        validate_options_for_selection("cars", &options)?;
        execute_typed(
            self.client,
            "racing",
            vec!["cars".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `racing.carupgrades`.
    pub async fn carupgrades(
        &self,
        options: RacingOptions,
    ) -> Result<RacingCarUpgradesBundle, SdkError> {
        validate_options_for_selection("carupgrades", &options)?;
        execute_typed(
            self.client,
            "racing",
            vec!["carupgrades".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `racing.lookup`.
    pub async fn lookup(&self, options: RacingOptions) -> Result<RacingLookupBundle, SdkError> {
        validate_options_for_selection("lookup", &options)?;
        execute_typed(
            self.client,
            "racing",
            vec!["lookup".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `racing.timestamp`.
    pub async fn timestamp(
        &self,
        options: RacingOptions,
    ) -> Result<RacingTimestampBundle, SdkError> {
        validate_options_for_selection("timestamp", &options)?;
        execute_typed(
            self.client,
            "racing",
            vec!["timestamp".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Executes a single validated selection and returns a raw JSON bundle.
    pub async fn raw_selection(
        &self,
        selection: &str,
        options: RacingOptions,
    ) -> Result<RawSelectionBundle, SdkError> {
        validate_selection("racing", selection, Self::SUPPORTED_SELECTIONS)?;
        validate_options_for_selection(selection, &options)?;
        execute_raw(
            self.client,
            "racing",
            vec![selection.to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Executes multiple validated selections and returns a raw JSON bundle.
    pub async fn raw_selections<S, I>(
        &self,
        selections: I,
        options: RacingOptions,
    ) -> Result<RawSelectionBundle, SdkError>
    where
        S: Into<String>,
        I: IntoIterator<Item = S>,
    {
        let selection_values = selections.into_iter().map(Into::into).collect::<Vec<_>>();
        validate_selections("racing", &selection_values, Self::SUPPORTED_SELECTIONS)?;
        for selection in &selection_values {
            validate_options_for_selection(selection, &options)?;
        }
        execute_raw(
            self.client,
            "racing",
            selection_values,
            options.into_data_request_options(),
        )
        .await
    }

    /// Executes one validated selection and deserializes into a caller type.
    pub async fn typed_selection<R>(
        &self,
        selection: &str,
        options: RacingOptions,
    ) -> Result<R, SdkError>
    where
        R: serde::de::DeserializeOwned,
    {
        validate_selection("racing", selection, Self::SUPPORTED_SELECTIONS)?;
        validate_options_for_selection(selection, &options)?;
        execute_typed(
            self.client,
            "racing",
            vec![selection.to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    raw_selection_methods!(RacingOptions;
        records_raw => "records",
        races_raw => "races",
        tracks_raw => "tracks",
        timestamp_raw => "timestamp",
        carupgrades_raw => "carupgrades",
        cars_raw => "cars",
        race_raw => "race",
        lookup_raw => "lookup",
    );
}

fn validate_options_for_selection(
    selection: &str,
    options: &RacingOptions,
) -> Result<(), SdkError> {
    validate_range("racing", options.base.from, options.base.to)?;

    match selection {
        "race" => {
            validate_required_path_arg(
                "racing",
                selection,
                "raceId",
                options.race_id.is_some() || options.base.id.is_some(),
            )?;
        }
        "records" => {
            validate_required_path_arg(
                "racing",
                selection,
                "trackId",
                options.track_id.is_some() || options.base.id.is_some(),
            )?;
            validate_required_query_arg("racing", selection, "cat", options.base.cat.is_some())?;
        }
        _ => {}
    }

    Ok(())
}
