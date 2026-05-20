//! Stable TypeScript-facing DTO models for runtime metadata projections.

use crate::dto_shared_model::WasmSourceRange;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmRuntimeMetadataResponse {
    pub(crate) schema_version: u8,
    pub(crate) feeds: Vec<WasmFeedStatusRecord>,
    pub(crate) timers: Vec<WasmTimerRecord>,
    pub(crate) mobile: WasmMobileSyncMetadata,
    pub(crate) boundaries: Vec<WasmRuntimeMetadataBoundary>,
    pub(crate) warnings: Vec<WasmRuntimeMetadataWarning>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmFeedStatusRecord {
    pub(crate) source: WasmSourceRange,
    pub(crate) section_title: String,
    pub(crate) drawer: String,
    pub(crate) raw: String,
    pub(crate) entry_count: usize,
    pub(crate) readable: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmTimerRecord {
    pub(crate) source: WasmSourceRange,
    pub(crate) outline_path: Vec<String>,
    pub(crate) context: &'static str,
    pub(crate) raw: String,
    pub(crate) total_seconds: i64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmMobileSyncMetadata {
    pub(crate) readonly: Vec<WasmMobileReadonlyKeyword>,
    pub(crate) all_priorities: Vec<WasmMobilePriorityDeclaration>,
    pub(crate) index_links: Vec<WasmMobileIndexLink>,
    pub(crate) flagged_sections: Vec<WasmMobileFlaggedSection>,
    pub(crate) original_ids: Vec<WasmMobileOriginalId>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmMobileReadonlyKeyword {
    pub(crate) source: WasmSourceRange,
    pub(crate) value: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmMobilePriorityDeclaration {
    pub(crate) source: WasmSourceRange,
    pub(crate) values: Vec<String>,
    pub(crate) raw: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmMobileIndexLink {
    pub(crate) source: WasmSourceRange,
    pub(crate) title: String,
    pub(crate) file: String,
    pub(crate) description: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmMobileFlaggedSection {
    pub(crate) source: WasmSourceRange,
    pub(crate) outline_path: Vec<String>,
    pub(crate) title: String,
    pub(crate) original_id: Option<String>,
    pub(crate) mobile_properties: Vec<WasmMobileProperty>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmMobileOriginalId {
    pub(crate) source: WasmSourceRange,
    pub(crate) outline_path: Vec<String>,
    pub(crate) title: String,
    pub(crate) value: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmMobileProperty {
    pub(crate) source: WasmSourceRange,
    pub(crate) key: String,
    pub(crate) value: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmRuntimeMetadataBoundary {
    pub(crate) kind: &'static str,
    pub(crate) message: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmRuntimeMetadataWarning {
    pub(crate) kind: &'static str,
    pub(crate) message: String,
}
