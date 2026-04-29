//! Models for `spec/capabilities.json` used by request planning.

use std::{collections::BTreeMap, fs, path::Path};

use serde::Deserialize;

use crate::error::PlannerError;

#[derive(Debug, Clone, Deserialize)]
/// Parsed capabilities document generated from the Torn OpenAPI spec.
pub struct CapabilitiesDocument {
    /// Spec-level metadata for the generated capabilities artifact.
    #[serde(default)]
    pub spec: CapabilitySpec,
    /// Resource capability map keyed by normalized resource name (for example, `user`).
    #[serde(default)]
    pub resources: BTreeMap<String, ResourceCapabilities>,
}

impl CapabilitiesDocument {
    /// Parses a capabilities document from a JSON string.
    pub fn from_json_str(content: &str) -> Result<Self, serde_json::Error> {
        let content = content.trim_start_matches('\u{feff}');
        serde_json::from_str(content)
    }

    /// Reads and parses a capabilities document from disk.
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, PlannerError> {
        let path = path.as_ref();
        let content =
            fs::read_to_string(path).map_err(|source| PlannerError::CapabilitiesRead {
                path: path.to_path_buf(),
                source,
            })?;

        Self::from_json_str(&content).map_err(|source| PlannerError::CapabilitiesParse {
            path: path.to_path_buf(),
            source,
        })
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
/// Top-level metadata block describing the source spec version.
pub struct CapabilitySpec {
    /// Torn OpenAPI version string used to generate the capabilities file.
    #[serde(default)]
    pub version: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
/// Per-resource planner metadata used for batching and endpoint routing.
pub struct ResourceCapabilities {
    /// Generic endpoint path for selection batching, when supported.
    #[serde(default)]
    pub generic_path: Option<String>,
    /// Parameters accepted by the generic endpoint.
    #[serde(default)]
    pub generic_parameters: Vec<ParameterCapability>,
    /// Filter subset considered useful for generic planning.
    #[serde(default)]
    pub generic_filter_parameters: Vec<ParameterCapability>,
    /// Selection-level capability entries.
    #[serde(default)]
    pub selections: Vec<SelectionCapability>,
    /// Endpoint-level metadata for dedicated paths.
    #[serde(default)]
    pub endpoints: Vec<EndpointCapability>,
}

impl ResourceCapabilities {
    /// Finds a selection capability by its normalized name.
    pub fn selection(&self, name: &str) -> Option<&SelectionCapability> {
        self.selections.iter().find(|s| s.name == name)
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
/// Capability details for a query or path parameter.
pub struct ParameterCapability {
    /// Parameter name.
    #[serde(default)]
    pub name: String,
    /// Parameter location (`query`, `path`, ...).
    #[serde(default, rename = "in")]
    pub location: String,
    /// Whether the parameter is required.
    #[serde(default)]
    pub required: bool,
    /// Optional schema summary for the parameter.
    #[serde(default)]
    pub schema: Option<ParameterSchema>,
}

#[derive(Debug, Clone, Default, Deserialize)]
/// Reduced schema metadata for a parameter definition.
pub struct ParameterSchema {
    /// JSON schema type.
    #[serde(default, rename = "type")]
    pub kind: Option<String>,
    /// Optional format hint.
    #[serde(default)]
    pub format: Option<String>,
    /// Optional schema description.
    #[serde(default)]
    pub description: Option<String>,
    /// Optional default value.
    #[serde(default)]
    pub default: Option<serde_json::Value>,
    /// Optional minimum constraint.
    #[serde(default)]
    pub minimum: Option<serde_json::Value>,
    /// Optional maximum constraint.
    #[serde(default)]
    pub maximum: Option<serde_json::Value>,
    /// Optional enum values.
    #[serde(default)]
    pub r#enum: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
/// Capability metadata for a single selection within a resource.
pub struct SelectionCapability {
    /// Selection name.
    #[serde(default)]
    pub name: String,
    /// Whether the selection must be requested alone.
    #[serde(default)]
    pub standalone_only: bool,
    /// Whether this selection can be requested via the generic endpoint.
    #[serde(default)]
    pub can_use_generic_endpoint: bool,
    /// Whether this selection must use dedicated endpoints only.
    #[serde(default)]
    pub requires_direct_endpoint_only: bool,
    /// Whether planners should route this selection to v1.
    #[serde(default)]
    pub fallback_to_v1: bool,
    /// Whether selection is unavailable on v2 generic endpoints.
    #[serde(default)]
    pub unavailable_in_v2: bool,
    /// Dedicated endpoint paths for this selection.
    #[serde(default)]
    pub endpoints: Vec<String>,
    /// Allowed query parameters for the selection endpoints.
    #[serde(default)]
    pub query_parameters: Vec<String>,
    /// Required path parameters for direct endpoint use.
    #[serde(default)]
    pub required_path_parameters: Vec<String>,
    /// Response schema refs observed for this selection.
    #[serde(default)]
    pub response_schema_refs: Vec<String>,
    /// Response top-level fields observed for this selection.
    #[serde(default)]
    pub response_top_level_fields: Vec<String>,
    /// Flattened response field paths observed for this selection.
    #[serde(default)]
    pub response_field_paths: Vec<String>,
    /// Supplemental notes from overrides/generation heuristics.
    #[serde(default)]
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
/// Endpoint-level capability metadata collected from the OpenAPI spec.
pub struct EndpointCapability {
    /// Endpoint path.
    #[serde(default)]
    pub path: String,
    /// HTTP method.
    #[serde(default)]
    pub method: String,
    /// Optional operation identifier from OpenAPI.
    #[serde(default)]
    pub operation_id: Option<String>,
    /// Derived selection hint from endpoint path.
    #[serde(default)]
    pub selection_hint: Option<String>,
    /// Whether endpoint selection should be treated as standalone.
    #[serde(default)]
    pub standalone_only: bool,
    /// Endpoint parameters.
    #[serde(default)]
    pub parameters: Vec<ParameterCapability>,
    /// Referenced response schemas.
    #[serde(default)]
    pub response_schema_refs: Vec<String>,
    /// Top-level response fields.
    #[serde(default)]
    pub response_top_level_fields: Vec<String>,
    /// Flattened response field paths.
    #[serde(default)]
    pub response_field_paths: Vec<String>,
}
