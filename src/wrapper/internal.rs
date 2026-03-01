use serde::de::DeserializeOwned;

use crate::client::{DataRequestOptions, TornClient};
use crate::models::generated::RawSelectionBundle;
use crate::transport::HttpTransport;
use crate::wrapper::error::SdkError;

pub(crate) async fn execute_raw<T: HttpTransport>(
    client: &TornClient<T>,
    resource: &str,
    selections: Vec<String>,
    options: DataRequestOptions,
) -> Result<RawSelectionBundle, SdkError> {
    let report = client
        .get_resource_data(resource, selections, options)
        .await?;
    serde_json::from_value(report.merged_json).map_err(SdkError::Decode)
}

pub(crate) async fn execute_typed<R: DeserializeOwned, T: HttpTransport>(
    client: &TornClient<T>,
    resource: &str,
    selections: Vec<String>,
    options: DataRequestOptions,
) -> Result<R, SdkError> {
    let report = client
        .get_resource_data(resource, selections, options)
        .await?;
    serde_json::from_value(report.merged_json).map_err(SdkError::Decode)
}

pub(crate) fn validate_selection(
    resource: &str,
    selection: &str,
    supported: &[&str],
) -> Result<(), SdkError> {
    if supported.contains(&selection) {
        return Ok(());
    }

    Err(SdkError::Validation(format!(
        "selection '{selection}' is not supported for resource '{resource}'"
    )))
}

pub(crate) fn validate_selections(
    resource: &str,
    selections: &[String],
    supported: &[&str],
) -> Result<(), SdkError> {
    let unknown = selections
        .iter()
        .filter(|selection| !supported.contains(&selection.as_str()))
        .cloned()
        .collect::<Vec<_>>();

    if unknown.is_empty() {
        return Ok(());
    }

    Err(SdkError::Validation(format!(
        "resource '{resource}' does not support selections: {}",
        unknown.join(",")
    )))
}

pub(crate) fn validate_required_path_arg(
    resource: &str,
    selection: &str,
    parameter: &str,
    present: bool,
) -> Result<(), SdkError> {
    if present {
        return Ok(());
    }

    Err(SdkError::Validation(format!(
        "resource '{resource}' selection '{selection}' requires '{parameter}' to be provided"
    )))
}

pub(crate) fn validate_range(
    resource: &str,
    from: Option<u64>,
    to: Option<u64>,
) -> Result<(), SdkError> {
    if let (Some(from), Some(to)) = (from, to)
        && from > to
    {
        return Err(SdkError::Validation(format!(
            "resource '{resource}' requires 'from' <= 'to' (received from={from}, to={to})"
        )));
    }

    Ok(())
}
