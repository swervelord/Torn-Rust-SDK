//! Transport abstraction used by the request executor.

use std::collections::BTreeMap;
use std::fmt;
use std::time::Duration;

use reqwest::Client;
use reqwest::Url;
use thiserror::Error;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
/// Supported HTTP methods for transport requests.
pub enum TransportMethod {
    /// HTTP GET.
    Get,
}

#[derive(Clone)]
/// Transport-layer request produced by the executor.
pub struct TransportRequest {
    /// Base URL (for example `https://api.torn.com/v2`).
    pub base_url: String,
    /// Request path without base URL.
    pub path: String,
    /// HTTP method.
    pub method: TransportMethod,
    /// Query parameters to encode into the URL.
    pub query: BTreeMap<String, String>,
    /// API key used for this request (redacted in `Debug` output).
    pub api_key: String,
    /// Optional per-request timeout override.
    pub timeout: Option<Duration>,
}

impl fmt::Debug for TransportRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TransportRequest")
            .field("base_url", &self.base_url)
            .field("path", &self.path)
            .field("method", &self.method)
            .field("query", &self.query)
            .field("api_key", &redacted_secret(&self.api_key))
            .field("timeout", &self.timeout)
            .finish()
    }
}

#[derive(Debug, Clone)]
/// Transport-layer response used by the executor.
pub struct TransportResponse {
    /// HTTP status code.
    pub status: u16,
    /// Response body as UTF-8 text.
    pub body: String,
}

#[derive(Debug, Error)]
/// Errors emitted by transport implementations.
pub enum TransportError {
    /// URL construction from base URL, path, and query failed.
    #[error("invalid URL built from base '{base_url}' and path '{path}': {message}")]
    InvalidUrl {
        /// Request base URL provided by executor configuration.
        base_url: String,
        /// Relative request path provided by planner/executor.
        path: String,
        /// Underlying URL parser message.
        message: String,
    },
    /// HTTP client request/response execution failed.
    #[error("HTTP transport error: {message}")]
    Http {
        /// Sanitized transport error message.
        message: String,
    },
}

/// Async HTTP transport abstraction for executor calls.
pub trait HttpTransport: Send + Sync {
    /// Executes a single transport request.
    fn execute(
        &self,
        request: &TransportRequest,
    ) -> impl std::future::Future<Output = Result<TransportResponse, TransportError>> + Send;
}

#[derive(Debug)]
/// Default transport implementation backed by `reqwest`.
pub struct ReqwestTransport {
    client: Client,
}

impl ReqwestTransport {
    /// Creates a reqwest-backed transport with timeout and user agent.
    pub fn new(timeout: Duration, user_agent: impl Into<String>) -> Result<Self, TransportError> {
        let client = Client::builder()
            .timeout(timeout)
            .user_agent(user_agent.into())
            .build()
            .map_err(|error| TransportError::Http {
                message: sanitize_error_message(&error.to_string()),
            })?;
        Ok(Self { client })
    }
}

impl HttpTransport for ReqwestTransport {
    async fn execute(
        &self,
        request: &TransportRequest,
    ) -> Result<TransportResponse, TransportError> {
        let url = build_url(
            &request.base_url,
            &request.path,
            &request.query,
            &request.api_key,
        )?;
        let mut request_builder = match request.method {
            TransportMethod::Get => self.client.get(url),
        };
        if let Some(timeout) = request.timeout {
            request_builder = request_builder.timeout(timeout);
        }

        let response = request_builder
            .send()
            .await
            .map_err(|error| TransportError::Http {
                message: sanitize_error_message(&error.to_string()),
            })?;
        let status = response.status().as_u16();
        let body = response
            .text()
            .await
            .map_err(|error| TransportError::Http {
                message: sanitize_error_message(&error.to_string()),
            })?;
        Ok(TransportResponse { status, body })
    }
}

fn build_url(
    base_url: &str,
    path: &str,
    query: &BTreeMap<String, String>,
    api_key: &str,
) -> Result<Url, TransportError> {
    let normalized_base = base_url.trim_end_matches('/');
    let normalized_path = if path.starts_with('/') {
        path.to_string()
    } else {
        format!("/{path}")
    };
    let full = format!("{normalized_base}{normalized_path}");
    let mut url = Url::parse(&full).map_err(|source| TransportError::InvalidUrl {
        base_url: base_url.to_string(),
        path: path.to_string(),
        message: source.to_string(),
    })?;

    {
        let mut pairs = url.query_pairs_mut();
        for (name, value) in query {
            pairs.append_pair(name, value);
        }
        pairs.append_pair("key", api_key);
    }

    Ok(url)
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

fn sanitize_error_message(message: &str) -> String {
    let mut sanitized = String::with_capacity(message.len());
    let mut remaining = message;

    while let Some(start) = remaining.find("key=") {
        let (prefix, with_key) = remaining.split_at(start);
        sanitized.push_str(prefix);
        sanitized.push_str("key=");

        let value = &with_key[4..];
        let value_len = value
            .find(['&', '"', '\'', ' ', ')'])
            .unwrap_or(value.len());
        sanitized.push_str("***");
        remaining = &value[value_len..];
    }

    sanitized.push_str(remaining);
    sanitized
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn builds_url_with_query_and_key() {
        let mut query = BTreeMap::new();
        query.insert("selections".to_string(), "profile,discord".to_string());
        query.insert("id".to_string(), "123".to_string());

        let url = build_url("https://api.torn.com/v2", "/user", &query, "abc")
            .expect("url build should work");

        let url = url.to_string();
        assert!(url.starts_with("https://api.torn.com/v2/user?"));
        assert!(url.contains("selections=profile%2Cdiscord"));
        assert!(url.contains("id=123"));
        assert!(url.contains("key=abc"));
    }

    #[test]
    fn transport_request_debug_redacts_api_key() {
        let mut query = BTreeMap::new();
        query.insert("selections".to_string(), "profile".to_string());

        let request = TransportRequest {
            base_url: "https://api.torn.com/v2".to_string(),
            path: "/user".to_string(),
            method: TransportMethod::Get,
            query,
            api_key: "super-secret-key-12345".to_string(),
            timeout: Some(Duration::from_secs(5)),
        };

        let debug = format!("{request:?}");
        assert!(!debug.contains("super-secret-key-12345"));
        assert!(debug.contains("sup***45"));
    }

    #[test]
    fn sanitize_error_message_redacts_key_query_values() {
        let raw = "request failed for https://api.torn.com/v2/user?selections=profile&key=secret123&foo=bar";
        let sanitized = sanitize_error_message(raw);
        assert!(!sanitized.contains("secret123"));
        assert!(sanitized.contains("key=***"));
        assert!(sanitized.contains("foo=bar"));
    }
}
