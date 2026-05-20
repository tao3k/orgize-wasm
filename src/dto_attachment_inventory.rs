//! Attachment inventory DTO projection.

use crate::{
    dto_attachment_inventory_model::{
        WasmAttachmentAnnexEvidence, WasmAttachmentArchiveAdvice, WasmAttachmentDisplayRecord,
        WasmAttachmentInventoryEntry, WasmAttachmentInventoryEntryKind,
        WasmAttachmentInventoryRequest, WasmAttachmentInventoryResponse,
        WasmAttachmentInventoryWarning, WasmAttachmentSyncAction, WasmAttachmentSyncPlan,
        WasmAttachmentVcsEvidence,
    },
    dto_common::{attachment_directory_source, attachment_link, section_source},
};
use orgize::ast::{
    AttachmentArchiveDeletePolicy, AttachmentInventory, AttachmentInventoryEntryKind,
    AttachmentInventoryOptions,
};

pub(crate) fn attachment_inventory_response(
    inventory: &AttachmentInventory,
) -> WasmAttachmentInventoryResponse {
    WasmAttachmentInventoryResponse {
        schema_version: 1,
        entries: inventory
            .entries
            .iter()
            .map(attachment_inventory_entry)
            .collect(),
        display: inventory
            .display
            .iter()
            .map(|record| WasmAttachmentDisplayRecord {
                source: section_source(&record.source),
                section_title: record.section_title.clone(),
                outline_path: record.outline_path.clone(),
                tags: record.tags.clone(),
                effective_tags: record.effective_tags.clone(),
                attachment_id: record
                    .attachment_id
                    .as_ref()
                    .map(|id| id.as_str().to_string()),
                directory_path: record.directory_path.as_str().to_string(),
                link_path: record.link_path.as_str().to_string(),
                absolute_path: record.absolute_path.as_str().to_string(),
                exists: record.exists,
                media_kind: record.media_kind.as_str(),
            })
            .collect(),
        sync_plan: WasmAttachmentSyncPlan {
            actions: inventory
                .sync_plan
                .actions
                .iter()
                .map(|action| WasmAttachmentSyncAction {
                    kind: action.kind.as_str(),
                    source: section_source(&action.source),
                    section_title: action.section_title.clone(),
                    path: action.path.clone(),
                    absolute_path: action.absolute_path.clone(),
                    message: action.message.clone(),
                })
                .collect(),
        },
        archive_advice: inventory
            .archive_advice
            .iter()
            .map(|advice| WasmAttachmentArchiveAdvice {
                source: section_source(&advice.source),
                section_title: advice.section_title.clone(),
                policy: advice.policy.as_str(),
                path: advice.path.clone(),
                message: advice.message.clone(),
            })
            .collect(),
        warnings: inventory
            .warnings
            .iter()
            .map(|warning| WasmAttachmentInventoryWarning {
                kind: warning.kind.as_str(),
                message: warning.message.clone(),
            })
            .collect(),
    }
}

pub(crate) fn attachment_inventory_options(
    request: WasmAttachmentInventoryRequest,
) -> Result<AttachmentInventoryOptions, String> {
    let mut options =
        AttachmentInventoryOptions::new(request.base_dir.unwrap_or_else(|| ".".into()));
    if let Some(attach_id_dir) = request.attach_id_dir {
        options = options.attach_id_dir(attach_id_dir);
    }
    if let Some(check_vcs) = request.check_vcs {
        options = options.check_vcs(check_vcs);
    }
    if let Some(check_annex) = request.check_annex {
        options = options.check_annex(check_annex);
    }
    if let Some(policy) = request.archive_delete_policy {
        options = options.archive_delete_policy(parse_archive_delete_policy(&policy)?);
    }
    if let Some(scan_orphans) = request.scan_orphans {
        options = options.scan_orphans(scan_orphans);
    }
    Ok(options)
}

fn attachment_inventory_entry(
    entry: &orgize::ast::AttachmentInventoryEntry,
) -> WasmAttachmentInventoryEntry {
    WasmAttachmentInventoryEntry {
        source: section_source(&entry.source),
        section_title: entry.section_title.clone(),
        kind: attachment_inventory_entry_kind(&entry.kind),
        path: entry.path.clone(),
        absolute_path: entry.absolute_path.clone(),
        exists: entry.exists,
        vcs: WasmAttachmentVcsEvidence {
            status: entry.vcs.status.as_str(),
            annex: WasmAttachmentAnnexEvidence {
                status: entry.vcs.annex.status.as_str(),
                raw: entry.vcs.annex.raw.clone(),
            },
            raw: entry.vcs.raw.clone(),
        },
    }
}

fn attachment_inventory_entry_kind(
    kind: &AttachmentInventoryEntryKind,
) -> WasmAttachmentInventoryEntryKind {
    match kind {
        AttachmentInventoryEntryKind::Directory { source } => WasmAttachmentInventoryEntryKind {
            label: kind.as_str(),
            directory_source: Some(attachment_directory_source(source)),
            link: None,
        },
        AttachmentInventoryEntryKind::Link { link } => WasmAttachmentInventoryEntryKind {
            label: kind.as_str(),
            directory_source: None,
            link: Some(attachment_link(link)),
        },
    }
}

fn parse_archive_delete_policy(value: &str) -> Result<AttachmentArchiveDeletePolicy, String> {
    match value {
        "notConfigured" => Ok(AttachmentArchiveDeletePolicy::NotConfigured),
        "never" => Ok(AttachmentArchiveDeletePolicy::Never),
        "query" => Ok(AttachmentArchiveDeletePolicy::Query),
        "always" => Ok(AttachmentArchiveDeletePolicy::Always),
        other => Err(format!("invalid attachment archive delete policy: {other}")),
    }
}
