//! Forum resource wrapper APIs.

use crate::client::{DataRequestOptions, TornClient};
use crate::models::generated::RawSelectionBundle;
use crate::models::manual::forum::ForumThreadBundle;
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
            #[doc = concat!("Convenience helper for raw `forum.", $selection, "`.")]
            pub async fn $method(&self, options: $options) -> Result<RawSelectionBundle, SdkError> {
                self.raw_selection($selection, options).await
            }
        )*
    };
}

#[derive(Debug, Clone, Default)]
/// Options for `forum` wrapper requests.
pub struct ForumOptions {
    /// Shared base options.
    pub base: BaseOptions,
    /// Optional category id list for `threads`.
    pub category_ids: Option<String>,
    /// Optional thread id for `thread`/`posts`.
    pub thread_id: Option<String>,
}

impl ForumOptions {
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

    /// Sets `categoryIds`.
    pub fn with_category_ids(mut self, value: impl Into<String>) -> Self {
        self.category_ids = Some(value.into());
        self
    }

    /// Sets `threadId`.
    pub fn with_thread_id(mut self, value: impl Into<String>) -> Self {
        self.thread_id = Some(value.into());
        self
    }

    pub(crate) fn into_data_request_options(self) -> DataRequestOptions {
        let mut options = self.base.into_data_request_options();
        if let Some(value) = self.category_ids {
            if options.id.is_none() {
                options = options.with_id(value.clone());
            }
            options = options.with_path_arg("categoryIds", value);
        }
        if let Some(value) = self.thread_id {
            if options.id.is_none() {
                options = options.with_id(value.clone());
            }
            options = options.with_path_arg("threadId", value);
        }
        options
    }
}

#[derive(Debug, Clone, Copy)]
/// Forum resource API entrypoint.
pub struct ForumApi<'a, T: HttpTransport> {
    pub(crate) client: &'a TornClient<T>,
}

impl<'a, T: HttpTransport> ForumApi<'a, T> {
    /// Supported `forum` selections validated by wrapper helpers.
    pub const SUPPORTED_SELECTIONS: &'static [&'static str] = &[
        "thread",
        "threads",
        "timestamp",
        "categories",
        "lookup",
        "posts",
    ];

    /// Returns the underlying low-level client.
    pub fn raw_client(&self) -> &TornClient<T> {
        self.client
    }

    /// Typed helper for `forum.thread`.
    pub async fn thread(&self, options: ForumOptions) -> Result<ForumThreadBundle, SdkError> {
        validate_options_for_selection("thread", &options)?;
        execute_typed(
            self.client,
            "forum",
            vec!["thread".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Executes a single validated selection and returns a raw JSON bundle.
    pub async fn raw_selection(
        &self,
        selection: &str,
        options: ForumOptions,
    ) -> Result<RawSelectionBundle, SdkError> {
        validate_selection("forum", selection, Self::SUPPORTED_SELECTIONS)?;
        validate_options_for_selection(selection, &options)?;
        execute_raw(
            self.client,
            "forum",
            vec![selection.to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Executes multiple validated selections and returns a raw JSON bundle.
    pub async fn raw_selections<S, I>(
        &self,
        selections: I,
        options: ForumOptions,
    ) -> Result<RawSelectionBundle, SdkError>
    where
        S: Into<String>,
        I: IntoIterator<Item = S>,
    {
        let selection_values = selections.into_iter().map(Into::into).collect::<Vec<_>>();
        validate_selections("forum", &selection_values, Self::SUPPORTED_SELECTIONS)?;
        for selection in &selection_values {
            validate_options_for_selection(selection, &options)?;
        }
        execute_raw(
            self.client,
            "forum",
            selection_values,
            options.into_data_request_options(),
        )
        .await
    }

    /// Executes one validated selection and deserializes into a caller type.
    pub async fn typed_selection<R>(
        &self,
        selection: &str,
        options: ForumOptions,
    ) -> Result<R, SdkError>
    where
        R: serde::de::DeserializeOwned,
    {
        validate_selection("forum", selection, Self::SUPPORTED_SELECTIONS)?;
        validate_options_for_selection(selection, &options)?;
        execute_typed(
            self.client,
            "forum",
            vec![selection.to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    raw_selection_methods!(ForumOptions;
        thread_raw => "thread",
        threads_raw => "threads",
        timestamp_raw => "timestamp",
        categories_raw => "categories",
        lookup_raw => "lookup",
        posts_raw => "posts",
    );
}

fn validate_options_for_selection(selection: &str, options: &ForumOptions) -> Result<(), SdkError> {
    validate_range("forum", options.base.from, options.base.to)?;

    if matches!(selection, "thread" | "posts") {
        validate_required_path_arg(
            "forum",
            selection,
            "threadId",
            options.thread_id.is_some() || options.base.id.is_some(),
        )?;
    }

    if selection == "threads" {
        validate_required_path_arg(
            "forum",
            selection,
            "categoryIds",
            options.category_ids.is_some() || options.base.id.is_some(),
        )?;
    }

    Ok(())
}
