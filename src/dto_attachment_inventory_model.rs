//! Attachment inventory DTO models.

use crate::dto_model::{WasmAttachmentDirectorySource, WasmAttachmentLink};
use crate::dto_shared_model::WasmSourceRange;
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmAttachmentInventoryRequest {
    pub(crate) base_dir: Option<String>,
    pub(crate) attach_id_dir: Option<String>,
    pub(crate) check_vcs: Option<bool>,
    pub(crate) check_annex: Option<bool>,
    pub(crate) archive_delete_policy: Option<String>,
    pub(crate) scan_orphans: Option<bool>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmAttachmentInventoryResponse {
    pub(crate) schema_version: u8,
    pub(crate) entries: Vec<WasmAttachmentInventoryEntry>,
    pub(crate) display: Vec<WasmAttachmentDisplayRecord>,
    pub(crate) sync_plan: WasmAttachmentSyncPlan,
    pub(crate) archive_advice: Vec<WasmAttachmentArchiveAdvice>,
    pub(crate) warnings: Vec<WasmAttachmentInventoryWarning>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmAttachmentInventoryEntry {
    pub(crate) source: WasmSourceRange,
    pub(crate) section_title: String,
    pub(crate) kind: WasmAttachmentInventoryEntryKind,
    pub(crate) path: String,
    pub(crate) absolute_path: String,
    pub(crate) exists: bool,
    pub(crate) vcs: WasmAttachmentVcsEvidence,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmAttachmentInventoryEntryKind {
    pub(crate) label: &'static str,
    pub(crate) directory_source: Option<WasmAttachmentDirectorySource>,
    pub(crate) link: Option<WasmAttachmentLink>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmAttachmentDisplayRecord {
    pub(crate) source: WasmSourceRange,
    pub(crate) section_title: String,
    pub(crate) outline_path: Vec<String>,
    pub(crate) tags: Vec<String>,
    pub(crate) effective_tags: Vec<String>,
    pub(crate) attachment_id: Option<String>,
    pub(crate) directory_path: String,
    pub(crate) link_path: String,
    pub(crate) absolute_path: String,
    pub(crate) exists: bool,
    pub(crate) media_kind: &'static str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmAttachmentSyncPlan {
    pub(crate) actions: Vec<WasmAttachmentSyncAction>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmAttachmentSyncAction {
    pub(crate) kind: &'static str,
    pub(crate) source: WasmSourceRange,
    pub(crate) section_title: String,
    pub(crate) path: String,
    pub(crate) absolute_path: Option<String>,
    pub(crate) message: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmAttachmentArchiveAdvice {
    pub(crate) source: WasmSourceRange,
    pub(crate) section_title: String,
    pub(crate) policy: &'static str,
    pub(crate) path: String,
    pub(crate) message: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmAttachmentInventoryWarning {
    pub(crate) kind: &'static str,
    pub(crate) message: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmAttachmentVcsEvidence {
    pub(crate) status: &'static str,
    pub(crate) annex: WasmAttachmentAnnexEvidence,
    pub(crate) raw: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmAttachmentAnnexEvidence {
    pub(crate) status: &'static str,
    pub(crate) raw: Option<String>,
}
