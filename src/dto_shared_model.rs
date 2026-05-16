//! Shared WebAssembly DTO primitives used across projection-specific models.

use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmSourcePosition {
    pub(crate) line: usize,
    pub(crate) column: usize,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmSourceRange {
    pub(crate) start: WasmSourcePosition,
    pub(crate) end: WasmSourcePosition,
    pub(crate) range_start: u32,
    pub(crate) range_end: u32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmOrgDuration {
    pub(crate) raw: String,
    pub(crate) total_seconds: u64,
    pub(crate) total_minutes: f64,
}
