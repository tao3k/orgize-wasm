//! Stable TypeScript-facing DTO models for property profile projections.

use crate::dto_shared_model::WasmSourceRange;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmPropertyProfileResponse {
    pub(crate) schema_version: u8,
    pub(crate) profile: WasmPropertyProfile,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmPropertyProfile {
    pub(crate) inheritance: &'static str,
    pub(crate) inherited_keys: Vec<String>,
    pub(crate) allowed_values: Vec<WasmPropertyAllowedValueRecord>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmPropertyAllowedValueRecord {
    pub(crate) source: Option<WasmSourceRange>,
    pub(crate) scope: WasmPropertyAllowedValueScope,
    pub(crate) property: String,
    pub(crate) descriptor_key: String,
    pub(crate) values: Vec<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmPropertyAllowedValueScope {
    pub(crate) kind: &'static str,
    pub(crate) outline_path: Vec<String>,
    pub(crate) level: Option<usize>,
    pub(crate) title: Option<String>,
}
