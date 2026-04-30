//! Company resource wrapper APIs.

use crate::client::{DataRequestOptions, TornClient};
use crate::models::generated::RawSelectionBundle;
use crate::models::manual::company::{
    CompanyApplicationsBundle, CompanyCompaniesBundle, CompanyDetailedBundle,
    CompanyEmployeesBundle, CompanyLookupBundle, CompanyNewsBundle, CompanyProfileBundle,
    CompanySearchBundle, CompanyStockBundle, CompanyTimestampBundle,
};
use crate::transport::HttpTransport;
use crate::wrapper::error::SdkError;
use crate::wrapper::internal::{
    execute_raw, execute_typed, validate_range, validate_selection, validate_selections,
};
use crate::wrapper::options::BaseOptions;

macro_rules! raw_selection_methods {
    ($options:ty; $($method:ident => $selection:literal),* $(,)?) => {
        $(
            #[doc = concat!("Convenience helper for raw `company.", $selection, "`.")]
            pub async fn $method(&self, options: $options) -> Result<RawSelectionBundle, SdkError> {
                self.raw_selection($selection, options).await
            }
        )*
    };
}

#[derive(Debug, Clone, Default)]
/// Options for `company` wrapper requests.
pub struct CompanyOptions {
    /// Shared base options.
    pub base: BaseOptions,
}

impl CompanyOptions {
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
/// Company resource API entrypoint.
pub struct CompanyApi<'a, T: HttpTransport> {
    pub(crate) client: &'a TornClient<T>,
}

impl<T: HttpTransport> CompanyApi<'_, T> {
    /// Supported `company` selections validated by wrapper helpers.
    pub const SUPPORTED_SELECTIONS: &'static [&'static str] = &[
        "applications",
        "companies",
        "employees",
        "lookup",
        "news",
        "profile",
        "search",
        "stock",
        "timestamp",
    ];

    /// Returns the underlying low-level client.
    pub fn raw_client(&self) -> &TornClient<T> {
        self.client
    }

    /// Typed helper for `company.lookup`.
    pub async fn lookup(&self, options: CompanyOptions) -> Result<CompanyLookupBundle, SdkError> {
        validate_options_for_selection(&options)?;
        execute_typed(
            self.client,
            "company",
            vec!["lookup".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `company.applications`.
    pub async fn applications(
        &self,
        options: CompanyOptions,
    ) -> Result<CompanyApplicationsBundle, SdkError> {
        validate_options_for_selection(&options)?;
        execute_typed(
            self.client,
            "company",
            vec!["applications".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `company.companies`.
    pub async fn companies(
        &self,
        options: CompanyOptions,
    ) -> Result<CompanyCompaniesBundle, SdkError> {
        validate_options_for_selection(&options)?;
        execute_typed(
            self.client,
            "company",
            vec!["companies".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `company.timestamp`.
    pub async fn timestamp(
        &self,
        options: CompanyOptions,
    ) -> Result<CompanyTimestampBundle, SdkError> {
        validate_options_for_selection(&options)?;
        execute_typed(
            self.client,
            "company",
            vec!["timestamp".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `company.profile`.
    pub async fn profile(&self, options: CompanyOptions) -> Result<CompanyProfileBundle, SdkError> {
        validate_options_for_selection(&options)?;
        execute_typed(
            self.client,
            "company",
            vec!["profile".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Compatibility helper for the removed `company.detailed` selection.
    ///
    /// Torn API v2 5.8.0 folded details into `company.profile`, so this method now requests
    /// `profile` and returns the profile bundle.
    pub async fn detailed(
        &self,
        options: CompanyOptions,
    ) -> Result<CompanyDetailedBundle, SdkError> {
        self.profile(options).await
    }

    /// Typed helper for `company.employees`.
    pub async fn employees(
        &self,
        options: CompanyOptions,
    ) -> Result<CompanyEmployeesBundle, SdkError> {
        validate_options_for_selection(&options)?;
        execute_typed(
            self.client,
            "company",
            vec!["employees".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `company.news`.
    pub async fn news(&self, options: CompanyOptions) -> Result<CompanyNewsBundle, SdkError> {
        validate_options_for_selection(&options)?;
        execute_typed(
            self.client,
            "company",
            vec!["news".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `company.search`.
    pub async fn search(&self, options: CompanyOptions) -> Result<CompanySearchBundle, SdkError> {
        validate_options_for_selection(&options)?;
        execute_typed(
            self.client,
            "company",
            vec!["search".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `company.stock`.
    pub async fn stock(&self, options: CompanyOptions) -> Result<CompanyStockBundle, SdkError> {
        validate_options_for_selection(&options)?;
        execute_typed(
            self.client,
            "company",
            vec!["stock".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Executes a single validated selection and returns a raw JSON bundle.
    pub async fn raw_selection(
        &self,
        selection: &str,
        options: CompanyOptions,
    ) -> Result<RawSelectionBundle, SdkError> {
        validate_selection("company", selection, Self::SUPPORTED_SELECTIONS)?;
        validate_options_for_selection(&options)?;
        execute_raw(
            self.client,
            "company",
            vec![selection.to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Executes multiple validated selections and returns a raw JSON bundle.
    pub async fn raw_selections<S, I>(
        &self,
        selections: I,
        options: CompanyOptions,
    ) -> Result<RawSelectionBundle, SdkError>
    where
        S: Into<String>,
        I: IntoIterator<Item = S>,
    {
        let selection_values = selections.into_iter().map(Into::into).collect::<Vec<_>>();
        validate_selections("company", &selection_values, Self::SUPPORTED_SELECTIONS)?;
        validate_options_for_selection(&options)?;
        execute_raw(
            self.client,
            "company",
            selection_values,
            options.into_data_request_options(),
        )
        .await
    }

    /// Executes one validated selection and deserializes into a caller type.
    pub async fn typed_selection<R>(
        &self,
        selection: &str,
        options: CompanyOptions,
    ) -> Result<R, SdkError>
    where
        R: serde::de::DeserializeOwned,
    {
        validate_selection("company", selection, Self::SUPPORTED_SELECTIONS)?;
        validate_options_for_selection(&options)?;
        execute_typed(
            self.client,
            "company",
            vec![selection.to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    raw_selection_methods!(CompanyOptions;
        applications_raw => "applications",
        companies_raw => "companies",
        employees_raw => "employees",
        lookup_raw => "lookup",
        news_raw => "news",
        profile_raw => "profile",
        search_raw => "search",
        stock_raw => "stock",
        timestamp_raw => "timestamp",
    );
}

fn validate_options_for_selection(options: &CompanyOptions) -> Result<(), SdkError> {
    validate_range("company", options.base.from, options.base.to)
}
