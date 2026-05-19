//! SDD WebAssembly DTO projection.

use crate::{
    dto_common::{section_source, todo_state},
    dto_sdd_model::{WasmSddNodeRecord, WasmSddParentRef, WasmSddResponse},
};
use orgize::ast::{Document, ParsedAnnotation, SddNodeRecord, SddParentRef};

pub(crate) fn sdd_response(document: &Document<ParsedAnnotation>) -> WasmSddResponse {
    WasmSddResponse {
        schema_version: 1,
        records: sdd_records(document),
    }
}

pub(crate) fn sdd_records(document: &Document<ParsedAnnotation>) -> Vec<WasmSddNodeRecord> {
    document
        .sdd_node_records()
        .iter()
        .map(sdd_node_record)
        .collect()
}

fn sdd_node_record(record: &SddNodeRecord) -> WasmSddNodeRecord {
    WasmSddNodeRecord {
        source: section_source(&record.source),
        outline_path: record.outline_path.clone(),
        level: record.level,
        title: record.title.clone(),
        kind: record.kind.as_str().to_string(),
        kind_known: record.kind.is_known(),
        id: record.id.clone(),
        parent: record.parent.as_ref().map(sdd_parent_ref),
        capability: record.capability.clone(),
        viewpoint: record.viewpoint.clone(),
        concern: record.concern.clone(),
        quality: record.quality.clone(),
        rationale: record.rationale.clone(),
        slug: record.slug.clone(),
        status: record
            .status
            .as_ref()
            .map(|status| status.as_str().to_string()),
        todo: record.todo.as_ref().map(|todo| todo.name.clone()),
        todo_state: record.todo.as_ref().map(todo_state),
        tags: record.tags.clone(),
    }
}

fn sdd_parent_ref(parent: &SddParentRef) -> WasmSddParentRef {
    WasmSddParentRef {
        raw: parent.raw.clone(),
        target_id: parent.target_id.clone(),
        label: parent.label.clone(),
    }
}
