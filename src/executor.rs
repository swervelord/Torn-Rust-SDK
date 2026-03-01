//! Request execution layer with retries, fallback handling, and rate limiting.

use std::collections::BTreeMap;
use std::fmt;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::Deserialize;
use serde::de::DeserializeOwned;
use serde_json::{Map, Value};
use thiserror::Error;
use tokio::sync::Semaphore;
use tokio::time::sleep;

use crate::error::PlannerError;
use crate::planner::{
    ApiVersion, HttpMethod, PlanRequest, PlannedRequest, RequestPlan, RequestPlanner,
    RequestStrategy, SplitRequest,
};
use crate::rate_limit::{AcquireResult, RateLimitConfig, RateLimiter};
use crate::transport::{
    HttpTransport, ReqwestTransport, TransportError, TransportMethod, TransportRequest,
    TransportResponse,
};
use crate::v1_catalog::V1Catalog;

#[derive(Debug, Clone)]
/// Runtime configuration for request execution behavior.
pub struct ExecutorConfig {
    /// Base URL for v2 requests.
    pub base_url_v2: String,
    /// Base URL for v1 fallback requests.
    pub base_url_v1: String,
    /// Default transport timeout.
    pub timeout: Duration,
    /// User-Agent value passed to the HTTP client.
    pub user_agent: String,
    /// Maximum number of attempts per request.
    pub max_attempts: u32,
    /// Base backoff for retriable network failures.
    pub network_retry_backoff: Duration,
    /// Per-key/per-IP rate-limit configuration.
    pub rate_limits: RateLimitConfig,
    /// Max concurrent in-flight requests.
    pub max_in_flight: usize,
}

impl Default for ExecutorConfig {
    fn default() -> Self {
        Self {
            base_url_v2: "https://api.torn.com/v2".to_string(),
            base_url_v1: "https://api.torn.com".to_string(),
            timeout: Duration::from_secs(30),
            user_agent: "torn-sdk-rust/0.1".to_string(),
            max_attempts: 3,
            network_retry_backoff: Duration::from_millis(250),
            rate_limits: RateLimitConfig::default(),
            max_in_flight: 8,
        }
    }
}

#[derive(Debug, Error)]
/// Execution-time error surface for planner-executor flows.
pub enum ExecutorError {
    /// No API keys were configured for request execution.
    #[error("no API keys configured")]
    NoApiKeys,
    /// Executor configuration failed validation.
    #[error("invalid executor configuration for '{field}': {reason}")]
    InvalidConfig {
        /// Name of the invalid configuration field.
        field: &'static str,
        /// Validation reason.
        reason: String,
    },
    /// Planner returned an error while building request plans.
    #[error("planner error: {0}")]
    Planner(#[from] PlannerError),
    /// HTTP transport returned an error.
    #[error("HTTP transport error: {0}")]
    Transport(#[from] TransportError),
    /// Response body was not valid JSON.
    #[error("failed to parse JSON response for path '{path}' (status {status}): {source}")]
    InvalidJson {
        /// Request path for the failed response.
        path: String,
        /// HTTP status code.
        status: u16,
        /// JSON parse error.
        #[source]
        source: serde_json::Error,
    },
    /// Torn API returned an application-level error envelope.
    #[error("Torn API error for path '{path}' (status {status}, code {code:?}): {message}")]
    Api {
        /// Request path for the failed response.
        path: String,
        /// HTTP status code.
        status: u16,
        /// Torn API error code, when provided.
        code: Option<i64>,
        /// Torn API error message.
        message: String,
    },
}

#[derive(Debug, Clone, Default)]
/// Optional per-call execution overrides.
pub struct ExecutionOptions {
    /// Maximum attempts for this call.
    pub max_attempts: Option<u32>,
    /// Per-request timeout override.
    pub request_timeout: Option<Duration>,
}

impl ExecutionOptions {
    /// Sets max attempts override.
    pub fn with_max_attempts(mut self, max_attempts: u32) -> Self {
        self.max_attempts = Some(max_attempts);
        self
    }

    /// Sets per-request timeout override.
    pub fn with_request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
}

#[derive(Clone)]
/// Single HTTP call record emitted by the executor.
pub struct ExecutedCall {
    /// API version used for this call.
    pub api_version: ApiVersion,
    /// Resource name.
    pub resource: String,
    /// Request path.
    pub path: String,
    /// HTTP method.
    pub method: HttpMethod,
    /// Query parameters used for the call.
    pub query: BTreeMap<String, String>,
    /// Selections represented by this call.
    pub selections: Vec<String>,
    /// Planning/execution strategy classification.
    pub strategy: RequestStrategy,
    /// API key used for the call (redacted in `Debug` output).
    pub key_used: String,
    /// HTTP status returned by the endpoint.
    pub status: u16,
    /// Whether the call was part of fallback behavior.
    pub fallback: bool,
}

impl fmt::Debug for ExecutedCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExecutedCall")
            .field("api_version", &self.api_version)
            .field("resource", &self.resource)
            .field("path", &self.path)
            .field("method", &self.method)
            .field("query", &self.query)
            .field("selections", &self.selections)
            .field("strategy", &self.strategy)
            .field("key_used", &redacted_secret(&self.key_used))
            .field("status", &self.status)
            .field("fallback", &self.fallback)
            .finish()
    }
}

#[derive(Debug, Clone)]
/// Merged execution result containing combined JSON and call traces.
pub struct ExecutionReport {
    /// Merged response payload across all calls in a plan.
    pub merged_json: Value,
    /// Ordered call trace records.
    pub calls: Vec<ExecutedCall>,
}

impl ExecutionReport {
    /// Deserializes the merged JSON payload into a typed model.
    pub fn deserialize<T: DeserializeOwned>(&self) -> Result<T, serde_json::Error> {
        serde_json::from_value(self.merged_json.clone())
    }
}

/// Executes planned requests through an [`HttpTransport`] implementation.
pub struct RequestExecutor<T: HttpTransport> {
    transport: T,
    config: ExecutorConfig,
    api_keys: Vec<String>,
    key_cursor: Mutex<usize>,
    rate_limiter: RateLimiter,
    in_flight: Arc<Semaphore>,
}

impl<T> fmt::Debug for RequestExecutor<T>
where
    T: HttpTransport + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RequestExecutor")
            .field("transport", &self.transport)
            .field("config", &self.config)
            .field("api_key_count", &self.api_keys.len())
            .field("key_cursor", &self.key_cursor)
            .field("rate_limiter", &self.rate_limiter)
            .field("in_flight", &self.in_flight)
            .finish()
    }
}

impl<T: HttpTransport> RequestExecutor<T> {
    /// Creates a request executor with the provided transport, keys, and config.
    pub fn new(
        transport: T,
        api_keys: Vec<String>,
        config: ExecutorConfig,
    ) -> Result<Self, ExecutorError> {
        if api_keys.is_empty() {
            return Err(ExecutorError::NoApiKeys);
        }
        if config.max_in_flight == 0 {
            return Err(ExecutorError::InvalidConfig {
                field: "max_in_flight",
                reason: "must be greater than zero".to_string(),
            });
        }

        Ok(Self {
            transport,
            rate_limiter: RateLimiter::new(config.rate_limits),
            key_cursor: Mutex::new(0),
            api_keys,
            in_flight: Arc::new(Semaphore::new(config.max_in_flight)),
            config,
        })
    }

    /// Plans and executes a request with default execution options.
    pub async fn plan_and_execute(
        &self,
        planner: &RequestPlanner,
        request: &PlanRequest,
    ) -> Result<ExecutionReport, ExecutorError> {
        self.plan_and_execute_with_options(planner, request, ExecutionOptions::default())
            .await
    }

    /// Plans and executes a request with explicit execution options.
    pub async fn plan_and_execute_with_options(
        &self,
        planner: &RequestPlanner,
        request: &PlanRequest,
        options: ExecutionOptions,
    ) -> Result<ExecutionReport, ExecutorError> {
        let plan = planner.plan(request)?;
        self.execute_plan_with_options(&plan, options).await
    }

    /// Executes a pre-built request plan with default execution options.
    pub async fn execute_plan(&self, plan: &RequestPlan) -> Result<ExecutionReport, ExecutorError> {
        self.execute_plan_with_options(plan, ExecutionOptions::default())
            .await
    }

    /// Executes a pre-built request plan with explicit execution options.
    pub async fn execute_plan_with_options(
        &self,
        plan: &RequestPlan,
        options: ExecutionOptions,
    ) -> Result<ExecutionReport, ExecutorError> {
        let mut merged = Value::Object(Map::new());
        let mut calls = Vec::new();

        for request in &plan.requests {
            let target = RequestExecutionSpec::from_planned(request);
            let outcome = self.execute_request(&target, &options, false).await;

            match outcome {
                Ok(single) => {
                    merge_json(&mut merged, single.response_json);
                    calls.push(single.executed_call);
                }
                Err(error) => {
                    if let Some(v1_fallback_request) =
                        build_runtime_v1_fallback_request(request, &error)
                    {
                        let target = RequestExecutionSpec::from_planned(&v1_fallback_request);
                        let single = self.execute_request(&target, &options, true).await?;
                        merge_json(&mut merged, single.response_json);
                        calls.push(single.executed_call);
                        continue;
                    }

                    if request.fallback_split.is_empty() || !is_selection_combination_error(&error)
                    {
                        return Err(error);
                    }

                    for split in &request.fallback_split {
                        let single = self.execute_split_request(split, &options).await?;
                        merge_json(&mut merged, single.response_json);
                        calls.push(single.executed_call);
                    }
                }
            }
        }

        Ok(ExecutionReport {
            merged_json: merged,
            calls,
        })
    }

    async fn execute_split_request(
        &self,
        split: &SplitRequest,
        options: &ExecutionOptions,
    ) -> Result<SingleExecutionResult, ExecutorError> {
        let target = RequestExecutionSpec::from_split(split);
        self.execute_request(&target, options, true).await
    }

    async fn execute_request(
        &self,
        target: &RequestExecutionSpec<'_>,
        options: &ExecutionOptions,
        fallback: bool,
    ) -> Result<SingleExecutionResult, ExecutorError> {
        let max_attempts = options
            .max_attempts
            .unwrap_or(self.config.max_attempts)
            .max(1);
        for attempt in 0..max_attempts {
            let key = self.acquire_key_slot().await;
            let base_url = self.base_url_for(target.api_version);
            let _permit = self
                .in_flight
                .acquire()
                .await
                .expect("in-flight semaphore should not be closed");

            let transport_request = TransportRequest {
                base_url: base_url.to_string(),
                path: target.path.to_string(),
                method: map_method(target.method),
                query: target.query.clone(),
                api_key: key.clone(),
                timeout: options.request_timeout,
            };

            let transport_response = match self.transport.execute(&transport_request).await {
                Ok(response) => response,
                Err(error) => {
                    if attempt + 1 < max_attempts {
                        let multiplier = attempt.saturating_add(1) as u64;
                        sleep(
                            self.config
                                .network_retry_backoff
                                .saturating_mul(multiplier as u32),
                        )
                        .await;
                        continue;
                    }
                    return Err(ExecutorError::Transport(error));
                }
            };

            let response_json = parse_response_json(target.path, &transport_response)?;
            if let Some(api_error) =
                parse_api_error(target.path, &transport_response, &response_json)
            {
                if is_rate_limit_error(&api_error) {
                    let now = current_unix_seconds();
                    self.rate_limiter
                        .mark_key_exhausted_at_unix_seconds(&key, now);
                    if is_ip_rate_limit_error(&api_error) {
                        self.rate_limiter.mark_ip_exhausted_at_unix_seconds(now);
                    }

                    if attempt + 1 < max_attempts {
                        sleep(RateLimiter::wait_duration_until_next_minute(now)).await;
                        continue;
                    }
                }

                return Err(api_error);
            }

            let executed_call = ExecutedCall {
                api_version: target.api_version,
                resource: target.resource.to_string(),
                path: target.path.to_string(),
                method: target.method,
                query: target.query.clone(),
                selections: target.selections.to_vec(),
                strategy: target.strategy,
                key_used: key,
                status: transport_response.status,
                fallback,
            };

            return Ok(SingleExecutionResult {
                response_json,
                executed_call,
            });
        }

        Err(ExecutorError::Api {
            path: target.path.to_string(),
            status: 0,
            code: None,
            message: "request attempts exhausted".to_string(),
        })
    }

    fn base_url_for(&self, api_version: ApiVersion) -> &str {
        match api_version {
            ApiVersion::V2 => &self.config.base_url_v2,
            ApiVersion::V1 => &self.config.base_url_v1,
        }
    }

    async fn acquire_key_slot(&self) -> String {
        loop {
            let now = current_unix_seconds();
            {
                let mut cursor = self
                    .key_cursor
                    .lock()
                    .expect("key cursor mutex should not be poisoned");
                let start = *cursor;
                let key_count = self.api_keys.len();

                for offset in 0..key_count {
                    let index = (start + offset) % key_count;
                    let key = &self.api_keys[index];
                    if self.rate_limiter.try_acquire_at_unix_seconds(key, now)
                        == AcquireResult::Acquired
                    {
                        *cursor = (index + 1) % key_count;
                        return key.clone();
                    }
                }
            }

            sleep(RateLimiter::wait_duration_until_next_minute(now)).await;
        }
    }
}

impl RequestExecutor<ReqwestTransport> {
    /// Builds a [`RequestExecutor`] using the default reqwest transport.
    pub fn with_default_transport(
        api_keys: Vec<String>,
        config: ExecutorConfig,
    ) -> Result<Self, ExecutorError> {
        let transport = ReqwestTransport::new(config.timeout, config.user_agent.clone())?;
        Self::new(transport, api_keys, config)
    }
}

#[derive(Debug, Clone, Copy)]
struct RequestExecutionSpec<'a> {
    api_version: ApiVersion,
    resource: &'a str,
    path: &'a str,
    method: HttpMethod,
    query: &'a BTreeMap<String, String>,
    selections: &'a [String],
    strategy: RequestStrategy,
}

impl<'a> RequestExecutionSpec<'a> {
    fn from_planned(request: &'a PlannedRequest) -> Self {
        Self {
            api_version: request.api_version,
            resource: &request.resource,
            path: &request.path,
            method: request.method,
            query: &request.query,
            selections: &request.selections,
            strategy: request.strategy,
        }
    }

    fn from_split(split: &'a SplitRequest) -> Self {
        Self {
            api_version: split.api_version,
            resource: &split.resource,
            path: &split.path,
            method: split.method,
            query: &split.query,
            selections: &split.selections,
            strategy: split.strategy,
        }
    }
}

#[derive(Debug)]
struct SingleExecutionResult {
    response_json: Value,
    executed_call: ExecutedCall,
}

#[derive(Debug, Deserialize)]
struct TornErrorEnvelope {
    error: TornErrorBody,
}

#[derive(Debug, Deserialize)]
struct TornErrorBody {
    #[serde(default)]
    code: Option<i64>,
    #[serde(default)]
    error: Option<String>,
}

fn parse_response_json(path: &str, response: &TransportResponse) -> Result<Value, ExecutorError> {
    serde_json::from_str(&response.body).map_err(|source| ExecutorError::InvalidJson {
        path: path.to_string(),
        status: response.status,
        source,
    })
}

fn parse_api_error(
    path: &str,
    response: &TransportResponse,
    json: &Value,
) -> Option<ExecutorError> {
    if response.status >= 400 {
        if let Some((code, message)) = extract_torn_error(json) {
            return Some(ExecutorError::Api {
                path: path.to_string(),
                status: response.status,
                code,
                message,
            });
        }
        return Some(ExecutorError::Api {
            path: path.to_string(),
            status: response.status,
            code: None,
            message: "HTTP error response".to_string(),
        });
    }

    // Torn has historically returned some errors as 200 with a top-level `{ "error": ... }`.
    // Only treat it as an API error when the payload is purely an error envelope.
    if is_top_level_error_envelope(json)
        && let Some((code, message)) = extract_torn_error(json)
    {
        return Some(ExecutorError::Api {
            path: path.to_string(),
            status: response.status,
            code,
            message,
        });
    }

    None
}

fn extract_torn_error(json: &Value) -> Option<(Option<i64>, String)> {
    let envelope = serde_json::from_value::<TornErrorEnvelope>(json.clone()).ok()?;
    let message = envelope.error.error?.trim().to_string();
    if message.is_empty() {
        return None;
    }

    Some((envelope.error.code, message))
}

fn is_top_level_error_envelope(json: &Value) -> bool {
    matches!(json, Value::Object(map) if map.len() == 1 && map.contains_key("error"))
}

fn is_rate_limit_error(error: &ExecutorError) -> bool {
    let ExecutorError::Api {
        status, message, ..
    } = error
    else {
        return false;
    };

    if *status == 429 {
        return true;
    }

    let message = message.to_ascii_lowercase();
    message.contains("rate limit")
        || message.contains("too many request")
        || message.contains("too many calls")
        || message.contains("request limit")
}

fn is_ip_rate_limit_error(error: &ExecutorError) -> bool {
    let ExecutorError::Api { message, .. } = error else {
        return false;
    };
    let message = message.to_ascii_lowercase();
    message.contains("ip") && is_rate_limit_error(error)
}

fn is_selection_combination_error(error: &ExecutorError) -> bool {
    let ExecutorError::Api { message, .. } = error else {
        return false;
    };
    let message = message.to_ascii_lowercase();
    (message.contains("selection") && message.contains("cannot"))
        || message.contains("cannot be used together")
        || message.contains("standalone")
}

fn is_v2_migration_error(error: &ExecutorError) -> bool {
    let ExecutorError::Api { message, .. } = error else {
        return false;
    };
    let message = message.to_ascii_lowercase();
    message.contains("not available in api v2")
        || message.contains("fallback to api v1")
        || message.contains("fall back to api v1")
        || (message.contains("api v1") && message.contains("selection"))
}

fn build_runtime_v1_fallback_request(
    request: &PlannedRequest,
    error: &ExecutorError,
) -> Option<PlannedRequest> {
    if request.api_version != ApiVersion::V2 || !is_v2_migration_error(error) {
        return None;
    }

    let catalog = V1Catalog;
    catalog.resource(&request.resource)?;

    let id = request.query.get("id").cloned();
    let path = catalog.build_generic_path(&request.resource, id.as_deref())?;
    let mut query = BTreeMap::new();
    for (name, value) in &request.query {
        if name == "selections" || is_allowed_v1_filter(name) {
            query.insert(name.clone(), value.clone());
        }
    }

    Some(PlannedRequest {
        resource: request.resource.clone(),
        path,
        method: request.method,
        query,
        selections: request.selections.clone(),
        strategy: RequestStrategy::VersionFallback,
        fallback_split: Vec::new(),
        api_version: ApiVersion::V1,
    })
}

fn merge_json(target: &mut Value, incoming: Value) {
    match (target, incoming) {
        (Value::Object(target_object), Value::Object(incoming_object)) => {
            for (key, value) in incoming_object {
                match target_object.get_mut(&key) {
                    Some(existing) => merge_json(existing, value),
                    None => {
                        target_object.insert(key, value);
                    }
                }
            }
        }
        (target_slot, incoming_value) => {
            *target_slot = incoming_value;
        }
    }
}

fn map_method(method: HttpMethod) -> TransportMethod {
    match method {
        HttpMethod::Get => TransportMethod::Get,
    }
}

fn current_unix_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after UNIX_EPOCH")
        .as_secs()
}

const V1_ALLOWED_FILTERS: &[&str] = &[
    "cat",
    "filters",
    "from",
    "limit",
    "offset",
    "sort",
    "stat",
    "striptags",
    "timestamp",
    "to",
];

fn is_allowed_v1_filter(name: &str) -> bool {
    V1_ALLOWED_FILTERS.contains(&name)
}

fn redacted_secret(secret: &str) -> String {
    if secret.is_empty() {
        return "<empty>".to_string();
    }

    if secret.len() <= 6 {
        return "***".to_string();
    }

    let prefix = &secret[..3];
    let suffix = &secret[secret.len() - 2..];
    format!("{prefix}***{suffix}")
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use std::sync::Mutex;

    use super::*;
    use crate::planner::{PlannedRequest, RequestPlan, SplitRequest};
    use crate::transport::{HttpTransport, TransportError};

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

        fn requests(&self) -> Vec<TransportRequest> {
            self.requests
                .lock()
                .expect("requests lock should not be poisoned")
                .clone()
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
                .expect("response queue should have an entry")
        }
    }

    fn test_config() -> ExecutorConfig {
        ExecutorConfig {
            base_url_v2: "https://api.torn.com/v2".to_string(),
            base_url_v1: "https://api.torn.com".to_string(),
            timeout: Duration::from_secs(1),
            user_agent: "test-agent".to_string(),
            max_attempts: 1,
            network_retry_backoff: Duration::from_millis(1),
            rate_limits: RateLimitConfig {
                per_key_per_minute: 1000,
                per_ip_per_minute: 10000,
            },
            max_in_flight: 4,
        }
    }

    #[tokio::test]
    async fn executes_combined_request_without_fallback() {
        let transport = MockTransport::with_responses(vec![Ok(TransportResponse {
            status: 200,
            body: r#"{"profile":{"id":1},"discord":{"discordID":"x"},"hof":{"position":2}}"#
                .to_string(),
        })]);
        let executor =
            RequestExecutor::new(transport, vec!["k1".to_string()], test_config()).expect("init");

        let mut query = BTreeMap::new();
        query.insert("selections".to_string(), "profile,discord,hof".to_string());
        query.insert("id".to_string(), "123".to_string());

        let plan = RequestPlan {
            resource: "user".to_string(),
            requests: vec![PlannedRequest {
                resource: "user".to_string(),
                path: "/user".to_string(),
                method: HttpMethod::Get,
                query,
                selections: vec![
                    "profile".to_string(),
                    "discord".to_string(),
                    "hof".to_string(),
                ],
                strategy: RequestStrategy::GenericCombined,
                fallback_split: Vec::new(),
                api_version: ApiVersion::V2,
            }],
        };

        let report = executor
            .execute_plan(&plan)
            .await
            .expect("execution should work");
        assert_eq!(report.calls.len(), 1);
        assert_eq!(report.calls[0].path, "/user");
        assert_eq!(report.calls[0].status, 200);
        assert_eq!(report.merged_json["profile"]["id"], 1);
        assert_eq!(report.merged_json["discord"]["discordID"], "x");
        assert_eq!(report.merged_json["hof"]["position"], 2);
    }

    #[tokio::test]
    async fn falls_back_to_split_requests_on_selection_combination_error() {
        let transport = MockTransport::with_responses(vec![
            Ok(TransportResponse {
                status: 400,
                body: r#"{"error":{"code":999,"error":"Selection cannot be used together with another selection"}}"#
                    .to_string(),
            }),
            Ok(TransportResponse {
                status: 200,
                body: r#"{"profile":{"id":1}}"#.to_string(),
            }),
            Ok(TransportResponse {
                status: 200,
                body: r#"{"discord":{"discordID":"abc"}}"#.to_string(),
            }),
        ]);
        let executor =
            RequestExecutor::new(transport, vec!["k1".to_string()], test_config()).expect("init");

        let mut combined_query = BTreeMap::new();
        combined_query.insert("selections".to_string(), "profile,discord".to_string());

        let mut split_profile_query = BTreeMap::new();
        split_profile_query.insert("selections".to_string(), "profile".to_string());
        let mut split_discord_query = BTreeMap::new();
        split_discord_query.insert("selections".to_string(), "discord".to_string());

        let plan = RequestPlan {
            resource: "user".to_string(),
            requests: vec![PlannedRequest {
                resource: "user".to_string(),
                path: "/user".to_string(),
                method: HttpMethod::Get,
                query: combined_query,
                selections: vec!["profile".to_string(), "discord".to_string()],
                strategy: RequestStrategy::GenericCombined,
                fallback_split: vec![
                    SplitRequest {
                        resource: "user".to_string(),
                        path: "/user".to_string(),
                        method: HttpMethod::Get,
                        query: split_profile_query,
                        selections: vec!["profile".to_string()],
                        strategy: RequestStrategy::GenericSingle,
                        api_version: ApiVersion::V2,
                    },
                    SplitRequest {
                        resource: "user".to_string(),
                        path: "/user".to_string(),
                        method: HttpMethod::Get,
                        query: split_discord_query,
                        selections: vec!["discord".to_string()],
                        strategy: RequestStrategy::GenericSingle,
                        api_version: ApiVersion::V2,
                    },
                ],
                api_version: ApiVersion::V2,
            }],
        };

        let report = executor
            .execute_plan(&plan)
            .await
            .expect("execution should work");
        assert_eq!(report.calls.len(), 2);
        assert!(report.calls.iter().all(|call| call.fallback));
        assert_eq!(report.merged_json["profile"]["id"], 1);
        assert_eq!(report.merged_json["discord"]["discordID"], "abc");
    }

    #[tokio::test]
    async fn rotate_keys_when_multiple_are_available() {
        let transport = MockTransport::with_responses(vec![
            Ok(TransportResponse {
                status: 200,
                body: r#"{"profile":{"id":1}}"#.to_string(),
            }),
            Ok(TransportResponse {
                status: 200,
                body: r#"{"discord":{"discordID":"x"}}"#.to_string(),
            }),
        ]);
        let executor = RequestExecutor::new(
            transport,
            vec!["key-1".to_string(), "key-2".to_string()],
            test_config(),
        )
        .expect("init");

        let mut q1 = BTreeMap::new();
        q1.insert("selections".to_string(), "profile".to_string());
        let mut q2 = BTreeMap::new();
        q2.insert("selections".to_string(), "discord".to_string());

        let plan = RequestPlan {
            resource: "user".to_string(),
            requests: vec![
                PlannedRequest {
                    resource: "user".to_string(),
                    path: "/user".to_string(),
                    method: HttpMethod::Get,
                    query: q1,
                    selections: vec!["profile".to_string()],
                    strategy: RequestStrategy::GenericSingle,
                    fallback_split: vec![],
                    api_version: ApiVersion::V2,
                },
                PlannedRequest {
                    resource: "user".to_string(),
                    path: "/user".to_string(),
                    method: HttpMethod::Get,
                    query: q2,
                    selections: vec!["discord".to_string()],
                    strategy: RequestStrategy::GenericSingle,
                    fallback_split: vec![],
                    api_version: ApiVersion::V2,
                },
            ],
        };

        let report = executor
            .execute_plan(&plan)
            .await
            .expect("execution should work");
        assert_eq!(report.calls.len(), 2);
        assert_eq!(report.calls[0].key_used, "key-1");
        assert_eq!(report.calls[1].key_used, "key-2");
    }

    #[tokio::test]
    async fn ignores_noncanonical_error_fields_in_success_payloads() {
        let transport = MockTransport::with_responses(vec![Ok(TransportResponse {
            status: 200,
            body: r#"{"profile":{"id":1},"error":{"hint":"non-fatal metadata"}}"#.to_string(),
        })]);
        let executor = RequestExecutor::new(
            transport,
            vec!["key-1".to_string()],
            ExecutorConfig {
                max_attempts: 1,
                ..test_config()
            },
        )
        .expect("init");

        let mut query = BTreeMap::new();
        query.insert("selections".to_string(), "profile".to_string());

        let plan = RequestPlan {
            resource: "user".to_string(),
            requests: vec![PlannedRequest {
                resource: "user".to_string(),
                path: "/user".to_string(),
                method: HttpMethod::Get,
                query,
                selections: vec!["profile".to_string()],
                strategy: RequestStrategy::GenericSingle,
                fallback_split: vec![],
                api_version: ApiVersion::V2,
            }],
        };

        let report = executor
            .execute_plan(&plan)
            .await
            .expect("execution should work");
        assert_eq!(report.calls.len(), 1);
        assert_eq!(report.merged_json["profile"]["id"], 1);
    }

    #[tokio::test]
    async fn treats_top_level_error_envelope_as_error_on_200_status() {
        let transport = MockTransport::with_responses(vec![Ok(TransportResponse {
            status: 200,
            body: r#"{"error":{"code":101,"error":"Key is invalid"}}"#.to_string(),
        })]);
        let executor = RequestExecutor::new(
            transport,
            vec!["key-1".to_string()],
            ExecutorConfig {
                max_attempts: 1,
                ..test_config()
            },
        )
        .expect("init");

        let mut query = BTreeMap::new();
        query.insert("selections".to_string(), "profile".to_string());

        let plan = RequestPlan {
            resource: "user".to_string(),
            requests: vec![PlannedRequest {
                resource: "user".to_string(),
                path: "/user".to_string(),
                method: HttpMethod::Get,
                query,
                selections: vec!["profile".to_string()],
                strategy: RequestStrategy::GenericSingle,
                fallback_split: vec![],
                api_version: ApiVersion::V2,
            }],
        };

        let err = executor
            .execute_plan(&plan)
            .await
            .expect_err("execution should fail");
        match err {
            ExecutorError::Api {
                status,
                code,
                message,
                ..
            } => {
                assert_eq!(status, 200);
                assert_eq!(code, Some(101));
                assert_eq!(message, "Key is invalid");
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[tokio::test]
    async fn falls_back_to_v1_on_v2_migration_error() {
        let transport = MockTransport::with_responses(vec![
            Ok(TransportResponse {
                status: 400,
                body: r#"{"error":{"code":123,"error":"The following selections are not available in API v2: 'networth'"}}"#
                    .to_string(),
            }),
            Ok(TransportResponse {
                status: 200,
                body: r#"{"networth":{"total":999}}"#.to_string(),
            }),
        ]);
        let executor = RequestExecutor::new(
            transport,
            vec!["key-1".to_string()],
            ExecutorConfig {
                max_attempts: 1,
                ..test_config()
            },
        )
        .expect("init");

        let mut query = BTreeMap::new();
        query.insert("selections".to_string(), "networth".to_string());
        query.insert("id".to_string(), "3637232".to_string());
        query.insert("limit".to_string(), "10".to_string());
        query.insert("foo".to_string(), "bar".to_string());

        let plan = RequestPlan {
            resource: "user".to_string(),
            requests: vec![PlannedRequest {
                resource: "user".to_string(),
                path: "/user".to_string(),
                method: HttpMethod::Get,
                query,
                selections: vec!["networth".to_string()],
                strategy: RequestStrategy::GenericSingle,
                fallback_split: vec![],
                api_version: ApiVersion::V2,
            }],
        };

        let report = executor
            .execute_plan(&plan)
            .await
            .expect("execution should work");
        assert_eq!(report.calls.len(), 1);
        assert_eq!(report.calls[0].api_version, ApiVersion::V1);
        assert_eq!(report.calls[0].path, "/user/3637232");
        assert!(report.calls[0].fallback);
        assert_eq!(report.merged_json["networth"]["total"], 999);

        let requests = executor.transport.requests();
        assert_eq!(requests.len(), 2);
        assert_eq!(requests[0].base_url, "https://api.torn.com/v2");
        assert_eq!(requests[1].base_url, "https://api.torn.com");
        assert_eq!(requests[1].path, "/user/3637232");
        assert_eq!(
            requests[1].query.get("selections"),
            Some(&"networth".to_string())
        );
        assert_eq!(requests[1].query.get("limit"), Some(&"10".to_string()));
        assert!(!requests[1].query.contains_key("id"));
        assert!(!requests[1].query.contains_key("foo"));
    }

    #[test]
    fn executed_call_debug_redacts_key_used() {
        let call = ExecutedCall {
            api_version: ApiVersion::V2,
            resource: "user".to_string(),
            path: "/user".to_string(),
            method: HttpMethod::Get,
            query: BTreeMap::new(),
            selections: vec!["profile".to_string()],
            strategy: RequestStrategy::GenericSingle,
            key_used: "raw-secret-api-key".to_string(),
            status: 200,
            fallback: false,
        };

        let debug = format!("{call:?}");
        assert!(!debug.contains("raw-secret-api-key"));
        assert!(debug.contains("raw***ey"));
    }
}
