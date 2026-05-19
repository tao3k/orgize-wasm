//! Stable TypeScript-facing DTO models for SDD projections.

use crate::dto_shared_model::WasmSourceRange;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmSddResponse {
    pub(crate) schema_version: u8,
    pub(crate) records: Vec<WasmSddNodeRecord>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmSddNodeRecord {
    pub(crate) source: WasmSourceRange,
    pub(crate) outline_path: Vec<String>,
    pub(crate) level: usize,
    pub(crate) title: String,
    pub(crate) kind: String,
    pub(crate) kind_known: bool,
    pub(crate) id: Option<String>,
    pub(crate) parent: Option<WasmSddParentRef>,
    pub(crate) capability: Option<String>,
    pub(crate) viewpoint: Option<String>,
    pub(crate) concern: Option<String>,
    pub(crate) quality: Option<String>,
    pub(crate) rationale: Option<String>,
    pub(crate) slug: Option<String>,
    pub(crate) status: Option<String>,
    pub(crate) todo: Option<String>,
    pub(crate) todo_state: Option<&'static str>,
    pub(crate) tags: Vec<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmSddParentRef {
    pub(crate) raw: String,
    pub(crate) target_id: Option<String>,
    pub(crate) label: Option<String>,
}
