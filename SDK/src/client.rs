//! High-level client API for planned Torn resource requests.

use std::collections::BTreeMap;
use std::path::Path;

use serde::de::DeserializeOwned;
use thiserror::Error;

use crate::CapabilitiesDocument;
use crate::env_config::EnvConfigError;
use crate::error::PlannerError;
use crate::executor::{
    ExecutionOptions, ExecutionReport, ExecutorConfig, ExecutorError, RequestExecutor,
};
use crate::planner::{PlanRequest, RequestPlanner};
use crate::transport::{HttpTransport, ReqwestTransport};

#[derive(Debug, Clone, Default)]
/// Optional request controls applied by high-level client helpers.
pub struct DataRequestOptions {
    /// Resource identifier used by generic or direct endpoints.
    pub id: Option<String>,
    /// Query filter values merged into planner requests when supported.
    pub filters: BTreeMap<String, String>,
    /// Explicit path template arguments for direct endpoint routing.
    pub path_args: BTreeMap<String, String>,
    /// Legacy selections to request through `legacy` when supported.
    pub legacy_selections: Vec<String>,
    /// Execution overrides (attempts, timeout) for this request.
    pub execution_options: ExecutionOptions,
}

impl DataRequestOptions {
    /// Sets a resource `id` value.
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Adds a query filter (`name=value`) if the planner/endpoint permits it.
    pub fn with_filter(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.filters.insert(name.into(), value.into());
        self
    }

    /// Adds a direct endpoint path argument.
    pub fn with_path_arg(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.path_args.insert(name.into(), value.into());
        self
    }

    /// Adds a legacy selection to pass through the `legacy` parameter.
    pub fn with_legacy_selection(mut self, selection: impl Into<String>) -> Self {
        self.legacy_selections.push(selection.into());
        self
    }

    /// Replaces execution options for this request.
    pub fn with_execution_options(mut self, execution_options: ExecutionOptions) -> Self {
        self.execution_options = execution_options;
        self
    }

    /// Overrides maximum execution attempts for this request.
    pub fn with_max_attempts(mut self, max_attempts: u32) -> Self {
        self.execution_options.max_attempts = Some(max_attempts);
        self
    }

    /// Overrides per-request timeout for this request.
    pub fn with_request_timeout(mut self, timeout: std::time::Duration) -> Self {
        self.execution_options.request_timeout = Some(timeout);
        self
    }
}

#[derive(Debug, Error)]
/// High-level client error covering config, planner, executor, and decode failures.
pub enum ClientError {
    /// Environment loading failed while constructing a client from env.
    #[error("environment configuration failed: {0}")]
    Env(#[from] EnvConfigError),
    /// Planner construction or planning failed.
    #[error("planner initialization failed: {0}")]
    Planner(#[from] PlannerError),
    /// Request execution failed.
    #[error("execution failed: {0}")]
    Executor(#[from] ExecutorError),
    /// Response JSON could not be deserialized into the requested type.
    #[error("response deserialization failed: {0}")]
    Deserialize(#[from] serde_json::Error),
}

#[derive(Debug)]
/// Planner + executor client for direct resource/selection requests.
pub struct TornClient<T: HttpTransport> {
    planner: RequestPlanner,
    executor: RequestExecutor<T>,
}

impl<T: HttpTransport> TornClient<T> {
    /// Creates a client from an initialized planner and executor.
    pub fn new(planner: RequestPlanner, executor: RequestExecutor<T>) -> Self {
        Self { planner, executor }
    }

    /// Returns the underlying request planner.
    pub fn planner(&self) -> &RequestPlanner {
        &self.planner
    }

    /// Returns the underlying request executor.
    pub fn executor(&self) -> &RequestExecutor<T> {
        &self.executor
    }

    /// Executes a fully specified planning request with default execution options.
    pub async fn execute(&self, request: PlanRequest) -> Result<ExecutionReport, ClientError> {
        self.execute_with_options(request, ExecutionOptions::default())
            .await
    }

    /// Executes a planning request with explicit execution options.
    pub async fn execute_with_options(
        &self,
        request: PlanRequest,
        execution_options: ExecutionOptions,
    ) -> Result<ExecutionReport, ClientError> {
        self.executor
            .plan_and_execute_with_options(&self.planner, &request, execution_options)
            .await
            .map_err(ClientError::Executor)
    }

    /// Convenience helper for `user` resource data requests.
    pub async fn get_user_data<S, I>(
        &self,
        selections: I,
        options: DataRequestOptions,
    ) -> Result<ExecutionReport, ClientError>
    where
        S: Into<String>,
        I: IntoIterator<Item = S>,
    {
        self.get_resource_data("user", selections, options).await
    }

    /// Executes resource selections and returns an [`ExecutionReport`].
    pub async fn get_resource_data<S, I>(
        &self,
        resource: &str,
        selections: I,
        options: DataRequestOptions,
    ) -> Result<ExecutionReport, ClientError>
    where
        S: Into<String>,
        I: IntoIterator<Item = S>,
    {
        let selection_values = selections.into_iter().map(Into::into).collect::<Vec<_>>();
        let execution_options = options.execution_options.clone();
        let request = build_plan_request(resource, selection_values, options);
        self.execute_with_options(request, execution_options).await
    }

    /// Executes resource selections and deserializes merged JSON into `R`.
    pub async fn get_resource_data_typed<R, S, I>(
        &self,
        resource: &str,
        selections: I,
        options: DataRequestOptions,
    ) -> Result<R, ClientError>
    where
        R: DeserializeOwned,
        S: Into<String>,
        I: IntoIterator<Item = S>,
    {
        let report = self
            .get_resource_data(resource, selections, options)
            .await?;
        report.deserialize().map_err(ClientError::Deserialize)
    }
}

impl TornClient<ReqwestTransport> {
    /// Loads runtime configuration from environment variables and `.env`.
    pub fn from_env(capabilities_path: impl AsRef<Path>) -> Result<Self, ClientError> {
        let runtime = crate::env_config::RuntimeEnvConfig::from_env()?;
        Self::from_capabilities_file(capabilities_path, runtime.api_keys, runtime.executor_config)
    }

    /// Creates a client from a capabilities JSON file and API key set.
    pub fn from_capabilities_file(
        capabilities_path: impl AsRef<Path>,
        api_keys: Vec<String>,
        config: ExecutorConfig,
    ) -> Result<Self, ClientError> {
        let planner = RequestPlanner::from_capabilities_file(capabilities_path)?;
        let executor = RequestExecutor::with_default_transport(api_keys, config)
            .map_err(ClientError::Executor)?;
        Ok(Self::new(planner, executor))
    }

    /// Creates a client from an in-memory capabilities document and API key set.
    pub fn from_capabilities(
        capabilities: CapabilitiesDocument,
        api_keys: Vec<String>,
        config: ExecutorConfig,
    ) -> Result<Self, ClientError> {
        let planner = RequestPlanner::from_capabilities(capabilities);
        let executor = RequestExecutor::with_default_transport(api_keys, config)
            .map_err(ClientError::Executor)?;
        Ok(Self::new(planner, executor))
    }
}

fn build_plan_request(
    resource: &str,
    selections: Vec<String>,
    options: DataRequestOptions,
) -> PlanRequest {
    let mut request = PlanRequest::new(resource, selections);
    if let Some(id) = options.id {
        request = request.with_id(id);
    }
    for (name, value) in options.filters {
        request = request.with_filter(name, value);
    }
    for (name, value) in options.path_args {
        request = request.with_path_arg(name, value);
    }
    for selection in options.legacy_selections {
        request = request.with_legacy_selection(selection);
    }
    request
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use std::sync::Mutex;
    use std::time::Duration;

    use super::*;
    use crate::executor::ExecutorConfig;
    use crate::planner::ApiVersion;
    use crate::rate_limit::RateLimitConfig;
    use crate::transport::{TransportError, TransportRequest, TransportResponse};

    #[derive(Debug)]
    struct MockTransport {
        responses: Mutex<VecDeque<Result<TransportResponse, TransportError>>>,
        requests: Mutex<Vec<TransportRequest>>,
    }

    impl MockTransport {
        fn with_responses(responses: Vec<Result<TransportResponse, TransportError>>) -> Self {
            Self {
                responses: Mutex::new(responses.into()),
                requests: Mutex::new(Vec::new()),
            }
        }
    }

    impl HttpTransport for MockTransport {
        async fn execute(
            &self,
            request: &TransportRequest,
        ) -> Result<TransportResponse, TransportError> {
            self.requests
                .lock()
                .expect("requests lock should not be poisoned")
                .push(request.clone());
            self.responses
                .lock()
                .expect("responses lock should not be poisoned")
                .pop_front()
                .expect("missing mocked response")
        }
    }

    fn fixture_capabilities() -> CapabilitiesDocument {
        let json = r#"
{
  "spec": { "version": "5.4.1" },
  "resources": {
    "user": {
      "generic_path": "/user",
      "generic_parameters": [
        { "name": "selections" },
        { "name": "id" },
        { "name": "striptags" }
      ],
      "selections": [
        { "name": "profile", "can_use_generic_endpoint": true, "endpoints": ["/user/profile"] },
        { "name": "discord", "can_use_generic_endpoint": true, "endpoints": ["/user/discord"] },
        { "name": "hof", "can_use_generic_endpoint": true, "endpoints": ["/user/hof"] },
        { "name": "bazaar", "can_use_generic_endpoint": true, "fallback_to_v1": true, "endpoints": ["/user/bazaar"] }
      ]
    }
  }
}
"#;
        CapabilitiesDocument::from_json_str(json).expect("fixture should parse")
    }

    fn test_config() -> ExecutorConfig {
        ExecutorConfig {
            base_url_v2: "https://api.torn.com/v2".to_string(),
            base_url_v1: "https://api.torn.com".to_string(),
            timeout: Duration::from_secs(1),
            user_agent: "test-client".to_string(),
            max_attempts: 1,
            network_retry_backoff: Duration::from_millis(1),
            rate_limits: RateLimitConfig {
                per_key_per_minute: 1000,
                per_ip_per_minute: 1000,
            },
            max_in_flight: 4,
        }
    }

    #[tokio::test]
    async fn get_user_data_batches_selections_for_v2() {
        let planner = RequestPlanner::from_capabilities(fixture_capabilities());
        let transport = MockTransport::with_responses(vec![Ok(TransportResponse {
            status: 200,
            body: r#"{"profile":{"id":1},"discord":{"discordID":"x"},"hof":{"position":2}}"#
                .to_string(),
        })]);
        let executor =
            RequestExecutor::new(transport, vec!["k1".to_string()], test_config()).expect("init");
        let client = TornClient::new(planner, executor);

        let report = client
            .get_user_data(
                vec!["profile", "discord", "hof"],
                DataRequestOptions::default()
                    .with_id("3637232")
                    .with_filter("striptags", "true"),
            )
            .await
            .expect("request should succeed");

        assert_eq!(report.calls.len(), 1);
        assert_eq!(report.calls[0].api_version, ApiVersion::V2);
        assert_eq!(report.calls[0].path, "/user");
        assert_eq!(
            report.calls[0].query.get("selections"),
            Some(&"profile,discord,hof".to_string())
        );
        assert_eq!(report.merged_json["profile"]["id"], 1);
    }

    #[tokio::test]
    async fn get_resource_data_falls_back_to_v1_for_catalog_selection() {
        let planner = RequestPlanner::from_capabilities(fixture_capabilities());
        let transport = MockTransport::with_responses(vec![Ok(TransportResponse {
            status: 200,
            body: r#"{"bazaar":{"listings":[]}}"#.to_string(),
        })]);
        let executor =
            RequestExecutor::new(transport, vec!["k1".to_string()], test_config()).expect("init");
        let client = TornClient::new(planner, executor);

        let report = client
            .get_resource_data(
                "user",
                vec!["bazaar"],
                DataRequestOptions::default().with_id("3637232"),
            )
            .await
            .expect("request should succeed");

        assert_eq!(report.calls.len(), 1);
        assert_eq!(report.calls[0].api_version, ApiVersion::V1);
        assert_eq!(report.calls[0].path, "/user/3637232");
        assert_eq!(
            report.calls[0].query.get("selections"),
            Some(&"bazaar".to_string())
        );
    }
}
