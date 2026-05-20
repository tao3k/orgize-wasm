//! Stable TypeScript-facing DTO models for browser projections.

use crate::dto_clock_model::{WasmClockIssueFinding, WasmClockRollupRecord, WasmClockTablePlan};
use crate::dto_crypt_model::WasmCryptRecord;
use crate::dto_dynamic_block_model::WasmDynamicBlockRecord;
use crate::dto_memory_model::WasmMemoryResponse;
use crate::dto_property_profile_model::WasmPropertyProfile;
use crate::dto_refile_model::WasmRefileTarget;
use crate::dto_runtime_model::WasmRuntimeMetadataResponse;
use crate::dto_sdd_model::WasmSddNodeRecord;
use crate::dto_shared_model::{WasmOrgDuration, WasmSourceRange};
use crate::dto_source_block_model::WasmSourceBlockRecord;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmOutlineResponse {
    pub(crate) schema_version: u8,
    pub(crate) nodes: Vec<WasmOutlineNode>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmMetadataResponse {
    pub(crate) schema_version: u8,
    pub(crate) properties: Vec<WasmProperty>,
    pub(crate) keywords: Vec<WasmKeyword>,
    pub(crate) filetags: Vec<String>,
    pub(crate) tag_definitions: Vec<WasmTagDefinition>,
    pub(crate) export_settings: WasmExportSettings,
    pub(crate) link_abbreviations: Vec<WasmLinkAbbreviation>,
    pub(crate) includes: Vec<WasmIncludeDirective>,
    pub(crate) macros: Vec<WasmMacroDefinition>,
    pub(crate) targets: Vec<WasmTargetDefinition>,
    pub(crate) footnotes: Vec<WasmFootnoteEntry>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmLintResponse {
    pub(crate) schema_version: u8,
    pub(crate) findings: Vec<WasmLintFinding>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmSectionIndexResponse {
    pub(crate) schema_version: u8,
    pub(crate) records: Vec<WasmSectionIndexRecord>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmSparseTreeResponse {
    pub(crate) schema_version: u8,
    pub(crate) total_candidates: usize,
    pub(crate) cards: Vec<WasmSparseTreeCard>,
    pub(crate) skipped: Vec<WasmSparseTreeSkip>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmAgendaViewResponse {
    pub(crate) schema_version: u8,
    pub(crate) total_candidates: usize,
    pub(crate) limit: Option<usize>,
    pub(crate) sort_strategy: Vec<WasmAgendaViewSortSpec>,
    pub(crate) cards: Vec<WasmAgendaViewCard>,
    pub(crate) skipped: Vec<WasmAgendaViewSkip>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmAgendaBlockViewResponse {
    pub(crate) schema_version: u8,
    pub(crate) title: String,
    pub(crate) total_candidates: usize,
    pub(crate) sections: Vec<WasmAgendaBlockSectionPlan>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmAgendaBlockSectionPlan {
    pub(crate) index: usize,
    pub(crate) name: String,
    pub(crate) plan: WasmAgendaViewResponse,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmViewIndexResponse {
    pub(crate) schema_version: u8,
    pub(crate) records: Vec<WasmViewIndexRecord>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmAttachmentsResponse {
    pub(crate) schema_version: u8,
    pub(crate) records: Vec<WasmAttachmentRecord>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmColumnViewsResponse {
    pub(crate) schema_version: u8,
    pub(crate) records: Vec<WasmColumnViewRecord>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmIncludeExpansionResponse {
    pub(crate) schema_version: u8,
    pub(crate) entries: Vec<WasmIncludeExpansionEntry>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmDateTreeResponse {
    pub(crate) schema_version: u8,
    pub(crate) records: Vec<WasmDateTreeEntry>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmProgressStatsResponse {
    pub(crate) schema_version: u8,
    pub(crate) records: Vec<WasmProgressStatsRecord>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmTaskBlockersResponse {
    pub(crate) schema_version: u8,
    pub(crate) records: Vec<WasmTaskBlockerRecord>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmSnapshotResponse {
    pub(crate) schema_version: u8,
    pub(crate) metadata: WasmMetadataResponse,
    pub(crate) outline: Vec<WasmOutlineNode>,
    pub(crate) section_index: Vec<WasmSectionIndexRecord>,
    pub(crate) attachments: Vec<WasmAttachmentRecord>,
    pub(crate) source_blocks: Vec<WasmSourceBlockRecord>,
    pub(crate) column_views: Vec<WasmColumnViewRecord>,
    pub(crate) dynamic_blocks: Vec<WasmDynamicBlockRecord>,
    pub(crate) property_profile: WasmPropertyProfile,
    pub(crate) refile_targets: Vec<WasmRefileTarget>,
    pub(crate) include_expansion: Vec<WasmIncludeExpansionEntry>,
    pub(crate) datetree: Vec<WasmDateTreeEntry>,
    pub(crate) progress_stats: Vec<WasmProgressStatsRecord>,
    pub(crate) clock_rollups: Vec<WasmClockRollupRecord>,
    pub(crate) clock_table_plans: Vec<WasmClockTablePlan>,
    pub(crate) clock_issues: Vec<WasmClockIssueFinding>,
    pub(crate) sdd: Vec<WasmSddNodeRecord>,
    pub(crate) memory: WasmMemoryResponse,
    pub(crate) crypt: Vec<WasmCryptRecord>,
    pub(crate) runtime_metadata: WasmRuntimeMetadataResponse,
    pub(crate) lint: Vec<WasmLintFinding>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmOutlineNode {
    pub(crate) source: WasmSourceRange,
    pub(crate) level: usize,
    pub(crate) title: String,
    pub(crate) anchor: Option<String>,
    pub(crate) todo: Option<String>,
    pub(crate) todo_state: Option<&'static str>,
    pub(crate) priority: WasmPriority,
    pub(crate) tags: Vec<String>,
    pub(crate) effective_tags: Vec<String>,
    pub(crate) is_comment: bool,
    pub(crate) archive: WasmArchive,
    pub(crate) attachment: WasmAttachmentState,
    pub(crate) children: Vec<WasmOutlineNode>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmSectionIndexRecord {
    pub(crate) source: WasmSourceRange,
    pub(crate) outline_path: Vec<String>,
    pub(crate) level: usize,
    pub(crate) title: String,
    pub(crate) body: Vec<WasmTextSlice>,
    pub(crate) todo: Option<String>,
    pub(crate) todo_state: Option<&'static str>,
    pub(crate) priority: WasmPriority,
    pub(crate) category: Option<String>,
    pub(crate) tags: Vec<String>,
    pub(crate) effective_tags: Vec<String>,
    pub(crate) properties: Vec<WasmProperty>,
    pub(crate) effective_properties: Vec<WasmProperty>,
    pub(crate) special_properties: Vec<WasmProperty>,
    pub(crate) planning: WasmPlanning,
    pub(crate) is_comment: bool,
    pub(crate) archive: WasmArchive,
    pub(crate) attachment: WasmAttachmentState,
    pub(crate) links: Vec<WasmLink>,
    pub(crate) targets: Vec<WasmTarget>,
    pub(crate) lifecycle: Vec<WasmLifecycleRecord>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmSparseTreeCard {
    pub(crate) source: WasmSourceRange,
    pub(crate) outline_path: Vec<String>,
    pub(crate) level: usize,
    pub(crate) title: String,
    pub(crate) matches: Vec<WasmSparseTreeMatch>,
    pub(crate) receipts: Vec<WasmSparseTreeReceipt>,
    pub(crate) preview: Option<String>,
    pub(crate) todo: Option<String>,
    pub(crate) todo_state: Option<&'static str>,
    pub(crate) priority: WasmPriority,
    pub(crate) category: Option<String>,
    pub(crate) tags: Vec<String>,
    pub(crate) effective_tags: Vec<String>,
    pub(crate) properties: Vec<WasmProperty>,
    pub(crate) special_properties: Vec<WasmProperty>,
    pub(crate) planning: WasmPlanning,
    pub(crate) archive: WasmArchive,
    pub(crate) attachment: WasmAttachmentState,
    pub(crate) links: Vec<WasmLink>,
    pub(crate) targets: Vec<WasmTarget>,
    pub(crate) lifecycle: Vec<WasmLifecycleRecord>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmSparseTreeSkip {
    pub(crate) source: WasmSourceRange,
    pub(crate) outline_path: Vec<String>,
    pub(crate) level: usize,
    pub(crate) title: String,
    pub(crate) reason: &'static str,
    pub(crate) receipts: Vec<WasmSparseTreeReceipt>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmSparseTreeMatch {
    pub(crate) source: WasmSourceRange,
    pub(crate) kind: &'static str,
    pub(crate) key: Option<String>,
    pub(crate) value: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmSparseTreeReceipt {
    pub(crate) kind: &'static str,
    pub(crate) message: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmAgendaViewCard {
    pub(crate) source: WasmSourceRange,
    pub(crate) sorted_position: usize,
    pub(crate) kind: &'static str,
    pub(crate) display_date: String,
    pub(crate) target_date: String,
    pub(crate) target_end_date: Option<String>,
    pub(crate) time: Option<String>,
    pub(crate) end_time: Option<String>,
    pub(crate) title: String,
    pub(crate) category: Option<String>,
    pub(crate) todo: Option<String>,
    pub(crate) todo_state: Option<&'static str>,
    pub(crate) effective_tags: Vec<String>,
    pub(crate) blockers: Vec<WasmTaskBlockerRecord>,
    pub(crate) urgency: WasmAgendaUrgencyScore,
    pub(crate) sort_keys: Vec<WasmAgendaViewSortValue>,
    pub(crate) receipts: Vec<WasmAgendaViewReceipt>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmAgendaViewSkip {
    pub(crate) source: WasmSourceRange,
    pub(crate) sorted_position: usize,
    pub(crate) title: String,
    pub(crate) reason: &'static str,
    pub(crate) limit: Option<usize>,
    pub(crate) blockers: Vec<WasmTaskBlockerRecord>,
    pub(crate) urgency: WasmAgendaUrgencyScore,
    pub(crate) sort_keys: Vec<WasmAgendaViewSortValue>,
    pub(crate) receipts: Vec<WasmAgendaViewReceipt>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmAgendaUrgencyScore {
    pub(crate) total: i32,
    pub(crate) ingredients: Vec<WasmAgendaUrgencyIngredient>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmAgendaUrgencyIngredient {
    pub(crate) kind: &'static str,
    pub(crate) score: i32,
    pub(crate) message: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmAgendaViewSortValue {
    pub(crate) key: &'static str,
    pub(crate) value: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmAgendaViewSortSpec {
    pub(crate) key: &'static str,
    pub(crate) direction: &'static str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmAgendaViewReceipt {
    pub(crate) kind: &'static str,
    pub(crate) message: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmProgressStatsRecord {
    pub(crate) source: WasmSourceRange,
    pub(crate) outline_path: Vec<String>,
    pub(crate) level: usize,
    pub(crate) title: String,
    pub(crate) todo: &'static str,
    pub(crate) descendant_todos: WasmProgressTodoSummary,
    pub(crate) checkboxes: WasmProgressCheckboxSummary,
    pub(crate) statistic_cookies: Vec<WasmProgressStatisticCookie>,
    pub(crate) effort: WasmProgressEffortSummary,
    pub(crate) dependencies: Vec<WasmTaskDependencyRecord>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmProgressTodoSummary {
    pub(crate) total: u32,
    pub(crate) done: u32,
    pub(crate) open: u32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmProgressCheckboxSummary {
    pub(crate) total: u32,
    pub(crate) checked: u32,
    pub(crate) unchecked: u32,
    pub(crate) partial: u32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmProgressStatisticCookie {
    pub(crate) source: WasmSourceRange,
    pub(crate) raw: String,
    pub(crate) kind: &'static str,
    pub(crate) done: Option<u32>,
    pub(crate) total: Option<u32>,
    pub(crate) percent: Option<u8>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmProgressEffortSummary {
    pub(crate) local: Option<WasmOrgDuration>,
    pub(crate) subtree_total_seconds: u64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmTaskDependencyRecord {
    pub(crate) source: WasmSourceRange,
    pub(crate) kind: &'static str,
    pub(crate) count: u32,
    pub(crate) message: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmTaskBlockerRecord {
    pub(crate) kind: &'static str,
    pub(crate) blocked: WasmTaskBlockerTask,
    pub(crate) blocker: WasmTaskBlockerTask,
    pub(crate) parent: WasmTaskBlockerParent,
    pub(crate) message: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmTaskBlockerTask {
    pub(crate) source: WasmSourceRange,
    pub(crate) outline_path: Vec<String>,
    pub(crate) level: usize,
    pub(crate) title: String,
    pub(crate) todo: Option<String>,
    pub(crate) todo_state: Option<&'static str>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmTaskBlockerParent {
    pub(crate) source: WasmSourceRange,
    pub(crate) ordered_property_source: WasmSourceRange,
    pub(crate) outline_path: Vec<String>,
    pub(crate) level: usize,
    pub(crate) title: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmViewIndexRecord {
    pub(crate) range_start: u32,
    pub(crate) outline: String,
    pub(crate) level: usize,
    pub(crate) title: String,
    pub(crate) body_preview: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) todo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) todo_state: Option<&'static str>,
    pub(crate) effective_tags: Vec<String>,
    pub(crate) properties: Vec<WasmViewProperty>,
    pub(crate) planning: WasmViewPlanning,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmViewProperty {
    pub(crate) key: String,
    pub(crate) value: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmViewPlanning {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) deadline: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) scheduled: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) closed: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmTextSlice {
    pub(crate) source: WasmSourceRange,
    pub(crate) text: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmProperty {
    pub(crate) source: WasmSourceRange,
    pub(crate) key: String,
    pub(crate) value: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmPlanning {
    pub(crate) deadline: Option<WasmTimestamp>,
    pub(crate) scheduled: Option<WasmTimestamp>,
    pub(crate) closed: Option<WasmTimestamp>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmTimestamp {
    pub(crate) kind: &'static str,
    pub(crate) raw: String,
    pub(crate) is_range: bool,
    pub(crate) start: Option<WasmTimestampMoment>,
    pub(crate) end: Option<WasmTimestampMoment>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmTimestampMoment {
    pub(crate) year: u16,
    pub(crate) month: u8,
    pub(crate) day: u8,
    pub(crate) day_name: Option<String>,
    pub(crate) hour: Option<u8>,
    pub(crate) minute: Option<u8>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmPriority {
    pub(crate) raw: Option<String>,
    pub(crate) effective: String,
    pub(crate) is_default: bool,
    pub(crate) score: Option<i32>,
    pub(crate) range_status: &'static str,
    pub(crate) profile: WasmPriorityProfile,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmPriorityProfile {
    pub(crate) highest: String,
    pub(crate) lowest: String,
    pub(crate) default: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmArchive {
    pub(crate) archived: bool,
    pub(crate) has_archive_tag: bool,
    pub(crate) location: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmAttachmentState {
    pub(crate) has_attach_tag: bool,
    pub(crate) directory: Option<WasmAttachmentDirectory>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmAttachmentDirectory {
    pub(crate) source: WasmAttachmentDirectorySource,
    pub(crate) path: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmAttachmentDirectorySource {
    pub(crate) kind: &'static str,
    pub(crate) id: Option<String>,
    pub(crate) layout: Option<&'static str>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmLink {
    pub(crate) source: WasmSourceRange,
    pub(crate) path: String,
    pub(crate) description: String,
    pub(crate) search: Option<WasmLinkSearch>,
    pub(crate) attachment: Option<WasmAttachmentLink>,
    pub(crate) file: Option<WasmFileLink>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmLinkSearch {
    pub(crate) raw: String,
    pub(crate) kind: &'static str,
    pub(crate) normalized: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmAttachmentLink {
    pub(crate) path: String,
    pub(crate) search: Option<WasmAttachmentLinkSearch>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmAttachmentLinkSearch {
    pub(crate) raw: String,
    pub(crate) kind: &'static str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmFileLink {
    pub(crate) protocol: String,
    pub(crate) path: String,
    pub(crate) path_kind: &'static str,
    pub(crate) search: Option<WasmLinkSearch>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmTarget {
    pub(crate) source: WasmSourceRange,
    pub(crate) kind: &'static str,
    pub(crate) key: String,
    pub(crate) value: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmLifecycleRecord {
    pub(crate) source: WasmSourceRange,
    pub(crate) kind: String,
    pub(crate) raw: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmAttachmentRecord {
    pub(crate) source: WasmSourceRange,
    pub(crate) section_title: String,
    pub(crate) outline_path: Vec<String>,
    pub(crate) directory: Option<WasmAttachmentDirectory>,
    pub(crate) links: Vec<WasmAttachmentLink>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmColumnViewRecord {
    pub(crate) source: WasmSourceRange,
    pub(crate) scope: WasmColumnViewScope,
    pub(crate) raw: String,
    pub(crate) columns: Vec<WasmColumnViewColumn>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmColumnViewScope {
    pub(crate) kind: &'static str,
    pub(crate) level: Option<usize>,
    pub(crate) title: Option<String>,
    pub(crate) outline_path: Vec<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmColumnViewColumn {
    pub(crate) property: String,
    pub(crate) title: Option<String>,
    pub(crate) width: Option<usize>,
    pub(crate) summary_operator: Option<String>,
    pub(crate) summary_format: Option<String>,
    pub(crate) raw: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmKeyword {
    pub(crate) source: WasmSourceRange,
    pub(crate) key: String,
    pub(crate) optional: Option<String>,
    pub(crate) value: String,
    pub(crate) parsed_object_count: usize,
    pub(crate) attributes: Vec<WasmKeywordAttribute>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmKeywordAttribute {
    pub(crate) key: String,
    pub(crate) value: Option<String>,
    pub(crate) raw: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmTagDefinition {
    pub(crate) name: String,
    pub(crate) shortcut: Option<String>,
    pub(crate) raw: String,
    pub(crate) is_group: bool,
    pub(crate) group: Option<WasmTagDefinitionGroup>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmTagDefinitionGroup {
    pub(crate) name: Option<String>,
    pub(crate) exclusive: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmExportSettings {
    pub(crate) select_tags: Vec<String>,
    pub(crate) exclude_tags: Vec<String>,
    pub(crate) headline_levels: Option<usize>,
    pub(crate) special_strings: Option<bool>,
    pub(crate) expand_entities: Option<bool>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmLinkAbbreviation {
    pub(crate) name: String,
    pub(crate) replacement: String,
    pub(crate) raw_value: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmIncludeDirective {
    pub(crate) source: WasmSourceRange,
    pub(crate) path: String,
    pub(crate) raw_path: String,
    pub(crate) arguments: Vec<String>,
    pub(crate) options: Vec<WasmIncludeOption>,
    pub(crate) raw_value: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmIncludeOption {
    pub(crate) key: String,
    pub(crate) value: Option<String>,
    pub(crate) raw: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmIncludeExpansionEntry {
    pub(crate) directive: WasmIncludeDirective,
    pub(crate) resolved_path: Option<String>,
    pub(crate) line_selection: WasmIncludeLineSelection,
    pub(crate) min_level: Option<usize>,
    pub(crate) mode: WasmIncludeExpansionMode,
    pub(crate) options: Vec<WasmIncludeOption>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmIncludeLineSelection {
    pub(crate) kind: &'static str,
    pub(crate) start: Option<usize>,
    pub(crate) end: Option<usize>,
    pub(crate) raw: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmIncludeExpansionMode {
    pub(crate) kind: &'static str,
    pub(crate) language: Option<String>,
    pub(crate) backend: Option<String>,
    pub(crate) arguments: Vec<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmDateTreeEntry {
    pub(crate) source: WasmSourceRange,
    pub(crate) date: String,
    pub(crate) year: u16,
    pub(crate) month: u8,
    pub(crate) day: u8,
    pub(crate) year_title: String,
    pub(crate) month_title: String,
    pub(crate) day_title: String,
    pub(crate) outline_path: Vec<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmMacroDefinition {
    pub(crate) source: WasmSourceRange,
    pub(crate) name: String,
    pub(crate) template: String,
    pub(crate) raw_value: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmTargetDefinition {
    pub(crate) source: WasmSourceRange,
    pub(crate) kind: &'static str,
    pub(crate) key: String,
    pub(crate) value: String,
    pub(crate) raw: String,
    pub(crate) alias_object_count: usize,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmFootnoteEntry {
    pub(crate) source: WasmSourceRange,
    pub(crate) label: String,
    pub(crate) definition_kind: &'static str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmLintFinding {
    pub(crate) code: &'static str,
    pub(crate) severity: &'static str,
    pub(crate) message: String,
    pub(crate) source: WasmSourceRange,
}
