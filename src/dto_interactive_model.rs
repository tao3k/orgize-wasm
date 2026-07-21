//! Stable TypeScript-facing DTO models for Org-Interactive projections.

use crate::dto_shared_model::WasmSourceRange;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmOrgInteractiveResponse {
    pub(crate) schema_version: u8,
    pub(crate) choices: Vec<WasmOrgInteractiveChoice>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmOrgInteractiveChoice {
    pub(crate) source: WasmSourceRange,
    pub(crate) id: String,
    pub(crate) method: String,
    pub(crate) stage: String,
    pub(crate) group: Option<String>,
    pub(crate) target: Option<String>,
    pub(crate) create: Option<String>,
    pub(crate) info: String,
    pub(crate) categories: Vec<WasmOrgInteractiveCategory>,
    pub(crate) entries: Vec<WasmOrgInteractiveChoiceEntry>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmOrgInteractiveCategory {
    pub(crate) key: String,
    pub(crate) value: String,
    pub(crate) detail: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmOrgInteractiveChoiceEntry {
    pub(crate) number: String,
    pub(crate) id: String,
    pub(crate) contract: Option<String>,
    pub(crate) full: String,
    pub(crate) use_if: String,
}
