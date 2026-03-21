use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
/// Planner error types for capabilities loading and request planning.
pub enum PlannerError {
    /// Failed to read the capabilities JSON file from disk.
    #[error("failed to read capabilities file '{path}': {source}")]
    CapabilitiesRead {
        /// Filesystem path that was attempted.
        path: PathBuf,
        /// I/O error returned by the filesystem.
        #[source]
        source: std::io::Error,
    },
    /// Failed to parse capabilities JSON after reading it.
    #[error("failed to parse capabilities file '{path}': {source}")]
    CapabilitiesParse {
        /// Filesystem path that was parsed.
        path: PathBuf,
        /// JSON parse error.
        #[source]
        source: serde_json::Error,
    },
    /// Requested resource is missing from capabilities.
    #[error("resource '{resource}' is not available in capabilities")]
    UnknownResource {
        /// Resource name that was requested.
        resource: String,
    },
    /// Requested selection is not supported for the resource.
    #[error("selection '{selection}' is not available for resource '{resource}'")]
    UnknownSelection {
        /// Resource name that was requested.
        resource: String,
        /// Selection name that was requested.
        selection: String,
    },
    /// Resource does not provide a generic endpoint route.
    #[error("resource '{resource}' does not expose a generic endpoint")]
    MissingGenericEndpoint {
        /// Resource name that lacks a generic endpoint.
        resource: String,
    },
    /// Selection does not map to a direct endpoint route.
    #[error("selection '{selection}' has no direct endpoint mapping in resource '{resource}'")]
    MissingDirectEndpoint {
        /// Resource name that was requested.
        resource: String,
        /// Selection name that lacks a direct endpoint.
        selection: String,
    },
    /// Required path argument for a direct endpoint was not provided.
    #[error(
        "selection '{selection}' requires path parameter '{parameter}' for resource '{resource}'"
    )]
    MissingPathParameter {
        /// Resource being requested.
        resource: String,
        /// Selection being requested.
        selection: String,
        /// Missing path parameter name.
        parameter: String,
    },
}
