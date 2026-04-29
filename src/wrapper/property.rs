//! Property resource wrapper APIs.

use crate::client::{DataRequestOptions, TornClient};
use crate::models::generated::RawSelectionBundle;
use crate::models::manual::property::{
    PropertyBundle, PropertyLookupBundle, PropertyTimestampBundle,
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
            #[doc = concat!("Convenience helper for raw `property.", $selection, "`.")]
            pub async fn $method(&self, options: $options) -> Result<RawSelectionBundle, SdkError> {
                self.raw_selection($selection, options).await
            }
        )*
    };
}

#[derive(Debug, Clone, Default)]
/// Options for `property` wrapper requests.
pub struct PropertyOptions {
    /// Shared base options.
    pub base: BaseOptions,
}

impl PropertyOptions {
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

    pub(crate) fn into_data_request_options(self) -> DataRequestOptions {
        self.base.into_data_request_options()
    }
}

#[derive(Debug, Clone, Copy)]
/// Property resource API entrypoint.
pub struct PropertyApi<'a, T: HttpTransport> {
    pub(crate) client: &'a TornClient<T>,
}

impl<T: HttpTransport> PropertyApi<'_, T> {
    /// Supported `property` selections validated by wrapper helpers.
    pub const SUPPORTED_SELECTIONS: &'static [&'static str] = &["timestamp", "property", "lookup"];

    /// Returns the underlying low-level client.
    pub fn raw_client(&self) -> &TornClient<T> {
        self.client
    }

    /// Typed helper for `property.property`.
    pub async fn property(&self, options: PropertyOptions) -> Result<PropertyBundle, SdkError> {
        validate_options_for_selection("property", &options)?;
        execute_typed(
            self.client,
            "property",
            vec!["property".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `property.lookup`.
    pub async fn lookup(&self, options: PropertyOptions) -> Result<PropertyLookupBundle, SdkError> {
        validate_options_for_selection("lookup", &options)?;
        execute_typed(
            self.client,
            "property",
            vec!["lookup".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `property.timestamp`.
    pub async fn timestamp(
        &self,
        options: PropertyOptions,
    ) -> Result<PropertyTimestampBundle, SdkError> {
        validate_options_for_selection("timestamp", &options)?;
        execute_typed(
            self.client,
            "property",
            vec!["timestamp".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Executes a single validated selection and returns a raw JSON bundle.
    pub async fn raw_selection(
        &self,
        selection: &str,
        options: PropertyOptions,
    ) -> Result<RawSelectionBundle, SdkError> {
        validate_selection("property", selection, Self::SUPPORTED_SELECTIONS)?;
        validate_options_for_selection(selection, &options)?;
        execute_raw(
            self.client,
            "property",
            vec![selection.to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Executes multiple validated selections and returns a raw JSON bundle.
    pub async fn raw_selections<S, I>(
        &self,
        selections: I,
        options: PropertyOptions,
    ) -> Result<RawSelectionBundle, SdkError>
    where
        S: Into<String>,
        I: IntoIterator<Item = S>,
    {
        let selection_values = selections.into_iter().map(Into::into).collect::<Vec<_>>();
        validate_selections("property", &selection_values, Self::SUPPORTED_SELECTIONS)?;
        for selection in &selection_values {
            validate_options_for_selection(selection, &options)?;
        }
        execute_raw(
            self.client,
            "property",
            selection_values,
            options.into_data_request_options(),
        )
        .await
    }

    /// Executes one validated selection and deserializes into a caller type.
    pub async fn typed_selection<R>(
        &self,
        selection: &str,
        options: PropertyOptions,
    ) -> Result<R, SdkError>
    where
        R: serde::de::DeserializeOwned,
    {
        validate_selection("property", selection, Self::SUPPORTED_SELECTIONS)?;
        validate_options_for_selection(selection, &options)?;
        execute_typed(
            self.client,
            "property",
            vec![selection.to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    raw_selection_methods!(PropertyOptions;
        timestamp_raw => "timestamp",
        property_raw => "property",
        lookup_raw => "lookup",
    );
}

fn validate_options_for_selection(
    selection: &str,
    options: &PropertyOptions,
) -> Result<(), SdkError> {
    validate_range("property", options.base.from, options.base.to)?;

    if selection == "property" {
        validate_required_path_arg("property", selection, "id", options.base.id.is_some())?;
    }

    Ok(())
}
