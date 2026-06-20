//! Shared helpers for WebAssembly DTO projections.

use crate::dto_model::{
    WasmAttachmentDirectory, WasmAttachmentDirectorySource, WasmAttachmentLink,
    WasmAttachmentLinkSearch, WasmFileLink, WasmLinkSearch, WasmPlanning, WasmPriority,
    WasmPriorityProfile, WasmProperty, WasmTimestamp, WasmTimestampMoment,
};
use crate::dto_shared_model::{WasmOrgDuration, WasmSourcePosition, WasmSourceRange};
use orgize::ast::{
    AttachmentDirectorySource, AttachmentIdPathLayout, AttachmentLink, AttachmentLinkSearch,
    AttachmentLinkSearchKind, FileLink, FileLinkPathKind, LinkSearch, LinkSearchKind,
    ParsedAnnotation, Planning, PriorityProfile, SectionIndexSource, SourcePosition, TargetKind,
    Timestamp, TimestampKind, TodoKeyword, TodoState,
};
use serde::Serialize;

pub(crate) fn to_json<T: Serialize>(value: &T) -> String {
    serde_json::to_string(value).expect("wasm DTO serialization should not fail")
}

pub(crate) fn annotation_source(annotation: &ParsedAnnotation) -> WasmSourceRange {
    WasmSourceRange {
        start: source_position(annotation.start),
        end: source_position(annotation.end),
        range_start: annotation.range.start().into(),
        range_end: annotation.range.end().into(),
    }
}

pub(crate) fn section_source(source: &SectionIndexSource) -> WasmSourceRange {
    WasmSourceRange {
        start: source_position(source.start),
        end: source_position(source.end),
        range_start: source.range_start,
        range_end: source.range_end,
    }
}

pub(crate) fn source_position(position: SourcePosition) -> WasmSourcePosition {
    WasmSourcePosition {
        line: position.line,
        column: position.column,
    }
}

pub(crate) fn org_duration(duration: &orgize::ast::OrgDuration) -> WasmOrgDuration {
    WasmOrgDuration {
        raw: duration.raw.clone(),
        total_seconds: duration.total_seconds,
        total_minutes: duration.total_minutes(),
    }
}

pub(crate) fn semantic_property(
    property: &orgize::ast::Property<ParsedAnnotation>,
) -> WasmProperty {
    WasmProperty {
        source: annotation_source(&property.ann),
        key: property.key.clone(),
        value: property.value.clone(),
    }
}

pub(crate) fn priority(priority: &orgize::ast::Priority) -> WasmPriority {
    let profile = PriorityProfile::org_default();
    WasmPriority {
        raw: priority.raw_cookie().map(str::to_string),
        effective: priority.effective_text(),
        is_default: priority.is_default(),
        score: priority.score_with_profile(&profile),
        range_status: priority.range_status_with_profile(&profile).as_str(),
        profile: priority_profile(&profile),
    }
}

fn priority_profile(profile: &PriorityProfile) -> WasmPriorityProfile {
    WasmPriorityProfile {
        highest: profile.highest().to_normalized_string(),
        lowest: profile.lowest().to_normalized_string(),
        default: profile.default_priority().to_normalized_string(),
    }
}

pub(crate) fn planning(planning: &Planning) -> WasmPlanning {
    WasmPlanning {
        deadline: planning.deadline.as_ref().map(timestamp),
        scheduled: planning.scheduled.as_ref().map(timestamp),
        closed: planning.closed.as_ref().map(timestamp),
    }
}

fn timestamp(timestamp: &Timestamp) -> WasmTimestamp {
    WasmTimestamp {
        kind: timestamp_kind(timestamp.kind),
        raw: timestamp.raw.clone(),
        is_range: timestamp.is_range,
        start: timestamp.start.as_ref().map(timestamp_moment),
        end: timestamp.end.as_ref().map(timestamp_moment),
    }
}

fn timestamp_moment(moment: &orgize::ast::TimestampMoment) -> WasmTimestampMoment {
    WasmTimestampMoment {
        year: moment.year,
        month: moment.month,
        day: moment.day,
        day_name: moment.day_name.clone(),
        hour: moment.hour,
        minute: moment.minute,
    }
}

pub(crate) fn attachment_directory(
    source: &AttachmentDirectorySource,
    path: &str,
) -> WasmAttachmentDirectory {
    WasmAttachmentDirectory {
        source: attachment_directory_source(source),
        path: path.to_string(),
    }
}

pub(crate) fn link_search(search: &LinkSearch) -> WasmLinkSearch {
    WasmLinkSearch {
        raw: search.raw.clone(),
        kind: link_search_kind(search.kind),
        normalized: search.normalized.clone(),
    }
}

pub(crate) fn attachment_link(link: &AttachmentLink) -> WasmAttachmentLink {
    WasmAttachmentLink {
        path: link.path.clone(),
        search: link.search.as_ref().map(attachment_link_search),
    }
}

fn attachment_link_search(search: &AttachmentLinkSearch) -> WasmAttachmentLinkSearch {
    WasmAttachmentLinkSearch {
        raw: search.raw.clone(),
        kind: attachment_link_search_kind(search.kind),
    }
}

pub(crate) fn file_link(link: &FileLink) -> WasmFileLink {
    WasmFileLink {
        protocol: link.protocol.clone(),
        path: link.path.clone(),
        path_kind: file_link_path_kind(link.path_kind),
        search: link.search.as_ref().map(link_search),
    }
}

pub(crate) fn todo_state(todo: &TodoKeyword) -> &'static str {
    match todo.state {
        TodoState::Todo => "todo",
        TodoState::Done => "done",
    }
}

pub(crate) fn target_kind(kind: TargetKind) -> &'static str {
    match kind {
        TargetKind::Headline => "headline",
        TargetKind::CustomId => "customId",
        TargetKind::Id => "id",
        TargetKind::Target => "target",
        TargetKind::RadioTarget => "radioTarget",
        TargetKind::FootnoteDefinition => "footnoteDefinition",
        TargetKind::CodeRef => "codeRef",
    }
}

fn timestamp_kind(kind: TimestampKind) -> &'static str {
    match kind {
        TimestampKind::Active => "active",
        TimestampKind::Inactive => "inactive",
        TimestampKind::Diary => "diary",
    }
}

fn link_search_kind(kind: LinkSearchKind) -> &'static str {
    match kind {
        LinkSearchKind::Headline => "headline",
        LinkSearchKind::LineNumber => "lineNumber",
        LinkSearchKind::CustomId => "customId",
        LinkSearchKind::Regexp => "regexp",
        LinkSearchKind::Text => "text",
    }
}

fn attachment_link_search_kind(kind: AttachmentLinkSearchKind) -> &'static str {
    match kind {
        AttachmentLinkSearchKind::Headline => "headline",
        AttachmentLinkSearchKind::LineNumber => "lineNumber",
        AttachmentLinkSearchKind::CustomId => "customId",
        AttachmentLinkSearchKind::Regexp => "regexp",
        AttachmentLinkSearchKind::Text => "text",
    }
}

fn file_link_path_kind(kind: FileLinkPathKind) -> &'static str {
    match kind {
        FileLinkPathKind::Empty => "empty",
        FileLinkPathKind::Absolute => "absolute",
        FileLinkPathKind::HomeRelative => "homeRelative",
        FileLinkPathKind::Relative => "relative",
        FileLinkPathKind::Remote => "remote",
    }
}

pub(crate) fn attachment_directory_source(
    source: &AttachmentDirectorySource,
) -> WasmAttachmentDirectorySource {
    match source {
        AttachmentDirectorySource::DirProperty => WasmAttachmentDirectorySource {
            kind: "dirProperty",
            id: None,
            layout: None,
        },
        AttachmentDirectorySource::AttachDirProperty => WasmAttachmentDirectorySource {
            kind: "attachDirProperty",
            id: None,
            layout: None,
        },
        AttachmentDirectorySource::IdDerived { id, layout } => WasmAttachmentDirectorySource {
            kind: "idDerived",
            id: Some(id.clone()),
            layout: Some(attachment_id_path_layout(*layout)),
        },
    }
}

fn attachment_id_path_layout(layout: AttachmentIdPathLayout) -> &'static str {
    match layout {
        AttachmentIdPathLayout::Uuid => "uuid",
        AttachmentIdPathLayout::Timestamp => "timestamp",
        AttachmentIdPathLayout::Fallback => "fallback",
    }
}
