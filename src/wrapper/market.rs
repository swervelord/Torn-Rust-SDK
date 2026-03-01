//! Market resource wrapper APIs.

use crate::client::{DataRequestOptions, TornClient};
use crate::models::generated::RawSelectionBundle;
use crate::models::manual::market::{MarketBazaarBundle, MarketItemMarketBundle};
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
            #[doc = concat!("Convenience helper for raw `market.", $selection, "`.")]
            pub async fn $method(&self, options: $options) -> Result<RawSelectionBundle, SdkError> {
                self.raw_selection($selection, options).await
            }
        )*
    };
}

#[derive(Debug, Clone, Default)]
/// Options for `market` wrapper requests.
pub struct MarketOptions {
    /// Shared base options.
    pub base: BaseOptions,
    /// Optional `propertyTypeId` for property/rental market endpoints.
    pub property_type_id: Option<String>,
}

impl MarketOptions {
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

    /// Sets `propertyTypeId`.
    pub fn with_property_type_id(mut self, value: impl Into<String>) -> Self {
        self.property_type_id = Some(value.into());
        self
    }

    pub(crate) fn into_data_request_options(self) -> DataRequestOptions {
        let mut options = self.base.into_data_request_options();
        if let Some(value) = self.property_type_id {
            if options.id.is_none() {
                options = options.with_id(value.clone());
            }
            options = options.with_path_arg("propertyTypeId", value);
        }
        options
    }
}

#[derive(Debug, Clone, Copy)]
/// Market resource API entrypoint.
pub struct MarketApi<'a, T: HttpTransport> {
    pub(crate) client: &'a TornClient<T>,
}

impl<'a, T: HttpTransport> MarketApi<'a, T> {
    /// Supported `market` selections validated by wrapper helpers.
    pub const SUPPORTED_SELECTIONS: &'static [&'static str] = &[
        "properties",
        "pointsmarket",
        "timestamp",
        "rentals",
        "lookup",
        "auctionhouselisting",
        "auctionhouse",
        "itemmarket",
        "bazaar",
    ];

    /// Returns the underlying low-level client.
    pub fn raw_client(&self) -> &TornClient<T> {
        self.client
    }

    /// Typed helper for `market.bazaar`.
    pub async fn bazaar(&self, options: MarketOptions) -> Result<MarketBazaarBundle, SdkError> {
        validate_options_for_selection("bazaar", &options)?;
        execute_typed(
            self.client,
            "market",
            vec!["bazaar".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `market.itemmarket`.
    pub async fn itemmarket(
        &self,
        options: MarketOptions,
    ) -> Result<MarketItemMarketBundle, SdkError> {
        validate_options_for_selection("itemmarket", &options)?;
        execute_typed(
            self.client,
            "market",
            vec!["itemmarket".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Executes a single validated selection and returns a raw JSON bundle.
    pub async fn raw_selection(
        &self,
        selection: &str,
        options: MarketOptions,
    ) -> Result<RawSelectionBundle, SdkError> {
        validate_selection("market", selection, Self::SUPPORTED_SELECTIONS)?;
        validate_options_for_selection(selection, &options)?;
        execute_raw(
            self.client,
            "market",
            vec![selection.to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Executes multiple validated selections and returns a raw JSON bundle.
    pub async fn raw_selections<S, I>(
        &self,
        selections: I,
        options: MarketOptions,
    ) -> Result<RawSelectionBundle, SdkError>
    where
        S: Into<String>,
        I: IntoIterator<Item = S>,
    {
        let selection_values = selections.into_iter().map(Into::into).collect::<Vec<_>>();
        validate_selections("market", &selection_values, Self::SUPPORTED_SELECTIONS)?;
        for selection in &selection_values {
            validate_options_for_selection(selection, &options)?;
        }
        execute_raw(
            self.client,
            "market",
            selection_values,
            options.into_data_request_options(),
        )
        .await
    }

    /// Executes one validated selection and deserializes into a caller type.
    pub async fn typed_selection<R>(
        &self,
        selection: &str,
        options: MarketOptions,
    ) -> Result<R, SdkError>
    where
        R: serde::de::DeserializeOwned,
    {
        validate_selection("market", selection, Self::SUPPORTED_SELECTIONS)?;
        validate_options_for_selection(selection, &options)?;
        execute_typed(
            self.client,
            "market",
            vec![selection.to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    raw_selection_methods!(MarketOptions;
        properties_raw => "properties",
        pointsmarket_raw => "pointsmarket",
        timestamp_raw => "timestamp",
        rentals_raw => "rentals",
        lookup_raw => "lookup",
        auctionhouselisting_raw => "auctionhouselisting",
        auctionhouse_raw => "auctionhouse",
        itemmarket_raw => "itemmarket",
        bazaar_raw => "bazaar",
    );
}

fn validate_options_for_selection(
    selection: &str,
    options: &MarketOptions,
) -> Result<(), SdkError> {
    validate_range("market", options.base.from, options.base.to)?;

    if matches!(selection, "properties" | "rentals") {
        validate_required_path_arg(
            "market",
            selection,
            "propertyTypeId",
            options.property_type_id.is_some() || options.base.id.is_some(),
        )?;
    }

    if matches!(
        selection,
        "auctionhouse" | "auctionhouselisting" | "bazaar" | "itemmarket"
    ) {
        validate_required_path_arg("market", selection, "id", options.base.id.is_some())?;
    }

    Ok(())
}
