//! Memory WebAssembly DTO projection.

use crate::{
    dto_common::{source_position, todo_state},
    dto_memory_model::{
        WasmAgentMemoryCard, WasmAgentMemoryDecision, WasmMemoryAuthorityReason,
        WasmMemoryEvidence, WasmMemoryEvidenceKind, WasmMemoryFacet, WasmMemoryLink,
        WasmMemoryProperty, WasmMemoryRecord, WasmMemoryResponse, WasmMemoryStats,
    },
    dto_shared_model::WasmSourceRange,
};
use orgize::ast::{
    AgentMemoryCard, AgentMemoryDecision, AgentMemoryQuery, AgentMemorySeverity, Document,
    MemoryAuthorityKind, MemoryAuthorityReason, MemoryEvidence, MemoryEvidenceKind,
    MemoryLifecycleKind, MemoryLink, MemoryProperty, MemoryQuery, MemoryRecord, MemoryRecordState,
    MemorySource, ParsedAnnotation, TimestampKind,
};
use std::collections::BTreeMap;

pub(crate) fn memory_response(
    document: &Document<ParsedAnnotation>,
    query: &MemoryQuery,
) -> WasmMemoryResponse {
    let records = document.memory_records(query);
    let cards = document
        .agent_memory_snapshot(&AgentMemoryQuery::new(query.clone()))
        .cards;
    let stats = memory_stats(&records, &cards);
    WasmMemoryResponse {
        schema_version: 1,
        stats,
        evidence_kinds: evidence_facets(&records),
        authority_kinds: authority_facets(&cards),
        records: records.iter().map(memory_record).collect(),
        cards: cards.iter().map(agent_memory_card).collect(),
    }
}

fn memory_stats(records: &[MemoryRecord], cards: &[AgentMemoryCard]) -> WasmMemoryStats {
    let mut stats = WasmMemoryStats {
        total_records: records.len(),
        cards: cards.len(),
        ..WasmMemoryStats::default()
    };
    for record in records {
        match record.state {
            MemoryRecordState::Current => stats.current_records += 1,
            MemoryRecordState::Background => stats.background_records += 1,
            MemoryRecordState::Closed => stats.closed_records += 1,
            MemoryRecordState::Archived => stats.archived_records += 1,
        }
        stats.evidence += record.evidence.len();
        stats.properties += record.properties.len();
        stats.links += record.links.len();
    }
    for card in cards {
        match card.decision.severity() {
            AgentMemorySeverity::Action => stats.action_cards += 1,
            AgentMemorySeverity::Suppressed => stats.suppressed_cards += 1,
            AgentMemorySeverity::Info => stats.info_cards += 1,
        }
        stats.authority_reasons += card.authority.len();
    }
    stats
}

fn memory_record(record: &MemoryRecord) -> WasmMemoryRecord {
    WasmMemoryRecord {
        source: memory_source(&record.source),
        state: memory_state(record.state),
        level: record.level,
        title: memory_title(
            &record.title,
            record.todo.as_ref().map(|todo| todo.name.as_str()),
        ),
        todo: record.todo.as_ref().map(|todo| todo.name.clone()),
        todo_state: record.todo.as_ref().map(todo_state),
        tags: record.tags.clone(),
        effective_tags: record.effective_tags.clone(),
        anchor: record.anchor.clone(),
        properties: record.properties.iter().map(memory_property).collect(),
        evidence: record.evidence.iter().map(memory_evidence).collect(),
        links: record.links.iter().map(memory_link).collect(),
    }
}

fn agent_memory_card(card: &AgentMemoryCard) -> WasmAgentMemoryCard {
    WasmAgentMemoryCard {
        source: memory_source(&card.source),
        decision: agent_memory_decision(&card.decision),
        authority: card.authority.iter().map(authority_reason).collect(),
        title: memory_title(
            &card.title,
            card.todo.as_ref().map(|todo| todo.name.as_str()),
        ),
        todo: card.todo.as_ref().map(|todo| todo.name.clone()),
        todo_state: card.todo.as_ref().map(todo_state),
        tags: card.tags.clone(),
        effective_tags: card.effective_tags.clone(),
        anchor: card.anchor.clone(),
        evidence: card.evidence.iter().map(memory_evidence).collect(),
        links: card.links.iter().map(memory_link).collect(),
    }
}

fn memory_property(property: &MemoryProperty) -> WasmMemoryProperty {
    WasmMemoryProperty {
        source: memory_source(&property.source),
        key: property.key.clone(),
        value: property.value.clone(),
    }
}

fn memory_evidence(evidence: &MemoryEvidence) -> WasmMemoryEvidence {
    WasmMemoryEvidence {
        source: memory_source(&evidence.source),
        kind: evidence_kind(&evidence.kind),
        value: evidence.value.clone(),
    }
}

fn memory_link(link: &MemoryLink) -> WasmMemoryLink {
    WasmMemoryLink {
        source: memory_source(&link.source),
        path: link.path.clone(),
        description: link.description.clone(),
    }
}

fn authority_reason(reason: &MemoryAuthorityReason) -> WasmMemoryAuthorityReason {
    let (kind, label) = authority_kind(reason.kind);
    WasmMemoryAuthorityReason {
        kind,
        label,
        message: reason.message.clone(),
    }
}

fn memory_source(source: &MemorySource) -> WasmSourceRange {
    WasmSourceRange {
        start: source_position(source.start),
        end: source_position(source.end),
        range_start: source.range_start,
        range_end: source.range_end,
    }
}

fn memory_state(state: MemoryRecordState) -> &'static str {
    match state {
        MemoryRecordState::Current => "current",
        MemoryRecordState::Closed => "closed",
        MemoryRecordState::Archived => "archived",
        MemoryRecordState::Background => "background",
    }
}

fn memory_title(title: &str, todo: Option<&str>) -> String {
    let trimmed = title.trim();
    let Some(todo) = todo else {
        return trimmed.to_string();
    };
    trimmed
        .strip_prefix(todo)
        .filter(|rest| rest.chars().next().is_some_and(char::is_whitespace))
        .map(str::trim_start)
        .filter(|rest| !rest.is_empty())
        .unwrap_or(trimmed)
        .to_string()
}

fn agent_memory_decision(decision: &AgentMemoryDecision) -> WasmAgentMemoryDecision {
    let (kind, title, next_action) = match decision {
        AgentMemoryDecision::Current => (
            "current",
            "Current memory",
            "Allow this fact into current decisions unless a caller profile narrows scope.",
        ),
        AgentMemoryDecision::Closed => (
            "closed",
            "Closed memory",
            "Keep as historical evidence; do not promote as current without newer evidence.",
        ),
        AgentMemoryDecision::Archived => (
            "archived",
            "Archived memory",
            "Keep as archived evidence; exclude from active decisions by default.",
        ),
        AgentMemoryDecision::Background => (
            "background",
            "Background memory",
            "Use as context only; do not let it drive action by itself.",
        ),
    };
    WasmAgentMemoryDecision {
        code: decision.code(),
        kind,
        severity: agent_memory_severity(decision.severity()),
        title,
        next_action,
    }
}

fn agent_memory_severity(severity: AgentMemorySeverity) -> &'static str {
    match severity {
        AgentMemorySeverity::Action => "action",
        AgentMemorySeverity::Suppressed => "suppressed",
        AgentMemorySeverity::Info => "info",
    }
}

fn authority_kind(kind: MemoryAuthorityKind) -> (&'static str, &'static str) {
    match kind {
        MemoryAuthorityKind::Current => ("current", "Current"),
        MemoryAuthorityKind::Closed => ("closed", "Closed"),
        MemoryAuthorityKind::Archived => ("archived", "Archived"),
        MemoryAuthorityKind::Background => ("background", "Background"),
        MemoryAuthorityKind::Identity => ("identity", "Identity"),
        MemoryAuthorityKind::Temporal => ("temporal", "Temporal"),
        MemoryAuthorityKind::Lifecycle => ("lifecycle", "Lifecycle"),
        MemoryAuthorityKind::Attachment => ("attachment", "Attachment"),
        MemoryAuthorityKind::Habit => ("habit", "Habit"),
        MemoryAuthorityKind::Repeat => ("repeat", "Repeat"),
        MemoryAuthorityKind::StaleCandidate => ("staleCandidate", "Stale candidate"),
        MemoryAuthorityKind::SupersededCandidate => ("supersededCandidate", "Superseded candidate"),
    }
}

fn evidence_kind(kind: &MemoryEvidenceKind) -> WasmMemoryEvidenceKind {
    let (code, label, family, detail) = match kind {
        MemoryEvidenceKind::TodoState => ("todoState", "TODO state".to_string(), "lifecycle", None),
        MemoryEvidenceKind::ArchiveTag => {
            ("archiveTag", "ARCHIVE tag".to_string(), "archive", None)
        }
        MemoryEvidenceKind::ArchiveLocation => (
            "archiveLocation",
            "archive location".to_string(),
            "archive",
            None,
        ),
        MemoryEvidenceKind::ArchiveProperty => (
            "archiveProperty",
            "ARCHIVE property".to_string(),
            "archive",
            None,
        ),
        MemoryEvidenceKind::AttachmentTag => (
            "attachmentTag",
            "ATTACH tag".to_string(),
            "attachment",
            None,
        ),
        MemoryEvidenceKind::AttachmentDirectory => (
            "attachmentDirectory",
            "attachment directory".to_string(),
            "attachment",
            None,
        ),
        MemoryEvidenceKind::HabitStyle => ("habitStyle", "habit style".to_string(), "habit", None),
        MemoryEvidenceKind::HabitLastRepeat => (
            "habitLastRepeat",
            "habit last repeat".to_string(),
            "habit",
            None,
        ),
        MemoryEvidenceKind::HabitRepeater => {
            ("habitRepeater", "habit repeater".to_string(), "habit", None)
        }
        MemoryEvidenceKind::Identity { key } => (
            "identity",
            format!("identity {key}"),
            "identity",
            Some(key.clone()),
        ),
        MemoryEvidenceKind::Property { key } => (
            "property",
            format!("property {key}"),
            "property",
            Some(key.clone()),
        ),
        MemoryEvidenceKind::Scheduled => ("scheduled", "SCHEDULED".to_string(), "planning", None),
        MemoryEvidenceKind::Deadline => ("deadline", "DEADLINE".to_string(), "planning", None),
        MemoryEvidenceKind::Closed => ("closed", "CLOSED".to_string(), "planning", None),
        MemoryEvidenceKind::Timestamp { kind } => (
            "timestamp",
            timestamp_kind_label(*kind).to_string(),
            "timestamp",
            Some(timestamp_kind_code(*kind).to_string()),
        ),
        MemoryEvidenceKind::Logbook => ("logbook", "LOGBOOK".to_string(), "lifecycle", None),
        MemoryEvidenceKind::Drawer { name } => (
            "drawer",
            format!("drawer {name}"),
            "drawer",
            Some(name.clone()),
        ),
        MemoryEvidenceKind::Clock => ("clock", "CLOCK".to_string(), "clock", None),
        MemoryEvidenceKind::Link => ("link", "link".to_string(), "link", None),
        MemoryEvidenceKind::AttachmentLink => (
            "attachmentLink",
            "attachment link".to_string(),
            "attachment",
            None,
        ),
        MemoryEvidenceKind::Lifecycle(kind) => (
            "lifecycle",
            format!("lifecycle {}", lifecycle_kind_label(*kind)),
            "lifecycle",
            Some(lifecycle_kind_code(*kind).to_string()),
        ),
    };
    WasmMemoryEvidenceKind {
        code,
        label,
        family,
        detail,
    }
}

fn timestamp_kind_code(kind: TimestampKind) -> &'static str {
    match kind {
        TimestampKind::Active => "active",
        TimestampKind::Inactive => "inactive",
        TimestampKind::Diary => "diary",
    }
}

fn timestamp_kind_label(kind: TimestampKind) -> &'static str {
    match kind {
        TimestampKind::Active => "active timestamp",
        TimestampKind::Inactive => "inactive timestamp",
        TimestampKind::Diary => "diary timestamp",
    }
}

fn lifecycle_kind_code(kind: MemoryLifecycleKind) -> &'static str {
    match kind {
        MemoryLifecycleKind::StateChange => "stateChange",
        MemoryLifecycleKind::Note => "note",
        MemoryLifecycleKind::Refile => "refile",
        MemoryLifecycleKind::Reschedule => "reschedule",
        MemoryLifecycleKind::Redeadline => "redeadline",
        MemoryLifecycleKind::Clock => "clock",
    }
}

fn lifecycle_kind_label(kind: MemoryLifecycleKind) -> &'static str {
    match kind {
        MemoryLifecycleKind::StateChange => "state change",
        MemoryLifecycleKind::Note => "note",
        MemoryLifecycleKind::Refile => "refile",
        MemoryLifecycleKind::Reschedule => "reschedule",
        MemoryLifecycleKind::Redeadline => "redeadline",
        MemoryLifecycleKind::Clock => "clock",
    }
}

fn evidence_facets(records: &[MemoryRecord]) -> Vec<WasmMemoryFacet> {
    let mut counts: BTreeMap<(String, String), usize> = BTreeMap::new();
    for evidence in records.iter().flat_map(|record| &record.evidence) {
        let kind = evidence_kind(&evidence.kind);
        *counts
            .entry((kind.code.to_string(), kind.label))
            .or_default() += 1;
    }
    counts
        .into_iter()
        .map(|((code, label), count)| WasmMemoryFacet { code, label, count })
        .collect()
}

fn authority_facets(cards: &[AgentMemoryCard]) -> Vec<WasmMemoryFacet> {
    let mut counts: BTreeMap<(String, String), usize> = BTreeMap::new();
    for reason in cards.iter().flat_map(|card| &card.authority) {
        let (kind, label) = authority_kind(reason.kind);
        *counts
            .entry((kind.to_string(), label.to_string()))
            .or_default() += 1;
    }
    counts
        .into_iter()
        .map(|((code, label), count)| WasmMemoryFacet { code, label, count })
        .collect()
}
