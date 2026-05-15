//! JSON DTO projection entrypoints used by wasm-bindgen methods.

use crate::{
    dto_common::to_json,
    dto_document::{
        column_view_records, column_views_response, datetree_entries, datetree_response,
        document_metadata, include_expansion_entries, include_expansion_response, outline_node,
        source_block_records, source_blocks_response,
    },
    dto_index::{
        attachment_records, attachments_response, lint_findings, lint_response,
        section_index_record, section_index_records, section_index_response, sparse_tree_response,
        view_index_response,
    },
    dto_model::{WasmOutlineResponse, WasmSnapshotResponse},
};
use orgize::ast::{Document, IncludeExpansionOptions, ParsedAnnotation, SparseTreeQuery};

pub(crate) fn outline_json(document: &Document<ParsedAnnotation>) -> String {
    to_json(&WasmOutlineResponse {
        schema_version: 1,
        nodes: document.sections.iter().map(outline_node).collect(),
    })
}

pub(crate) fn metadata_json(document: &Document<ParsedAnnotation>) -> String {
    to_json(&document_metadata(document))
}

pub(crate) fn lint_json(document: &Document<ParsedAnnotation>, source: &str) -> String {
    to_json(&lint_response(document, source))
}

pub(crate) fn section_index_json(
    document: &Document<ParsedAnnotation>,
    source_file: Option<&str>,
) -> String {
    let records = section_index_records(document, source_file);
    to_json(&section_index_response(&records))
}

pub(crate) fn sparse_tree_json(
    document: &Document<ParsedAnnotation>,
    source_file: Option<&str>,
    match_expression: Option<&str>,
    text: Option<&str>,
    include_archived: Option<bool>,
) -> Result<String, String> {
    let mut query = SparseTreeQuery::new();
    if let Some(source_file) = source_file {
        query = query.source_file(source_file);
    }
    if let Some(include_archived) = include_archived {
        query = query.include_archived(include_archived);
    }
    if let Some(text) = text {
        query = query.text(text);
    }
    if let Some(match_expression) = match_expression {
        query = query
            .match_expression(match_expression)
            .map_err(|err| err.to_string())?;
    }
    Ok(to_json(&sparse_tree_response(
        &document.sparse_tree_projection(&query),
    )))
}

pub(crate) fn view_index_json(
    document: &Document<ParsedAnnotation>,
    _source_file: Option<&str>,
) -> String {
    to_json(&view_index_response(document))
}

pub(crate) fn attachments_json(
    document: &Document<ParsedAnnotation>,
    source_file: Option<&str>,
) -> String {
    let records = section_index_records(document, source_file);
    to_json(&attachments_response(&records))
}

pub(crate) fn source_blocks_json(document: &Document<ParsedAnnotation>) -> String {
    to_json(&source_blocks_response(document))
}

pub(crate) fn column_views_json(document: &Document<ParsedAnnotation>) -> String {
    to_json(&column_views_response(document))
}

pub(crate) fn include_expansion_json(
    document: &Document<ParsedAnnotation>,
    base_dir: Option<&str>,
) -> String {
    to_json(&include_expansion_response(document, base_dir))
}

pub(crate) fn datetree_json(document: &Document<ParsedAnnotation>) -> String {
    to_json(&datetree_response(document))
}

pub(crate) fn snapshot_json(
    document: &Document<ParsedAnnotation>,
    source: &str,
    source_file: Option<&str>,
) -> String {
    let records = section_index_records(document, source_file);
    let lint = orgize::lint::lint_document(document, source);
    let include_options = IncludeExpansionOptions::default();
    to_json(&WasmSnapshotResponse {
        schema_version: 1,
        metadata: document_metadata(document),
        outline: document.sections.iter().map(outline_node).collect(),
        section_index: records.iter().map(section_index_record).collect(),
        attachments: attachment_records(&records),
        source_blocks: source_block_records(document),
        column_views: column_view_records(document),
        include_expansion: include_expansion_entries(document, &include_options),
        datetree: datetree_entries(document),
        lint: lint_findings(&lint.findings),
    })
}
