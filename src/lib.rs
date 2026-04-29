//! Torn Rust SDK with planner-driven routing, resilient execution, and typed wrappers.
//!
//! This crate offers two integration layers:
//! - [`TornClient`] for direct resource/selection execution with explicit options.
//! - [`TornSdk`] for ergonomic per-resource wrapper APIs with typed and raw methods.
//!
//! Runtime behavior is driven by generated capabilities metadata (`spec/capabilities.json`),
//! enabling v2 batching, direct endpoint routing, and controlled v1 fallback behavior.
#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links, rustdoc::bare_urls)]

/// Capabilities metadata model and loader for planner inputs.
pub mod capabilities;
/// High-level client API used by wrappers and direct SDK integrations.
pub mod client;
/// Environment variable loading for runtime client/executor configuration.
pub mod env_config;
/// Planner-specific error types.
pub mod error;
/// Request execution engine with retry, fallback, and rate-limit handling.
pub mod executor;
/// Typed and raw response models.
pub mod models;
/// Request planning logic for v2/v1 route selection and fallback plans.
pub mod planner;
/// In-memory per-key and per-IP rate limiting primitives.
pub mod rate_limit;
/// HTTP transport abstraction and default reqwest implementation.
pub mod transport;
/// Static v1 resource/selection catalog used for fallback routing.
pub mod v1_catalog;
/// Ergonomic typed/raw resource wrappers built on top of [`TornClient`].
pub mod wrapper;

/// Generated capabilities document root and related metadata models.
pub use capabilities::{CapabilitiesDocument, ResourceCapabilities, SelectionCapability};
/// High-level client and request option types.
pub use client::{ClientError, DataRequestOptions, TornClient};
/// Environment config loading types.
pub use env_config::{EnvConfigError, RuntimeEnvConfig};
/// Planner error type.
pub use error::PlannerError;
/// Execution configuration, results, and executor types.
pub use executor::{
    ExecutedCall, ExecutionOptions, ExecutionReport, ExecutorConfig, ExecutorError, RequestExecutor,
};
/// Planning request/plan types and routing enums.
pub use planner::{
    ApiVersion, HttpMethod, PlanRequest, PlannedRequest, RequestPlan, RequestPlanner,
    RequestStrategy, SplitRequest,
};
/// Rate-limit configuration and acquisition result types.
pub use rate_limit::{AcquireResult, RateLimitConfig, RateLimiter};
/// Transport abstraction and concrete reqwest transport type.
pub use transport::{
    HttpTransport, ReqwestTransport, TransportError, TransportMethod, TransportRequest,
    TransportResponse,
};
/// Static v1 catalog types.
pub use v1_catalog::{V1Catalog, V1ResourceSpec};
/// Wrapper entrypoint, options, and per-resource APIs.
pub use wrapper::{
    BaseOptions, CompanyApi, CompanyOptions, FactionApi, FactionOptions, ForumApi, ForumOptions,
    KeyApi, KeyOptions, MarketApi, MarketOptions, PropertyApi, PropertyOptions, RacingApi,
    RacingOptions, SdkError, SortOrder, TornApi, TornOptions, TornSdk, UserApi, UserOptions,
};
