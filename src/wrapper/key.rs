//! Key resource wrapper APIs.

use crate::client::{DataRequestOptions, TornClient};
use crate::models::generated::RawSelectionBundle;
use crate::models::manual::key::KeyInfoBundle;
use crate::transport::HttpTransport;
use crate::wrapper::error::SdkError;
use crate::wrapper::internal::{
    execute_raw, execute_typed, validate_range, validate_selection, validate_selections,
};
use crate::wrapper::options::BaseOptions;

macro_rules! raw_selection_methods {
    ($options:ty; $($method:ident => $selection:literal),* $(,)?) => {
        $(
            #[doc = concat!("Convenience helper for raw `key.", $selection, "`.")]
            pub async fn $method(&self, options: $options) -> Result<RawSelectionBundle, SdkError> {
                self.raw_selection($selection, options).await
            }
        )*
    };
}

#[derive(Debug, Clone, Default)]
/// Options for `key` wrapper requests.
pub struct KeyOptions {
    /// Shared base options.
    pub base: BaseOptions,
}

impl KeyOptions {
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
/// Key resource API entrypoint.
pub struct KeyApi<'a, T: HttpTransport> {
    pub(crate) client: &'a TornClient<T>,
}

impl<T: HttpTransport> KeyApi<'_, T> {
    /// Supported `key` selections validated by wrapper helpers.
    pub const SUPPORTED_SELECTIONS: &'static [&'static str] = &["log", "info"];

    /// Returns the underlying low-level client.
    pub fn raw_client(&self) -> &TornClient<T> {
        self.client
    }

    /// Typed helper for `key.info`.
    pub async fn info(&self, options: KeyOptions) -> Result<KeyInfoBundle, SdkError> {
        validate_options_for_selection(&options)?;
        execute_typed(
            self.client,
            "key",
            vec!["info".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Executes a single validated selection and returns a raw JSON bundle.
    pub async fn raw_selection(
        &self,
        selection: &str,
        options: KeyOptions,
    ) -> Result<RawSelectionBundle, SdkError> {
        validate_selection("key", selection, Self::SUPPORTED_SELECTIONS)?;
        validate_options_for_selection(&options)?;
        execute_raw(
            self.client,
            "key",
            vec![selection.to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Executes multiple validated selections and returns a raw JSON bundle.
    pub async fn raw_selections<S, I>(
        &self,
        selections: I,
        options: KeyOptions,
    ) -> Result<RawSelectionBundle, SdkError>
    where
        S: Into<String>,
        I: IntoIterator<Item = S>,
    {
        let selection_values = selections.into_iter().map(Into::into).collect::<Vec<_>>();
        validate_selections("key", &selection_values, Self::SUPPORTED_SELECTIONS)?;
        validate_options_for_selection(&options)?;
        execute_raw(
            self.client,
            "key",
            selection_values,
            options.into_data_request_options(),
        )
        .await
    }

    /// Executes one validated selection and deserializes into a caller type.
    pub async fn typed_selection<R>(
        &self,
        selection: &str,
        options: KeyOptions,
    ) -> Result<R, SdkError>
    where
        R: serde::de::DeserializeOwned,
    {
        validate_selection("key", selection, Self::SUPPORTED_SELECTIONS)?;
        validate_options_for_selection(&options)?;
        execute_typed(
            self.client,
            "key",
            vec![selection.to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    raw_selection_methods!(KeyOptions;
        log_raw => "log",
        info_raw => "info",
    );
}

fn validate_options_for_selection(options: &KeyOptions) -> Result<(), SdkError> {
    validate_range("key", options.base.from, options.base.to)
}
