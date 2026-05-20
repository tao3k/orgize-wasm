//! JSON DTO projection entrypoints used by wasm-bindgen methods.

use crate::{
    dto_agenda::{agenda_block_view_response, agenda_view_response},
    dto_capture::agent_capture_plan_response,
    dto_clock::{
        clock_issue_findings, clock_issues_response, clock_rollup_records, clock_rollups_response,
        clock_table_plans, clock_table_plans_response,
    },
    dto_common::to_json,
    dto_crypt::{crypt_records, crypt_response},
    dto_document::{
        column_view_records, column_views_response, datetree_entries, datetree_response,
        document_metadata, include_expansion_entries, include_expansion_response, outline_node,
        progress_stats_records, progress_stats_response, source_block_records,
        source_blocks_response, task_blockers_response,
    },
    dto_dynamic_block::{dynamic_block_records, dynamic_blocks_response},
    dto_index::{
        attachment_records, attachments_response, lint_findings, lint_response,
        section_index_record, section_index_records, section_index_response, sparse_tree_response,
        view_index_response,
    },
    dto_memory::memory_response,
    dto_model::{WasmOutlineResponse, WasmSnapshotResponse},
    dto_property_profile::{
        property_profile, property_profile_response, property_profile_with_schema_registry,
        property_profile_with_schema_registry_response, property_schema_registry,
    },
    dto_refile::{refile_plan_response, refile_target_index_response, refile_targets},
    dto_runtime::runtime_metadata_response,
    dto_sdd::{sdd_records, sdd_response},
};
use orgize::ast::{
    AgendaBlockViewQuery, AgendaViewQuery, AgentCaptureRequest, ClockIssueProfile, Document,
    IncludeExpansionOptions, MemoryQuery, ParsedAnnotation, RefilePlanRequest, RefileTargetQuery,
    SparseTreeQuery,
};
use orgize::{
    ast::PropertySchemaRegistry,
    lint::{LintOptions, lint_document_with_options},
};

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
    sparse_tree_json_with_options(
        document,
        source_file,
        match_expression,
        text,
        include_archived,
        false,
    )
}

pub(crate) fn sparse_tree_explain_json(
    document: &Document<ParsedAnnotation>,
    source_file: Option<&str>,
    match_expression: Option<&str>,
    text: Option<&str>,
    include_archived: Option<bool>,
) -> Result<String, String> {
    sparse_tree_json_with_options(
        document,
        source_file,
        match_expression,
        text,
        include_archived,
        true,
    )
}

fn sparse_tree_json_with_options(
    document: &Document<ParsedAnnotation>,
    source_file: Option<&str>,
    match_expression: Option<&str>,
    text: Option<&str>,
    include_archived: Option<bool>,
    explain_skips: bool,
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
    if explain_skips {
        query = query.explain_skips(true);
    }
    Ok(to_json(&sparse_tree_response(
        &document.sparse_tree_projection(&query),
    )))
}

pub(crate) fn agenda_view_json(
    document: &Document<ParsedAnnotation>,
    query: &AgendaViewQuery,
) -> String {
    to_json(&agenda_view_response(document, query))
}

pub(crate) fn agenda_block_json(
    document: &Document<ParsedAnnotation>,
    query: &AgendaBlockViewQuery,
) -> String {
    to_json(&agenda_block_view_response(document, query))
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

pub(crate) fn dynamic_blocks_json(document: &Document<ParsedAnnotation>) -> String {
    to_json(&dynamic_blocks_response(document))
}

pub(crate) fn property_profile_json(document: &Document<ParsedAnnotation>) -> String {
    to_json(&property_profile_response(document))
}

pub(crate) fn property_profile_with_schema_registry_json(
    document: &Document<ParsedAnnotation>,
    request: crate::dto_property_profile_model::WasmPropertySchemaRegistryRequest,
) -> String {
    to_json(&property_profile_with_schema_registry_response(
        document, request,
    ))
}

pub(crate) fn capture_plan_json(request: &AgentCaptureRequest) -> String {
    to_json(&agent_capture_plan_response(request))
}

pub(crate) fn refile_targets_json(
    document: &Document<ParsedAnnotation>,
    query: &RefileTargetQuery,
) -> String {
    to_json(&refile_target_index_response(document, query))
}

pub(crate) fn refile_plan_json(
    document: &Document<ParsedAnnotation>,
    request: &RefilePlanRequest,
) -> String {
    to_json(&refile_plan_response(document, request))
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

pub(crate) fn progress_stats_json(document: &Document<ParsedAnnotation>) -> String {
    to_json(&progress_stats_response(document))
}

pub(crate) fn clock_rollups_json(document: &Document<ParsedAnnotation>) -> String {
    to_json(&clock_rollups_response(document))
}

pub(crate) fn clock_table_plans_json(document: &Document<ParsedAnnotation>) -> String {
    to_json(&clock_table_plans_response(document))
}

pub(crate) fn clock_issues_json(
    document: &Document<ParsedAnnotation>,
    profile: &ClockIssueProfile,
) -> String {
    to_json(&clock_issues_response(document, profile))
}

pub(crate) fn task_blockers_json(document: &Document<ParsedAnnotation>) -> String {
    to_json(&task_blockers_response(document))
}

pub(crate) fn sdd_json(document: &Document<ParsedAnnotation>) -> String {
    to_json(&sdd_response(document))
}

pub(crate) fn crypt_json(document: &Document<ParsedAnnotation>) -> String {
    to_json(&crypt_response(document))
}

pub(crate) fn runtime_metadata_json(document: &Document<ParsedAnnotation>) -> String {
    to_json(&runtime_metadata_response(document))
}

pub(crate) fn memory_json(document: &Document<ParsedAnnotation>, query: &MemoryQuery) -> String {
    to_json(&memory_response(document, query))
}

pub(crate) fn snapshot_json(
    document: &Document<ParsedAnnotation>,
    source: &str,
    source_file: Option<&str>,
) -> String {
    to_json(&snapshot_response(document, source, source_file, None))
}

pub(crate) fn snapshot_with_schema_registry_json(
    document: &Document<ParsedAnnotation>,
    source: &str,
    source_file: Option<&str>,
    request: crate::dto_property_profile_model::WasmPropertySchemaRegistryRequest,
) -> String {
    let registry = property_schema_registry(request);
    to_json(&snapshot_response(
        document,
        source,
        source_file,
        Some(&registry),
    ))
}

fn snapshot_response(
    document: &Document<ParsedAnnotation>,
    source: &str,
    source_file: Option<&str>,
    schema_registry: Option<&PropertySchemaRegistry>,
) -> WasmSnapshotResponse {
    let records = section_index_records(document, source_file);
    let lint = match schema_registry {
        Some(registry) => lint_document_with_options(
            document,
            source,
            &LintOptions {
                property_schema_registry: registry.clone(),
                ..LintOptions::default()
            },
        ),
        None => orgize::lint::lint_document(document, source),
    };
    let include_options = IncludeExpansionOptions::default();
    let mut refile_query = RefileTargetQuery::new();
    if let Some(source_file) = source_file {
        refile_query = refile_query.source_file(source_file);
    }
    WasmSnapshotResponse {
        schema_version: 1,
        metadata: document_metadata(document),
        outline: document.sections.iter().map(outline_node).collect(),
        section_index: records.iter().map(section_index_record).collect(),
        attachments: attachment_records(&records),
        source_blocks: source_block_records(document),
        column_views: column_view_records(document),
        dynamic_blocks: dynamic_block_records(document),
        property_profile: schema_registry.map_or_else(
            || property_profile(document),
            |registry| property_profile_with_schema_registry(document, registry),
        ),
        refile_targets: refile_targets(document, &refile_query),
        include_expansion: include_expansion_entries(document, &include_options),
        datetree: datetree_entries(document),
        progress_stats: progress_stats_records(document),
        clock_rollups: clock_rollup_records(document),
        clock_table_plans: clock_table_plans(document),
        clock_issues: clock_issue_findings(document, &ClockIssueProfile::org_default()),
        sdd: sdd_records(document),
        memory: memory_response(document, &MemoryQuery::new()),
        crypt: crypt_records(document),
        runtime_metadata: runtime_metadata_response(document),
        lint: lint_findings(&lint.findings),
    }
}
