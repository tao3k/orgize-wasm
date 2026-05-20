//! Stable TypeScript-facing DTO models for memory projections.

use crate::dto_shared_model::WasmSourceRange;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmMemoryResponse {
    pub(crate) schema_version: u8,
    pub(crate) stats: WasmMemoryStats,
    pub(crate) records: Vec<WasmMemoryRecord>,
    pub(crate) cards: Vec<WasmAgentMemoryCard>,
    pub(crate) evidence_kinds: Vec<WasmMemoryFacet>,
    pub(crate) authority_kinds: Vec<WasmMemoryFacet>,
}

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmMemoryStats {
    pub(crate) total_records: usize,
    pub(crate) current_records: usize,
    pub(crate) background_records: usize,
    pub(crate) closed_records: usize,
    pub(crate) archived_records: usize,
    pub(crate) cards: usize,
    pub(crate) action_cards: usize,
    pub(crate) suppressed_cards: usize,
    pub(crate) info_cards: usize,
    pub(crate) evidence: usize,
    pub(crate) properties: usize,
    pub(crate) links: usize,
    pub(crate) authority_reasons: usize,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmMemoryProperty {
    pub(crate) source: WasmSourceRange,
    pub(crate) key: String,
    pub(crate) value: String,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmMemoryEvidence {
    pub(crate) source: WasmSourceRange,
    pub(crate) kind: WasmMemoryEvidenceKind,
    pub(crate) value: String,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmMemoryEvidenceKind {
    pub(crate) code: &'static str,
    pub(crate) label: String,
    pub(crate) family: &'static str,
    pub(crate) detail: Option<String>,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmMemoryLink {
    pub(crate) source: WasmSourceRange,
    pub(crate) path: String,
    pub(crate) description: String,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmMemoryAuthorityReason {
    pub(crate) kind: &'static str,
    pub(crate) label: &'static str,
    pub(crate) message: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmAgentMemoryDecision {
    pub(crate) code: &'static str,
    pub(crate) kind: &'static str,
    pub(crate) severity: &'static str,
    pub(crate) title: &'static str,
    pub(crate) next_action: &'static str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmMemoryRecord {
    pub(crate) source: WasmSourceRange,
    pub(crate) state: &'static str,
    pub(crate) level: usize,
    pub(crate) title: String,
    pub(crate) todo: Option<String>,
    pub(crate) todo_state: Option<&'static str>,
    pub(crate) tags: Vec<String>,
    pub(crate) effective_tags: Vec<String>,
    pub(crate) anchor: Option<String>,
    pub(crate) properties: Vec<WasmMemoryProperty>,
    pub(crate) evidence: Vec<WasmMemoryEvidence>,
    pub(crate) links: Vec<WasmMemoryLink>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmAgentMemoryCard {
    pub(crate) source: WasmSourceRange,
    pub(crate) decision: WasmAgentMemoryDecision,
    pub(crate) authority: Vec<WasmMemoryAuthorityReason>,
    pub(crate) title: String,
    pub(crate) todo: Option<String>,
    pub(crate) todo_state: Option<&'static str>,
    pub(crate) tags: Vec<String>,
    pub(crate) effective_tags: Vec<String>,
    pub(crate) anchor: Option<String>,
    pub(crate) evidence: Vec<WasmMemoryEvidence>,
    pub(crate) links: Vec<WasmMemoryLink>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmMemoryFacet {
    pub(crate) code: String,
    pub(crate) label: String,
    pub(crate) count: usize,
}
