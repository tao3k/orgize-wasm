//! Stable TypeScript-facing DTO models for Agent capture plans.

use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmAgentCapturePlanResponse {
    pub(crate) schema_version: u8,
    pub(crate) plan: WasmAgentCapturePlan,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmAgentCapturePlan {
    pub(crate) target: WasmAgentCaptureTarget,
    pub(crate) org_entry: String,
    pub(crate) receipts: Vec<WasmAgentCaptureReceipt>,
    pub(crate) warnings: Vec<WasmAgentCaptureWarning>,
    pub(crate) requires_confirmation: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmAgentCaptureTarget {
    pub(crate) kind: &'static str,
    pub(crate) source_file: Option<String>,
    pub(crate) outline_path: Vec<String>,
    pub(crate) date: Option<WasmAgentCaptureDate>,
    pub(crate) insert_position: &'static str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmAgentCaptureDate {
    pub(crate) year: u16,
    pub(crate) month: u8,
    pub(crate) day: u8,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmAgentCaptureReceipt {
    pub(crate) kind: &'static str,
    pub(crate) message: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmAgentCaptureWarning {
    pub(crate) kind: &'static str,
    pub(crate) message: String,
}
