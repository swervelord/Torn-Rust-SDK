//! Environment-based runtime configuration loading.

use std::time::Duration;

use thiserror::Error;

use crate::executor::ExecutorConfig;

/// Environment variable containing comma-separated API keys.
pub const ENV_API_KEYS: &str = "TORN_API_KEYS";
/// Environment variable containing a single API key fallback.
pub const ENV_API_KEY: &str = "TORN_API_KEY";
/// Environment variable overriding v2 API base URL.
pub const ENV_BASE_URL_V2: &str = "TORN_API_BASE_URL_V2";
/// Environment variable overriding v1 API base URL.
pub const ENV_BASE_URL_V1: &str = "TORN_API_BASE_URL_V1";
/// Environment variable overriding HTTP timeout in seconds.
pub const ENV_HTTP_TIMEOUT_SECS: &str = "TORN_HTTP_TIMEOUT_SECS";
/// Environment variable overriding HTTP user agent.
pub const ENV_USER_AGENT: &str = "TORN_USER_AGENT";
/// Environment variable overriding max request attempts.
pub const ENV_MAX_ATTEMPTS: &str = "TORN_MAX_ATTEMPTS";
/// Environment variable overriding network retry backoff in milliseconds.
pub const ENV_NETWORK_RETRY_BACKOFF_MS: &str = "TORN_NETWORK_RETRY_BACKOFF_MS";
/// Environment variable overriding per-key requests/minute.
pub const ENV_RATE_LIMIT_PER_KEY: &str = "TORN_RATE_LIMIT_PER_KEY_PER_MIN";
/// Environment variable overriding per-IP requests/minute.
pub const ENV_RATE_LIMIT_PER_IP: &str = "TORN_RATE_LIMIT_PER_IP_PER_MIN";
/// Environment variable overriding max concurrent in-flight requests.
pub const ENV_MAX_IN_FLIGHT: &str = "TORN_MAX_IN_FLIGHT";

#[derive(Debug, Clone)]
/// Fully parsed runtime configuration used to initialize a client.
pub struct RuntimeEnvConfig {
    /// Parsed API keys.
    pub api_keys: Vec<String>,
    /// Executor runtime configuration built from env defaults and overrides.
    pub executor_config: ExecutorConfig,
}

impl RuntimeEnvConfig {
    /// Loads runtime config from process environment and optional `.env` file.
    pub fn from_env() -> Result<Self, EnvConfigError> {
        let _ = dotenvy::dotenv();
        load_runtime_from_lookup(|name| std::env::var(name).ok())
    }
}

#[derive(Debug, Error)]
/// Errors produced while loading environment configuration.
pub enum EnvConfigError {
    /// No API keys were found in either the multi-key or single-key environment variable.
    #[error(
        "no Torn API keys found in environment; set '{api_keys_var}' (comma-separated) or '{api_key_var}'"
    )]
    MissingApiKeys {
        /// Name of the comma-separated API key environment variable.
        api_keys_var: &'static str,
        /// Name of the single API key fallback environment variable.
        api_key_var: &'static str,
    },
    /// An environment variable contained an invalid value.
    #[error("invalid value for env var '{var_name}': '{value}' ({reason})")]
    InvalidEnvValue {
        /// Name of the invalid environment variable.
        var_name: &'static str,
        /// Raw value read from the environment.
        value: String,
        /// Parse/validation error reason.
        reason: String,
    },
}

fn load_runtime_from_lookup<F>(lookup: F) -> Result<RuntimeEnvConfig, EnvConfigError>
where
    F: Fn(&str) -> Option<String>,
{
    let api_keys = read_api_keys(&lookup)?;
    let mut config = ExecutorConfig::default();

    if let Some(value) = lookup(ENV_BASE_URL_V2).filter(|v| !v.trim().is_empty()) {
        config.base_url_v2 = value.trim().to_string();
    }
    if let Some(value) = lookup(ENV_BASE_URL_V1).filter(|v| !v.trim().is_empty()) {
        config.base_url_v1 = value.trim().to_string();
    }
    if let Some(value) = lookup(ENV_USER_AGENT).filter(|v| !v.trim().is_empty()) {
        config.user_agent = value.trim().to_string();
    }

    if let Some(timeout_secs) = parse_u64_env(&lookup, ENV_HTTP_TIMEOUT_SECS)? {
        config.timeout = Duration::from_secs(timeout_secs);
    }
    if let Some(max_attempts) = parse_u32_env(&lookup, ENV_MAX_ATTEMPTS)? {
        config.max_attempts = max_attempts;
    }
    if let Some(backoff_ms) = parse_u64_env(&lookup, ENV_NETWORK_RETRY_BACKOFF_MS)? {
        config.network_retry_backoff = Duration::from_millis(backoff_ms);
    }
    if let Some(per_key) = parse_u32_env(&lookup, ENV_RATE_LIMIT_PER_KEY)? {
        config.rate_limits.per_key_per_minute = per_key;
    }
    if let Some(per_ip) = parse_u32_env(&lookup, ENV_RATE_LIMIT_PER_IP)? {
        config.rate_limits.per_ip_per_minute = per_ip;
    }
    if let Some(max_in_flight) = parse_u32_env(&lookup, ENV_MAX_IN_FLIGHT)? {
        config.max_in_flight = max_in_flight as usize;
    }

    Ok(RuntimeEnvConfig {
        api_keys,
        executor_config: config,
    })
}

fn read_api_keys<F>(lookup: &F) -> Result<Vec<String>, EnvConfigError>
where
    F: Fn(&str) -> Option<String>,
{
    if let Some(keys_value) = lookup(ENV_API_KEYS).filter(|v| !v.trim().is_empty()) {
        let keys = keys_value
            .split(',')
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>();
        if !keys.is_empty() {
            return Ok(keys);
        }
    }

    if let Some(single_key) = lookup(ENV_API_KEY).filter(|v| !v.trim().is_empty()) {
        return Ok(vec![single_key.trim().to_string()]);
    }

    Err(EnvConfigError::MissingApiKeys {
        api_keys_var: ENV_API_KEYS,
        api_key_var: ENV_API_KEY,
    })
}

fn parse_u32_env<F>(lookup: &F, var_name: &'static str) -> Result<Option<u32>, EnvConfigError>
where
    F: Fn(&str) -> Option<String>,
{
    let Some(value) = lookup(var_name).filter(|v| !v.trim().is_empty()) else {
        return Ok(None);
    };
    let trimmed = value.trim();
    trimmed
        .parse::<u32>()
        .map(Some)
        .map_err(|_| EnvConfigError::InvalidEnvValue {
            var_name,
            value: trimmed.to_string(),
            reason: "expected unsigned integer".to_string(),
        })
}

fn parse_u64_env<F>(lookup: &F, var_name: &'static str) -> Result<Option<u64>, EnvConfigError>
where
    F: Fn(&str) -> Option<String>,
{
    let Some(value) = lookup(var_name).filter(|v| !v.trim().is_empty()) else {
        return Ok(None);
    };
    let trimmed = value.trim();
    trimmed
        .parse::<u64>()
        .map(Some)
        .map_err(|_| EnvConfigError::InvalidEnvValue {
            var_name,
            value: trimmed.to_string(),
            reason: "expected unsigned integer".to_string(),
        })
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;
    use crate::rate_limit::RateLimitConfig;

    fn load_from_map(
        values: BTreeMap<&'static str, &'static str>,
    ) -> Result<RuntimeEnvConfig, EnvConfigError> {
        load_runtime_from_lookup(|name| values.get(name).map(|value| value.to_string()))
    }

    #[test]
    fn parses_env_with_multiple_keys_and_custom_limits() {
        let mut values = BTreeMap::new();
        values.insert(ENV_API_KEYS, "k1,k2 , k3");
        values.insert(ENV_BASE_URL_V2, "https://api.torn.com/v2");
        values.insert(ENV_BASE_URL_V1, "https://api.torn.com");
        values.insert(ENV_HTTP_TIMEOUT_SECS, "45");
        values.insert(ENV_USER_AGENT, "sdk-test/1.0");
        values.insert(ENV_MAX_ATTEMPTS, "5");
        values.insert(ENV_NETWORK_RETRY_BACKOFF_MS, "300");
        values.insert(ENV_RATE_LIMIT_PER_KEY, "90");
        values.insert(ENV_RATE_LIMIT_PER_IP, "900");
        values.insert(ENV_MAX_IN_FLIGHT, "12");

        let runtime = load_from_map(values).expect("env parse should succeed");
        assert_eq!(runtime.api_keys, vec!["k1", "k2", "k3"]);
        assert_eq!(runtime.executor_config.timeout, Duration::from_secs(45));
        assert_eq!(runtime.executor_config.user_agent, "sdk-test/1.0");
        assert_eq!(runtime.executor_config.max_attempts, 5);
        assert_eq!(
            runtime.executor_config.network_retry_backoff,
            Duration::from_millis(300)
        );
        assert_eq!(
            runtime.executor_config.rate_limits,
            RateLimitConfig {
                per_key_per_minute: 90,
                per_ip_per_minute: 900
            }
        );
        assert_eq!(runtime.executor_config.max_in_flight, 12);
    }

    #[test]
    fn falls_back_to_single_api_key_var() {
        let mut values = BTreeMap::new();
        values.insert(ENV_API_KEY, "single-key");

        let runtime = load_from_map(values).expect("env parse should succeed");
        assert_eq!(runtime.api_keys, vec!["single-key"]);
    }

    #[test]
    fn errors_when_no_keys_are_present() {
        let err = load_from_map(BTreeMap::new()).expect_err("should fail");
        assert!(matches!(err, EnvConfigError::MissingApiKeys { .. }));
    }

    #[test]
    fn errors_on_invalid_numeric_values() {
        let mut values = BTreeMap::new();
        values.insert(ENV_API_KEY, "single-key");
        values.insert(ENV_MAX_ATTEMPTS, "abc");
        let err = load_from_map(values).expect_err("should fail");
        assert!(matches!(err, EnvConfigError::InvalidEnvValue { .. }));
    }
}
