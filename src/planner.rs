//! Request planning for Torn selections across v2/v1 endpoints.

use std::collections::{BTreeMap, HashSet};
use std::path::Path;

use crate::capabilities::{CapabilitiesDocument, ResourceCapabilities, SelectionCapability};
use crate::error::PlannerError;
use crate::v1_catalog::V1Catalog;

#[derive(Debug, Clone)]
/// Plans resource/selection requests into concrete HTTP calls.
pub struct RequestPlanner {
    capabilities: CapabilitiesDocument,
    v1_catalog: V1Catalog,
}

impl RequestPlanner {
    /// Builds a planner from generated capabilities metadata.
    pub fn from_capabilities(capabilities: CapabilitiesDocument) -> Self {
        Self {
            capabilities,
            v1_catalog: V1Catalog,
        }
    }

    /// Loads capabilities from disk and creates a planner.
    pub fn from_capabilities_file(path: impl AsRef<Path>) -> Result<Self, PlannerError> {
        let capabilities = CapabilitiesDocument::from_path(path)?;
        Ok(Self::from_capabilities(capabilities))
    }

    /// Plans a high-level resource request into one or more executable requests.
    pub fn plan(&self, request: &PlanRequest) -> Result<RequestPlan, PlannerError> {
        let resource_name = normalize_resource_name(&request.resource);
        let v2_resource = self.capabilities.resources.get(&resource_name);
        let v1_resource = self.v1_catalog.resource(&resource_name);
        if v2_resource.is_none() && v1_resource.is_none() {
            return Err(PlannerError::UnknownResource {
                resource: resource_name,
            });
        }

        let requested_selections = normalize_and_dedup_selections(&request.selections);
        if requested_selections.is_empty() {
            return Ok(RequestPlan {
                resource: resource_name,
                requests: Vec::new(),
            });
        }

        let mut combinable_generic = Vec::new();
        let mut standalone = Vec::new();
        let mut direct_only = Vec::new();
        let mut v1_generic = Vec::new();

        for selection_name in requested_selections {
            let v2_selection = v2_resource.and_then(|resource| resource.selection(&selection_name));
            if let Some(selection) = v2_selection {
                if (selection.unavailable_in_v2 || selection.fallback_to_v1)
                    && self
                        .v1_catalog
                        .has_selection(&resource_name, &selection_name)
                {
                    v1_generic.push(selection_name.clone());
                    continue;
                }

                if selection.requires_direct_endpoint_only || !selection.can_use_generic_endpoint {
                    direct_only.push(selection);
                    continue;
                }

                if selection.standalone_only {
                    standalone.push(selection);
                    continue;
                }

                combinable_generic.push(selection);
                continue;
            }

            if self
                .v1_catalog
                .has_selection(&resource_name, &selection_name)
            {
                v1_generic.push(selection_name);
                continue;
            }

            return Err(PlannerError::UnknownSelection {
                resource: resource_name.clone(),
                selection: selection_name,
            });
        }

        let mut planned_requests = Vec::new();

        if !combinable_generic.is_empty() {
            let resource = v2_resource.ok_or_else(|| PlannerError::MissingGenericEndpoint {
                resource: resource_name.clone(),
            })?;
            let selections = combinable_generic
                .iter()
                .map(|selection| selection.name.clone())
                .collect::<Vec<_>>();

            let mut combined = self.build_generic_request(
                &resource_name,
                resource,
                request,
                selections,
                RequestStrategy::GenericCombined,
                ApiVersion::V2,
            )?;

            if combined.selections.len() > 1 {
                combined.fallback_split = combined
                    .selections
                    .iter()
                    .map(|selection| {
                        self.build_generic_split_request(
                            &resource_name,
                            resource,
                            request,
                            selection.clone(),
                            ApiVersion::V2,
                        )
                    })
                    .collect::<Result<Vec<_>, _>>()?;
            }

            planned_requests.push(combined);
        }

        for selection in standalone {
            let resource = v2_resource.ok_or_else(|| PlannerError::MissingGenericEndpoint {
                resource: resource_name.clone(),
            })?;
            let request =
                if selection.can_use_generic_endpoint && !selection.requires_direct_endpoint_only {
                    self.build_generic_request(
                        &resource_name,
                        resource,
                        request,
                        vec![selection.name.clone()],
                        RequestStrategy::GenericSingle,
                        ApiVersion::V2,
                    )?
                } else {
                    self.build_direct_request(&resource_name, resource, request, selection)?
                };

            planned_requests.push(request);
        }

        for selection in direct_only {
            let resource = v2_resource.ok_or_else(|| PlannerError::MissingGenericEndpoint {
                resource: resource_name.clone(),
            })?;
            let direct = self.build_direct_request(&resource_name, resource, request, selection)?;
            planned_requests.push(direct);
        }

        if !v1_generic.is_empty() {
            let strategy = if v1_generic.len() > 1 {
                RequestStrategy::GenericCombined
            } else {
                RequestStrategy::GenericSingle
            };
            planned_requests.push(self.build_v1_generic_request(
                &resource_name,
                request,
                v1_generic,
                strategy,
            )?);
        }

        Ok(RequestPlan {
            resource: resource_name,
            requests: planned_requests,
        })
    }

    fn build_generic_request(
        &self,
        resource_name: &str,
        resource: &ResourceCapabilities,
        request: &PlanRequest,
        selections: Vec<String>,
        strategy: RequestStrategy,
        api_version: ApiVersion,
    ) -> Result<PlannedRequest, PlannerError> {
        let path =
            resource
                .generic_path
                .clone()
                .ok_or_else(|| PlannerError::MissingGenericEndpoint {
                    resource: resource_name.to_string(),
                })?;

        let mut query = BTreeMap::new();
        let allowed = resource
            .generic_parameters
            .iter()
            .map(|p| p.name.as_str())
            .collect::<HashSet<_>>();

        for (name, value) in &request.filters {
            if allowed.contains(name.as_str()) {
                query.insert(name.clone(), value.clone());
            }
        }

        if !request.legacy_selections.is_empty() && allowed.contains("legacy") {
            let legacy = normalize_and_dedup_selections(&request.legacy_selections);
            if !legacy.is_empty() {
                query.insert("legacy".to_string(), legacy.join(","));
            }
        }

        let request_id = request.id.as_ref().filter(|_| allowed.contains("id"));
        if let Some(id) = request_id {
            query.insert("id".to_string(), id.clone());
        }

        query.insert("selections".to_string(), selections.join(","));

        Ok(PlannedRequest {
            resource: resource_name.to_string(),
            path,
            method: HttpMethod::Get,
            query,
            selections,
            strategy,
            fallback_split: Vec::new(),
            api_version,
        })
    }

    fn build_generic_split_request(
        &self,
        resource_name: &str,
        resource: &ResourceCapabilities,
        request: &PlanRequest,
        selection: String,
        api_version: ApiVersion,
    ) -> Result<SplitRequest, PlannerError> {
        let planned = self.build_generic_request(
            resource_name,
            resource,
            request,
            vec![selection],
            RequestStrategy::GenericSingle,
            api_version,
        )?;

        Ok(SplitRequest {
            resource: planned.resource,
            path: planned.path,
            method: planned.method,
            query: planned.query,
            selections: planned.selections,
            strategy: planned.strategy,
            api_version: planned.api_version,
        })
    }

    fn build_direct_request(
        &self,
        resource_name: &str,
        _resource: &ResourceCapabilities,
        request: &PlanRequest,
        selection: &SelectionCapability,
    ) -> Result<PlannedRequest, PlannerError> {
        let endpoint = choose_direct_endpoint(selection, request, resource_name)?;
        let path = fill_path_template(&endpoint, selection, request, resource_name)?;

        let allowed = selection
            .query_parameters
            .iter()
            .map(String::as_str)
            .collect::<HashSet<_>>();
        let mut query = BTreeMap::new();
        for (name, value) in &request.filters {
            if allowed.contains(name.as_str()) {
                query.insert(name.clone(), value.clone());
            }
        }

        Ok(PlannedRequest {
            resource: resource_name.to_string(),
            path,
            method: HttpMethod::Get,
            query,
            selections: vec![selection.name.clone()],
            strategy: RequestStrategy::DirectEndpoint,
            fallback_split: Vec::new(),
            api_version: ApiVersion::V2,
        })
    }

    fn build_v1_generic_request(
        &self,
        resource_name: &str,
        request: &PlanRequest,
        selections: Vec<String>,
        strategy: RequestStrategy,
    ) -> Result<PlannedRequest, PlannerError> {
        let path = self
            .v1_catalog
            .build_generic_path(resource_name, request.id.as_deref())
            .ok_or_else(|| PlannerError::UnknownResource {
                resource: resource_name.to_string(),
            })?;

        let mut query = BTreeMap::new();
        for (name, value) in &request.filters {
            if is_allowed_v1_filter(name) {
                query.insert(name.clone(), value.clone());
            }
        }
        query.insert("selections".to_string(), selections.join(","));

        Ok(PlannedRequest {
            resource: resource_name.to_string(),
            path,
            method: HttpMethod::Get,
            query,
            selections,
            strategy,
            fallback_split: Vec::new(),
            api_version: ApiVersion::V1,
        })
    }
}

#[derive(Debug, Clone)]
/// User-provided planning request for a resource and selection set.
pub struct PlanRequest {
    /// Normalized resource name.
    pub resource: String,
    /// Requested selection names.
    pub selections: Vec<String>,
    /// Optional generic/direct identifier.
    pub id: Option<String>,
    /// Query filters that may be applied to planned requests.
    pub filters: BTreeMap<String, String>,
    /// Explicit path placeholder values for direct endpoints.
    pub path_args: BTreeMap<String, String>,
    /// Optional legacy selections for endpoints supporting `legacy`.
    pub legacy_selections: Vec<String>,
}

impl PlanRequest {
    /// Creates a normalized planning request.
    pub fn new(resource: impl Into<String>, selections: Vec<impl Into<String>>) -> Self {
        Self {
            resource: normalize_resource_name(&resource.into()),
            selections: selections
                .into_iter()
                .map(|selection| normalize_selection_name(&selection.into()))
                .collect(),
            id: None,
            filters: BTreeMap::new(),
            path_args: BTreeMap::new(),
            legacy_selections: Vec::new(),
        }
    }

    /// Sets the request `id`.
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Adds a query filter.
    pub fn with_filter(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.filters
            .insert(normalize_filter_name(&name.into()), value.into());
        self
    }

    /// Adds a named path argument for direct endpoint templates.
    pub fn with_path_arg(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.path_args.insert(name.into(), value.into());
        self
    }

    /// Adds a legacy selection to forward when supported.
    pub fn with_legacy_selection(mut self, selection: impl Into<String>) -> Self {
        self.legacy_selections
            .push(normalize_selection_name(&selection.into()));
        self
    }
}

#[derive(Debug, Clone)]
/// Planned result containing executable requests.
pub struct RequestPlan {
    /// Normalized resource name.
    pub resource: String,
    /// Ordered planned requests.
    pub requests: Vec<PlannedRequest>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
/// Supported HTTP methods for planned requests.
pub enum HttpMethod {
    /// HTTP GET.
    Get,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
/// API version target selected by the planner/executor.
pub enum ApiVersion {
    /// Torn API v1.
    V1,
    /// Torn API v2.
    V2,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
/// Strategy used to produce a planned request.
pub enum RequestStrategy {
    /// Multiple selections batched on a generic endpoint.
    GenericCombined,
    /// Single selection routed through generic endpoint.
    GenericSingle,
    /// Selection routed through dedicated direct endpoint.
    DirectEndpoint,
    /// Selection routed to v1 via fallback behavior.
    VersionFallback,
}

#[derive(Debug, Clone)]
/// Concrete request emitted by the planner for executor dispatch.
pub struct PlannedRequest {
    /// Resource name.
    pub resource: String,
    /// Request path (without base URL).
    pub path: String,
    /// HTTP method.
    pub method: HttpMethod,
    /// Query parameters for this request.
    pub query: BTreeMap<String, String>,
    /// Selections represented by this request.
    pub selections: Vec<String>,
    /// Planning strategy used for this request.
    pub strategy: RequestStrategy,
    /// Split fallback requests for recoverable combination failures.
    pub fallback_split: Vec<SplitRequest>,
    /// API version target.
    pub api_version: ApiVersion,
}

#[derive(Debug, Clone)]
/// Precomputed split request used when a combined request must be retried individually.
pub struct SplitRequest {
    /// Resource name.
    pub resource: String,
    /// Request path (without base URL).
    pub path: String,
    /// HTTP method.
    pub method: HttpMethod,
    /// Query parameters for this request.
    pub query: BTreeMap<String, String>,
    /// Single selection represented by this split request.
    pub selections: Vec<String>,
    /// Planning strategy used for this split request.
    pub strategy: RequestStrategy,
    /// API version target.
    pub api_version: ApiVersion,
}

fn choose_direct_endpoint(
    selection: &SelectionCapability,
    request: &PlanRequest,
    resource_name: &str,
) -> Result<String, PlannerError> {
    if selection.endpoints.is_empty() {
        return Err(PlannerError::MissingDirectEndpoint {
            resource: resource_name.to_string(),
            selection: selection.name.clone(),
        });
    }

    let mut fillable_with_id = Vec::new();
    let mut fillable_without_id = Vec::new();
    let mut first_missing_param: Option<String> = None;

    for endpoint in &selection.endpoints {
        let placeholders = extract_path_placeholders(endpoint);
        let mut missing = Vec::new();
        for placeholder in &placeholders {
            if resolve_path_placeholder_value(placeholder, request).is_none() {
                missing.push(placeholder.clone());
            }
        }

        if !missing.is_empty() {
            if first_missing_param.is_none() {
                first_missing_param = missing.into_iter().next();
            }
            continue;
        }

        if placeholders.iter().any(|name| name == "id") {
            fillable_with_id.push(endpoint.clone());
        } else {
            fillable_without_id.push(endpoint.clone());
        }
    }

    if let Some(path) = request.id.as_ref().and_then(|_| fillable_with_id.first()) {
        return Ok(path.clone());
    }

    if let Some(path) = fillable_without_id.first() {
        return Ok(path.clone());
    }

    if let Some(path) = fillable_with_id.first() {
        return Ok(path.clone());
    }

    if let Some(parameter) = first_missing_param {
        return Err(PlannerError::MissingPathParameter {
            resource: resource_name.to_string(),
            selection: selection.name.clone(),
            parameter,
        });
    }

    Err(PlannerError::MissingDirectEndpoint {
        resource: resource_name.to_string(),
        selection: selection.name.clone(),
    })
}

fn fill_path_template(
    path_template: &str,
    selection: &SelectionCapability,
    request: &PlanRequest,
    resource_name: &str,
) -> Result<String, PlannerError> {
    let placeholders = extract_path_placeholders(path_template);
    let mut path = path_template.to_string();
    for placeholder in placeholders {
        let value = resolve_path_placeholder_value(&placeholder, request).ok_or_else(|| {
            PlannerError::MissingPathParameter {
                resource: resource_name.to_string(),
                selection: selection.name.clone(),
                parameter: placeholder.clone(),
            }
        })?;
        let token = format!("{{{placeholder}}}");
        path = path.replace(&token, &value);
    }

    Ok(path)
}

fn resolve_path_placeholder_value(name: &str, request: &PlanRequest) -> Option<String> {
    if name == "id" {
        return request
            .id
            .as_ref()
            .cloned()
            .or_else(|| request.path_args.get(name).cloned());
    }

    request.path_args.get(name).cloned()
}

fn extract_path_placeholders(path: &str) -> Vec<String> {
    let mut placeholders = Vec::new();
    let mut in_placeholder = false;
    let mut current = String::new();

    for c in path.chars() {
        if c == '{' {
            in_placeholder = true;
            current.clear();
            continue;
        }

        if c == '}' {
            if in_placeholder && !current.is_empty() {
                placeholders.push(current.clone());
            }
            in_placeholder = false;
            current.clear();
            continue;
        }

        if in_placeholder {
            current.push(c);
        }
    }

    placeholders
}

fn normalize_resource_name(name: &str) -> String {
    name.trim().to_ascii_lowercase()
}

fn normalize_selection_name(name: &str) -> String {
    name.trim().to_ascii_lowercase()
}

fn normalize_filter_name(name: &str) -> String {
    name.trim().to_ascii_lowercase()
}

fn normalize_and_dedup_selections(selections: &[String]) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    for selection in selections {
        let normalized = normalize_selection_name(selection);
        if normalized.is_empty() {
            continue;
        }
        if seen.insert(normalized.clone()) {
            out.push(normalized);
        }
    }
    out
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capabilities::CapabilitiesDocument;

    fn planner_from_fixture() -> RequestPlanner {
        let fixture = r#"
{
  "spec": { "version": "5.4.1" },
  "resources": {
    "user": {
      "generic_path": "/user",
      "generic_parameters": [
        { "name": "selections" },
        { "name": "id" },
        { "name": "limit" },
        { "name": "sort" },
        { "name": "striptags" },
        { "name": "timestamp" }
      ],
      "generic_filter_parameters": [
        { "name": "limit" },
        { "name": "sort" },
        { "name": "striptags" },
        { "name": "timestamp" }
      ],
      "selections": [
        {
          "name": "profile",
          "can_use_generic_endpoint": true,
          "requires_direct_endpoint_only": false,
          "standalone_only": false,
          "query_parameters": ["striptags", "timestamp"],
          "endpoints": ["/user/profile", "/user/{id}/profile"]
        },
        {
          "name": "discord",
          "can_use_generic_endpoint": true,
          "requires_direct_endpoint_only": false,
          "standalone_only": false,
          "query_parameters": ["timestamp"],
          "endpoints": ["/user/discord", "/user/{id}/discord"]
        },
        {
          "name": "hof",
          "can_use_generic_endpoint": true,
          "requires_direct_endpoint_only": false,
          "standalone_only": false,
          "query_parameters": ["timestamp"],
          "endpoints": ["/user/hof", "/user/{id}/hof"]
        },
        {
          "name": "bazaar",
          "can_use_generic_endpoint": true,
          "requires_direct_endpoint_only": false,
          "standalone_only": false,
          "fallback_to_v1": true,
          "query_parameters": [],
          "endpoints": ["/user/bazaar"]
        },
        {
          "name": "virus",
          "can_use_generic_endpoint": false,
          "requires_direct_endpoint_only": true,
          "standalone_only": false,
          "query_parameters": ["timestamp"],
          "endpoints": ["/user/virus"]
        },
        {
          "name": "crime",
          "can_use_generic_endpoint": false,
          "requires_direct_endpoint_only": true,
          "standalone_only": false,
          "query_parameters": [],
          "required_path_parameters": ["crimeId"],
          "endpoints": ["/user/{crimeId}/crimes"]
        }
      ]
    },
    "faction": {
      "generic_path": "/faction",
      "generic_parameters": [
        { "name": "selections" },
        { "name": "name" }
      ],
      "selections": [
        {
          "name": "search",
          "can_use_generic_endpoint": true,
          "requires_direct_endpoint_only": false,
          "standalone_only": true,
          "query_parameters": ["name"],
          "endpoints": ["/faction/search"]
        },
        {
          "name": "members",
          "can_use_generic_endpoint": true,
          "requires_direct_endpoint_only": false,
          "standalone_only": false,
          "query_parameters": [],
          "endpoints": ["/faction/members"]
        }
      ]
    }
  }
}
"#;
        let caps = CapabilitiesDocument::from_json_str(fixture).expect("fixture should parse");
        RequestPlanner::from_capabilities(caps)
    }

    #[test]
    fn merges_generic_selections_into_single_request() {
        let planner = planner_from_fixture();
        let request = PlanRequest::new("user", vec!["profile", "discord", "hof"])
            .with_id("3637232")
            .with_filter("limit", "100")
            .with_filter("foo", "bar")
            .with_filter("striptags", "true");

        let plan = planner.plan(&request).expect("planning should work");
        assert_eq!(plan.requests.len(), 1);

        let combined = &plan.requests[0];
        assert_eq!(combined.api_version, ApiVersion::V2);
        assert_eq!(combined.path, "/user");
        assert_eq!(
            combined.query.get("selections"),
            Some(&"profile,discord,hof".to_string())
        );
        assert_eq!(combined.query.get("id"), Some(&"3637232".to_string()));
        assert_eq!(combined.query.get("limit"), Some(&"100".to_string()));
        assert_eq!(combined.query.get("striptags"), Some(&"true".to_string()));
        assert!(!combined.query.contains_key("foo"));
        assert_eq!(combined.fallback_split.len(), 3);
    }

    #[test]
    fn splits_when_direct_only_selection_is_present() {
        let planner = planner_from_fixture();
        let request =
            PlanRequest::new("user", vec!["profile", "virus"]).with_filter("timestamp", "1");

        let plan = planner.plan(&request).expect("planning should work");
        assert_eq!(plan.requests.len(), 2);

        assert_eq!(plan.requests[0].path, "/user");
        assert_eq!(plan.requests[0].selections, vec!["profile".to_string()]);
        assert_eq!(plan.requests[0].api_version, ApiVersion::V2);
        assert_eq!(plan.requests[1].path, "/user/virus");
        assert_eq!(plan.requests[1].selections, vec!["virus".to_string()]);
        assert_eq!(plan.requests[1].api_version, ApiVersion::V2);
        assert_eq!(
            plan.requests[1].query.get("timestamp"),
            Some(&"1".to_string())
        );
    }

    #[test]
    fn keeps_standalone_selection_outside_combined_group() {
        let planner = planner_from_fixture();
        let request =
            PlanRequest::new("faction", vec!["search", "members"]).with_filter("name", "zeta");

        let plan = planner.plan(&request).expect("planning should work");
        assert_eq!(plan.requests.len(), 2);

        let selections = plan
            .requests
            .iter()
            .map(|req| req.query.get("selections").cloned().unwrap_or_default())
            .collect::<Vec<_>>();
        assert!(selections.contains(&"search".to_string()));
        assert!(selections.contains(&"members".to_string()));
        assert!(!selections.contains(&"search,members".to_string()));
    }

    #[test]
    fn errors_when_direct_endpoint_requires_missing_path_param() {
        let planner = planner_from_fixture();
        let request = PlanRequest::new("user", vec!["crime"]);

        let err = planner.plan(&request).expect_err("planning should fail");
        match err {
            PlannerError::MissingPathParameter {
                resource,
                selection,
                parameter,
            } => {
                assert_eq!(resource, "user");
                assert_eq!(selection, "crime");
                assert_eq!(parameter, "crimeId");
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn uses_path_arg_for_non_id_template_parameter() {
        let planner = planner_from_fixture();
        let request = PlanRequest::new("user", vec!["crime"]).with_path_arg("crimeId", "42");

        let plan = planner.plan(&request).expect("planning should work");
        assert_eq!(plan.requests.len(), 1);
        assert_eq!(plan.requests[0].path, "/user/42/crimes");
    }

    #[test]
    fn can_plan_against_real_generated_capabilities() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("spec")
            .join("capabilities.json");
        let planner = RequestPlanner::from_capabilities_file(&path).expect("should load file");

        let request = PlanRequest::new("user", vec!["profile", "discord", "hof"])
            .with_id("3637232")
            .with_filter("striptags", "true");
        let plan = planner.plan(&request).expect("plan should succeed");

        assert!(!plan.requests.is_empty());
        let first = &plan.requests[0];
        assert_eq!(first.api_version, ApiVersion::V2);
        assert_eq!(first.path, "/user");
        assert_eq!(
            first.query.get("selections"),
            Some(&"profile,discord,hof".to_string())
        );
    }

    #[test]
    fn routes_v2_fallback_selection_to_v1() {
        let planner = planner_from_fixture();
        let request = PlanRequest::new("user", vec!["profile", "bazaar"]).with_id("3637232");
        let plan = planner.plan(&request).expect("planning should work");

        assert_eq!(plan.requests.len(), 2);
        assert_eq!(plan.requests[0].api_version, ApiVersion::V2);
        assert_eq!(plan.requests[0].path, "/user");
        assert_eq!(
            plan.requests[0].query.get("selections"),
            Some(&"profile".to_string())
        );

        assert_eq!(plan.requests[1].api_version, ApiVersion::V1);
        assert_eq!(plan.requests[1].path, "/user/3637232");
        assert_eq!(
            plan.requests[1].query.get("selections"),
            Some(&"bazaar".to_string())
        );
    }

    #[test]
    fn supports_v1_only_company_resource() {
        let planner = planner_from_fixture();
        let request = PlanRequest::new("company", vec!["profile", "employees"]).with_id("55");
        let plan = planner.plan(&request).expect("planning should work");

        assert_eq!(plan.requests.len(), 1);
        assert_eq!(plan.requests[0].api_version, ApiVersion::V1);
        assert_eq!(plan.requests[0].path, "/company/55");
        assert_eq!(
            plan.requests[0].query.get("selections"),
            Some(&"profile,employees".to_string())
        );
    }

    #[test]
    fn scrubs_unknown_filters_from_v1_requests() {
        let planner = planner_from_fixture();
        let request = PlanRequest::new("company", vec!["profile"])
            .with_id("55")
            .with_filter("limit", "25")
            .with_filter("timestamp", "1")
            .with_filter("foo", "bar");
        let plan = planner.plan(&request).expect("planning should work");

        assert_eq!(plan.requests.len(), 1);
        let query = &plan.requests[0].query;
        assert_eq!(query.get("limit"), Some(&"25".to_string()));
        assert_eq!(query.get("timestamp"), Some(&"1".to_string()));
        assert!(!query.contains_key("foo"));
    }
}
