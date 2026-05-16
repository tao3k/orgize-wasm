//! Stable TypeScript-facing DTO models for dynamic block projections.

use crate::dto_shared_model::WasmSourceRange;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmDynamicBlocksResponse {
    pub(crate) schema_version: u8,
    pub(crate) records: Vec<WasmDynamicBlockRecord>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmDynamicBlockRecord {
    pub(crate) source: WasmSourceRange,
    pub(crate) name: String,
    pub(crate) writer: &'static str,
    pub(crate) parameters: Vec<WasmDynamicBlockParameter>,
    pub(crate) content_state: &'static str,
    pub(crate) content_line_count: usize,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmDynamicBlockParameter {
    pub(crate) key: String,
    pub(crate) value: Option<String>,
    pub(crate) raw: String,
}
