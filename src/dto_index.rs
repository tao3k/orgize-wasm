//! Section-index, attachment, and lint DTO projection.

use crate::{
    dto_common::{
        attachment_directory, attachment_link, file_link, link_search, planning, priority,
        section_source, target_kind, todo_state,
    },
    dto_model::{
        WasmArchive, WasmAttachmentRecord, WasmAttachmentState, WasmAttachmentsResponse,
        WasmLifecycleRecord, WasmLink, WasmLintFinding, WasmLintResponse, WasmProperty,
        WasmSectionIndexRecord, WasmSectionIndexResponse, WasmSparseTreeCard, WasmSparseTreeMatch,
        WasmSparseTreeReceipt, WasmSparseTreeResponse, WasmSparseTreeSkip, WasmTarget,
        WasmTextSlice, WasmViewIndexRecord, WasmViewIndexResponse, WasmViewPlanning,
        WasmViewProperty,
    },
    dto_shared_model::{WasmSourcePosition, WasmSourceRange},
};
use orgize::{
    ast::{
        Document, Element, ElementData, ParsedAnnotation, Section, SectionIndexRecord,
        SparseTreeProjection,
    },
    lint::{lint_document, LintFinding, LintSeverity},
};
use std::collections::HashSet;

pub(crate) fn section_index_records(
    document: &Document<ParsedAnnotation>,
    source_file: Option<&str>,
) -> Vec<SectionIndexRecord> {
    match source_file {
        Some(source_file) => document.section_index_records_for_file(source_file),
        None => document.section_index_records(),
    }
}

pub(crate) fn section_index_response(records: &[SectionIndexRecord]) -> WasmSectionIndexResponse {
    WasmSectionIndexResponse {
        schema_version: 1,
        records: records.iter().map(section_index_record).collect(),
    }
}

pub(crate) fn sparse_tree_response(projection: &SparseTreeProjection) -> WasmSparseTreeResponse {
    WasmSparseTreeResponse {
        schema_version: 1,
        total_candidates: projection.total_candidates,
        cards: projection.cards.iter().map(sparse_tree_card).collect(),
        skipped: projection.skipped.iter().map(sparse_tree_skip).collect(),
    }
}

pub(crate) fn view_index_response(document: &Document<ParsedAnnotation>) -> WasmViewIndexResponse {
    let mut records = Vec::new();
    let mut outline_path = Vec::new();
    for section in &document.sections {
        collect_view_index_records(section, &mut outline_path, &mut records);
    }
    WasmViewIndexResponse {
        schema_version: 1,
        records,
    }
}

pub(crate) fn attachments_response(records: &[SectionIndexRecord]) -> WasmAttachmentsResponse {
    WasmAttachmentsResponse {
        schema_version: 1,
        records: attachment_records(records),
    }
}

pub(crate) fn lint_response(
    document: &Document<ParsedAnnotation>,
    source: &str,
) -> WasmLintResponse {
    let report = lint_document(document, source);
    WasmLintResponse {
        schema_version: 1,
        findings: lint_findings(&report.findings),
    }
}

pub(crate) fn lint_findings(findings: &[LintFinding]) -> Vec<WasmLintFinding> {
    findings
        .iter()
        .map(|finding| WasmLintFinding {
            code: finding.code,
            severity: match finding.severity {
                LintSeverity::Error => "error",
                LintSeverity::Warning => "warning",
            },
            message: finding.message.clone(),
            source: WasmSourceRange {
                start: WasmSourcePosition {
                    line: finding.location.start.line,
                    column: finding.location.start.column,
                },
                end: WasmSourcePosition {
                    line: finding.location.end.line,
                    column: finding.location.end.column,
                },
                range_start: finding.location.range_start as u32,
                range_end: finding.location.range_end as u32,
            },
        })
        .collect()
}

pub(crate) fn section_index_record(record: &SectionIndexRecord) -> WasmSectionIndexRecord {
    WasmSectionIndexRecord {
        source: section_source(&record.source),
        outline_path: record.outline_path.clone(),
        level: record.level,
        title: record.title.clone(),
        body: record
            .body
            .iter()
            .map(|slice| WasmTextSlice {
                source: section_source(&slice.source),
                text: slice.text.clone(),
            })
            .collect(),
        todo: record.todo.as_ref().map(|todo| todo.name.clone()),
        todo_state: record.todo.as_ref().map(todo_state),
        priority: priority(&record.priority),
        category: record
            .category
            .as_ref()
            .map(|category| category.as_str().to_string()),
        tags: record.tags.clone(),
        effective_tags: record.effective_tags.clone(),
        properties: record
            .properties
            .iter()
            .map(|property| WasmProperty {
                source: section_source(&property.source),
                key: property.key.clone(),
                value: property.value.clone(),
            })
            .collect(),
        effective_properties: record
            .effective_properties
            .iter()
            .map(|property| WasmProperty {
                source: section_source(&property.source),
                key: property.key.clone(),
                value: property.value.clone(),
            })
            .collect(),
        special_properties: record
            .special_properties
            .iter()
            .map(|property| WasmProperty {
                source: section_source(&property.source),
                key: property.name.clone(),
                value: property.value.clone(),
            })
            .collect(),
        planning: planning(&record.planning),
        is_comment: record.is_comment,
        archive: WasmArchive {
            archived: record.archive.archived,
            has_archive_tag: record.archive.has_archive_tag,
            location: record.archive.location.clone(),
        },
        attachment: WasmAttachmentState {
            has_attach_tag: record.attachment.has_attach_tag,
            directory: record
                .attachment
                .directory
                .as_ref()
                .map(|directory| attachment_directory(&directory.source, &directory.path)),
        },
        links: record
            .links
            .iter()
            .map(|link| WasmLink {
                source: section_source(&link.source),
                path: link.path.clone(),
                description: link.description.clone(),
                search: link.search.as_ref().map(link_search),
                attachment: link.attachment.as_ref().map(attachment_link),
                file: link.file.as_ref().map(file_link),
            })
            .collect(),
        targets: record
            .targets
            .iter()
            .map(|target| WasmTarget {
                source: section_source(&target.source),
                kind: target_kind(target.kind),
                key: target.key.clone(),
                value: target.value.clone(),
            })
            .collect(),
        lifecycle: record
            .lifecycle
            .iter()
            .map(|record| WasmLifecycleRecord {
                source: section_source(&record.source),
                kind: format!("{:?}", record.kind),
                raw: record.raw.clone(),
            })
            .collect(),
    }
}

fn sparse_tree_card(card: &orgize::ast::SparseTreeCard) -> WasmSparseTreeCard {
    WasmSparseTreeCard {
        source: section_source(&card.source),
        outline_path: card.outline_path.clone(),
        level: card.level,
        title: card.title.clone(),
        matches: card
            .matches
            .iter()
            .map(|matched| WasmSparseTreeMatch {
                source: section_source(&matched.source),
                kind: matched.kind.as_str(),
                key: matched.key.clone(),
                value: matched.value.clone(),
            })
            .collect(),
        receipts: card.receipts.iter().map(sparse_tree_receipt).collect(),
        preview: card.preview.clone(),
        todo: card.todo.as_ref().map(|todo| todo.name.clone()),
        todo_state: card.todo.as_ref().map(todo_state),
        priority: priority(&card.priority),
        category: card
            .category
            .as_ref()
            .map(|category| category.as_str().to_string()),
        tags: card.tags.clone(),
        effective_tags: card.effective_tags.clone(),
        properties: card
            .properties
            .iter()
            .map(|property| WasmProperty {
                source: section_source(&property.source),
                key: property.key.clone(),
                value: property.value.clone(),
            })
            .collect(),
        special_properties: card
            .special_properties
            .iter()
            .map(|property| WasmProperty {
                source: section_source(&property.source),
                key: property.name.clone(),
                value: property.value.clone(),
            })
            .collect(),
        planning: planning(&card.planning),
        archive: WasmArchive {
            archived: card.archive.archived,
            has_archive_tag: card.archive.has_archive_tag,
            location: card.archive.location.clone(),
        },
        attachment: WasmAttachmentState {
            has_attach_tag: card.attachment.has_attach_tag,
            directory: card
                .attachment
                .directory
                .as_ref()
                .map(|directory| attachment_directory(&directory.source, &directory.path)),
        },
        links: card
            .links
            .iter()
            .map(|link| WasmLink {
                source: section_source(&link.source),
                path: link.path.clone(),
                description: link.description.clone(),
                search: link.search.as_ref().map(link_search),
                attachment: link.attachment.as_ref().map(attachment_link),
                file: link.file.as_ref().map(file_link),
            })
            .collect(),
        targets: card
            .targets
            .iter()
            .map(|target| WasmTarget {
                source: section_source(&target.source),
                kind: target_kind(target.kind),
                key: target.key.clone(),
                value: target.value.clone(),
            })
            .collect(),
        lifecycle: card
            .lifecycle
            .iter()
            .map(|record| WasmLifecycleRecord {
                source: section_source(&record.source),
                kind: format!("{:?}", record.kind),
                raw: record.raw.clone(),
            })
            .collect(),
    }
}

fn sparse_tree_skip(skip: &orgize::ast::SparseTreeSkip) -> WasmSparseTreeSkip {
    WasmSparseTreeSkip {
        source: section_source(&skip.source),
        outline_path: skip.outline_path.clone(),
        level: skip.level,
        title: skip.title.clone(),
        reason: skip.reason.as_str(),
        receipts: skip.receipts.iter().map(sparse_tree_receipt).collect(),
    }
}

fn sparse_tree_receipt(receipt: &orgize::ast::SparseTreeReceipt) -> WasmSparseTreeReceipt {
    WasmSparseTreeReceipt {
        kind: receipt.kind.as_str(),
        message: receipt.message.clone(),
    }
}

fn collect_view_index_records(
    section: &Section<ParsedAnnotation>,
    outline_path: &mut Vec<String>,
    records: &mut Vec<WasmViewIndexRecord>,
) {
    let title = section.raw_title.trim_end().to_string();
    outline_path.push(title.clone());
    if is_view_index_section(section) {
        records.push(view_index_record(section, outline_path, title));
    }
    for subsection in &section.subsections {
        collect_view_index_records(subsection, outline_path, records);
    }
    outline_path.pop();
}

fn view_index_record(
    section: &Section<ParsedAnnotation>,
    outline_path: &[String],
    title: String,
) -> WasmViewIndexRecord {
    WasmViewIndexRecord {
        range_start: section.ann.range.start().into(),
        outline: outline_path.join(" / "),
        level: section.level,
        title,
        body_preview: body_preview(&section.children),
        todo: section.todo.as_ref().map(|todo| todo.name.clone()),
        todo_state: section.todo.as_ref().map(todo_state),
        effective_tags: view_tags(&section.effective_tags),
        properties: section
            .properties
            .iter()
            .take(4)
            .map(|property| WasmViewProperty {
                key: property.key.clone(),
                value: property.value.clone(),
            })
            .collect(),
        planning: WasmViewPlanning {
            deadline: section
                .planning
                .deadline
                .as_ref()
                .map(|value| value.raw.clone()),
            scheduled: section
                .planning
                .scheduled
                .as_ref()
                .map(|value| value.raw.clone()),
            closed: section
                .planning
                .closed
                .as_ref()
                .map(|value| value.raw.clone()),
        },
    }
}

fn is_view_index_section(section: &Section<ParsedAnnotation>) -> bool {
    section.planning.scheduled.is_some()
        || section.planning.deadline.is_some()
        || section.planning.closed.is_some()
        || section
            .effective_tags
            .iter()
            .any(|tag| matches!(tag.as_str(), "blog" | "record" | "agenda"))
}

fn view_tags(tags: &[String]) -> Vec<String> {
    let mut seen = HashSet::new();
    tags.iter()
        .filter(|tag| seen.insert(tag.as_str()))
        .cloned()
        .collect()
}

const VIEW_BODY_PREVIEW_CHARS: usize = 260;

fn body_preview(elements: &[Element<ParsedAnnotation>]) -> String {
    let mut preview = String::new();
    for element in elements
        .iter()
        .filter(|element| !matches!(element.data, ElementData::PropertyDrawer(_)))
    {
        if preview.chars().count() >= VIEW_BODY_PREVIEW_CHARS {
            break;
        }
        let text = element.ann.raw.trim_start();
        if text.is_empty() {
            continue;
        }
        if !preview.is_empty() {
            preview.push('\n');
        }
        push_preview_chars(&mut preview, text);
    }
    while preview.ends_with(char::is_whitespace) {
        preview.pop();
    }
    preview
}

fn push_preview_chars(preview: &mut String, text: &str) {
    let remaining = VIEW_BODY_PREVIEW_CHARS.saturating_sub(preview.chars().count());
    for character in text.chars().take(remaining) {
        preview.push(character);
    }
}

pub(crate) fn attachment_records(records: &[SectionIndexRecord]) -> Vec<WasmAttachmentRecord> {
    records
        .iter()
        .filter_map(|record| {
            let links: Vec<_> = record
                .links
                .iter()
                .filter_map(|link| link.attachment.as_ref().map(attachment_link))
                .collect();
            if !record.attachment.has_attach_tag
                && record.attachment.directory.is_none()
                && links.is_empty()
            {
                return None;
            }
            Some(WasmAttachmentRecord {
                source: section_source(&record.source),
                section_title: record.title.clone(),
                outline_path: record.outline_path.clone(),
                directory: record
                    .attachment
                    .directory
                    .as_ref()
                    .map(|directory| attachment_directory(&directory.source, &directory.path)),
                links,
            })
        })
        .collect()
}
