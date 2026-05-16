//! Stable TypeScript-facing DTO models for refile projections.

use crate::dto_shared_model::WasmSourceRange;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmRefileTargetIndexResponse {
    pub(crate) schema_version: u8,
    pub(crate) source_file: Option<String>,
    pub(crate) outline_path_mode: &'static str,
    pub(crate) specs: Vec<WasmRefileTargetSpec>,
    pub(crate) targets: Vec<WasmRefileTarget>,
    pub(crate) warnings: Vec<WasmRefileWarning>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmRefileTarget {
    pub(crate) source_file: Option<String>,
    pub(crate) source: WasmSourceRange,
    pub(crate) level: usize,
    pub(crate) title: String,
    pub(crate) outline_path: Vec<String>,
    pub(crate) display: String,
    pub(crate) receipts: Vec<WasmRefileTargetReceipt>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmRefileTargetReceipt {
    pub(crate) spec: WasmRefileTargetSpec,
    pub(crate) message: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmRefileTargetSpec {
    pub(crate) kind: &'static str,
    pub(crate) value: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmRefilePlanResponse {
    pub(crate) schema_version: u8,
    pub(crate) plan: WasmRefilePlan,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmRefilePlan {
    pub(crate) source_file: Option<String>,
    pub(crate) action: &'static str,
    pub(crate) insert_position: &'static str,
    pub(crate) parent_creation: &'static str,
    pub(crate) source: Option<WasmRefilePlanSection>,
    pub(crate) target: Option<WasmRefileTarget>,
    pub(crate) created_target: Option<WasmRefileCreateParentPlan>,
    pub(crate) receipts: Vec<WasmRefilePlanReceipt>,
    pub(crate) warnings: Vec<WasmRefileWarning>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmRefileCreateParentPlan {
    pub(crate) source_file: Option<String>,
    pub(crate) existing_parent: WasmRefileTarget,
    pub(crate) target_outline_path: Vec<String>,
    pub(crate) nodes: Vec<WasmRefileCreateParentNode>,
    pub(crate) requires_confirmation: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmRefileCreateParentNode {
    pub(crate) title: String,
    pub(crate) level: usize,
    pub(crate) outline_path: Vec<String>,
    pub(crate) display: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmRefilePlanSection {
    pub(crate) source_file: Option<String>,
    pub(crate) source: WasmSourceRange,
    pub(crate) level: usize,
    pub(crate) title: String,
    pub(crate) outline_path: Vec<String>,
    pub(crate) local_ids: Vec<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmRefilePlanReceipt {
    pub(crate) kind: &'static str,
    pub(crate) message: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmRefileWarning {
    pub(crate) kind: &'static str,
    pub(crate) message: String,
}
