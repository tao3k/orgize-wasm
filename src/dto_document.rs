//! Document-level WebAssembly DTO projection.

use crate::{
    dto_common::{
        annotation_source, attachment_directory, org_duration, priority, section_source,
        semantic_property, source_position, target_kind, todo_state,
    },
    dto_model::{
        WasmArchive, WasmAttachmentState, WasmColumnViewColumn, WasmColumnViewRecord,
        WasmColumnViewScope, WasmColumnViewsResponse, WasmDateTreeEntry, WasmDateTreeResponse,
        WasmExportSettings, WasmFootnoteEntry, WasmIncludeDirective, WasmIncludeExpansionEntry,
        WasmIncludeExpansionMode, WasmIncludeExpansionResponse, WasmIncludeLineSelection,
        WasmIncludeOption, WasmKeyword, WasmKeywordAttribute, WasmLinkAbbreviation,
        WasmMacroDefinition, WasmMetadataResponse, WasmOutlineNode, WasmProgressCheckboxSummary,
        WasmProgressEffortSummary, WasmProgressStatisticCookie, WasmProgressStatsRecord,
        WasmProgressStatsResponse, WasmProgressTodoSummary, WasmSourceBlockCodeRef,
        WasmSourceBlockHeaderArg, WasmSourceBlockHeaderVar, WasmSourceBlockRecord,
        WasmSourceBlockReference, WasmSourceBlockResult, WasmSourceBlockTangle,
        WasmSourceBlocksResponse, WasmTagDefinition, WasmTargetDefinition, WasmTaskBlockerParent,
        WasmTaskBlockerRecord, WasmTaskBlockerTask, WasmTaskBlockersResponse,
        WasmTaskDependencyRecord,
    },
    dto_shared_model::WasmSourceRange,
};
use orgize::ast::{
    ColumnViewRecord, ColumnViewScope, ColumnViewSource, DateTreeEntry, Document, IncludeDirective,
    IncludeExpansionMode, IncludeExpansionOptions, IncludeLineSelection, ParsedAnnotation, Section,
    SourceBlockHeaderArgKind, SourceBlockHeaderArgSource, SourceBlockRecord, SourceBlockRecordKind,
    SourceBlockReference, SourceBlockReferenceKind, SourceBlockResultKind, SourceBlockSource,
    SourceBlockTangleMode, TaskBlockerRecord, TaskBlockerTask,
};

pub(crate) fn outline_node(section: &Section<ParsedAnnotation>) -> WasmOutlineNode {
    WasmOutlineNode {
        source: annotation_source(&section.ann),
        level: section.level,
        title: section.raw_title.trim_end().to_string(),
        anchor: section.anchor.clone(),
        todo: section.todo.as_ref().map(|todo| todo.name.clone()),
        todo_state: section.todo.as_ref().map(todo_state),
        priority: priority(&section.priority),
        tags: section.tags.clone(),
        effective_tags: section.effective_tags.clone(),
        is_comment: section.is_comment,
        archive: WasmArchive {
            archived: section.archive.archived,
            has_archive_tag: section.archive.has_archive_tag,
            location: section
                .archive
                .location()
                .map(|location| location.value.clone()),
        },
        attachment: WasmAttachmentState {
            has_attach_tag: section.attachment.has_attach_tag,
            directory: section
                .attachment
                .directory
                .as_ref()
                .map(|directory| attachment_directory(&directory.source, &directory.path)),
        },
        children: section.subsections.iter().map(outline_node).collect(),
    }
}

pub(crate) fn document_metadata(document: &Document<ParsedAnnotation>) -> WasmMetadataResponse {
    WasmMetadataResponse {
        schema_version: 1,
        properties: document.properties.iter().map(semantic_property).collect(),
        keywords: document
            .metadata
            .iter()
            .map(|keyword| WasmKeyword {
                source: annotation_source(&keyword.ann),
                key: keyword.key.clone(),
                optional: keyword.optional.clone(),
                value: keyword.value.clone(),
                parsed_object_count: keyword.parsed.len(),
                attributes: keyword
                    .attributes
                    .iter()
                    .map(|attribute| WasmKeywordAttribute {
                        key: attribute.key.clone(),
                        value: attribute.value.clone(),
                        raw: attribute.raw.clone(),
                    })
                    .collect(),
            })
            .collect(),
        filetags: document.filetags.clone(),
        tag_definitions: document
            .tag_definitions
            .iter()
            .map(|definition| WasmTagDefinition {
                name: definition.name.clone(),
                shortcut: definition.shortcut.clone(),
                raw: definition.raw.clone(),
                is_group: definition.is_group,
                group: definition.group.as_ref().map(|group| {
                    crate::dto_model::WasmTagDefinitionGroup {
                        name: group.name.clone(),
                        exclusive: group.exclusive,
                    }
                }),
            })
            .collect(),
        export_settings: WasmExportSettings {
            select_tags: document.export_settings.select_tags.clone(),
            exclude_tags: document.export_settings.exclude_tags.clone(),
            headline_levels: document.export_settings.headline_levels,
            special_strings: document.export_settings.special_strings,
            expand_entities: document.export_settings.expand_entities,
        },
        link_abbreviations: document
            .link_abbreviations
            .iter()
            .map(|abbrev| WasmLinkAbbreviation {
                name: abbrev.name.clone(),
                replacement: abbrev.replacement.clone(),
                raw_value: abbrev.raw_value.clone(),
            })
            .collect(),
        includes: document.includes.iter().map(include_directive).collect(),
        macros: document
            .macro_definitions
            .iter()
            .map(|definition| WasmMacroDefinition {
                source: annotation_source(&definition.ann),
                name: definition.name.clone(),
                template: definition.template.clone(),
                raw_value: definition.raw_value.clone(),
            })
            .collect(),
        targets: document
            .targets
            .iter()
            .map(|target| WasmTargetDefinition {
                source: annotation_source(&target.ann),
                kind: target_kind(target.kind),
                key: target.key.clone(),
                value: target.value.clone(),
                raw: target.raw.clone(),
                alias_object_count: target.alias.len(),
            })
            .collect(),
        footnotes: document
            .footnotes
            .iter()
            .map(|footnote| WasmFootnoteEntry {
                source: annotation_source(&footnote.ann),
                label: footnote.label.clone(),
                definition_kind: match &footnote.definition {
                    orgize::ast::FootnoteDefinition::Standalone(_) => "standalone",
                    orgize::ast::FootnoteDefinition::Inline(_) => "inline",
                },
            })
            .collect(),
    }
}

pub(crate) fn source_blocks_response(
    document: &Document<ParsedAnnotation>,
) -> WasmSourceBlocksResponse {
    WasmSourceBlocksResponse {
        schema_version: 1,
        records: source_block_records(document),
        references: source_block_references(document),
    }
}

pub(crate) fn source_block_records(
    document: &Document<ParsedAnnotation>,
) -> Vec<WasmSourceBlockRecord> {
    document
        .source_block_records()
        .iter()
        .map(source_block_record)
        .collect()
}

pub(crate) fn source_block_references(
    document: &Document<ParsedAnnotation>,
) -> Vec<WasmSourceBlockReference> {
    document
        .source_block_references()
        .iter()
        .map(source_block_reference)
        .collect()
}

pub(crate) fn column_views_response(
    document: &Document<ParsedAnnotation>,
) -> WasmColumnViewsResponse {
    WasmColumnViewsResponse {
        schema_version: 1,
        records: column_view_records(document),
    }
}

pub(crate) fn column_view_records(
    document: &Document<ParsedAnnotation>,
) -> Vec<WasmColumnViewRecord> {
    document
        .column_view_records()
        .iter()
        .map(column_view_record)
        .collect()
}

pub(crate) fn include_expansion_response(
    document: &Document<ParsedAnnotation>,
    base_dir: Option<&str>,
) -> WasmIncludeExpansionResponse {
    let options = IncludeExpansionOptions {
        base_dir: base_dir.map(str::to_string),
    };
    WasmIncludeExpansionResponse {
        schema_version: 1,
        entries: include_expansion_entries(document, &options),
    }
}

pub(crate) fn include_expansion_entries(
    document: &Document<ParsedAnnotation>,
    options: &IncludeExpansionOptions,
) -> Vec<WasmIncludeExpansionEntry> {
    document
        .include_expansion_plan(options)
        .entries
        .iter()
        .map(include_expansion_entry)
        .collect()
}

pub(crate) fn datetree_response(document: &Document<ParsedAnnotation>) -> WasmDateTreeResponse {
    WasmDateTreeResponse {
        schema_version: 1,
        records: datetree_entries(document),
    }
}

pub(crate) fn datetree_entries(document: &Document<ParsedAnnotation>) -> Vec<WasmDateTreeEntry> {
    document
        .datetree_entries()
        .iter()
        .map(datetree_entry)
        .collect()
}

pub(crate) fn progress_stats_response(
    document: &Document<ParsedAnnotation>,
) -> WasmProgressStatsResponse {
    WasmProgressStatsResponse {
        schema_version: 1,
        records: progress_stats_records(document),
    }
}

pub(crate) fn progress_stats_records(
    document: &Document<ParsedAnnotation>,
) -> Vec<WasmProgressStatsRecord> {
    document
        .progress_stats_records()
        .iter()
        .map(progress_stats_record)
        .collect()
}

pub(crate) fn task_blockers_response(
    document: &Document<ParsedAnnotation>,
) -> WasmTaskBlockersResponse {
    WasmTaskBlockersResponse {
        schema_version: 1,
        records: task_blocker_records(document),
    }
}

pub(crate) fn task_blocker_records(
    document: &Document<ParsedAnnotation>,
) -> Vec<WasmTaskBlockerRecord> {
    document
        .task_blocker_records()
        .iter()
        .map(task_blocker_record)
        .collect()
}

fn source_block_record(record: &SourceBlockRecord) -> WasmSourceBlockRecord {
    WasmSourceBlockRecord {
        source: source_block_source(&record.source),
        kind: match record.kind {
            SourceBlockRecordKind::Block => "block",
            SourceBlockRecordKind::InlineSource => "inlineSource",
        },
        name: record.name.clone(),
        language: record.language.clone(),
        parameters: record.parameters.clone(),
        header_args: record
            .normalized_header_args
            .iter()
            .map(|arg| WasmSourceBlockHeaderArg {
                key: arg.key.clone(),
                value: arg.value.clone(),
                raw: arg.raw.clone(),
                kind: source_block_header_arg_kind(arg.kind),
                source: source_block_header_arg_source(arg.source),
                tokens: arg.tokens.clone(),
                variable: arg
                    .variable
                    .as_ref()
                    .map(|variable| WasmSourceBlockHeaderVar {
                        name: variable.name.clone(),
                        assignment: variable.assignment.clone(),
                    }),
            })
            .collect(),
        code_refs: record
            .code_refs
            .iter()
            .map(|code_ref| WasmSourceBlockCodeRef {
                line: code_ref.line,
                column: code_ref.column,
                end_column: code_ref.end_column,
                name: code_ref.name.clone(),
                raw: code_ref.raw.clone(),
            })
            .collect(),
        tangle: record.tangle.as_ref().map(|tangle| WasmSourceBlockTangle {
            raw: tangle.raw.clone(),
            mode: match tangle.mode {
                SourceBlockTangleMode::Yes => "yes",
                SourceBlockTangleMode::No => "no",
                SourceBlockTangleMode::File => "file",
            },
            target: tangle.target.clone(),
        }),
        result: record.result.as_ref().map(|result| WasmSourceBlockResult {
            source: source_block_source(&result.source),
            kind: match result.kind {
                SourceBlockResultKind::Keyword => "keyword",
                SourceBlockResultKind::InlineMacro => "inlineMacro",
            },
            hash: result.hash.clone(),
            name: result.name.clone(),
            keyword_value: result.keyword_value.clone(),
            value: result.value.clone(),
        }),
        value: record.value.clone(),
    }
}

fn source_block_reference(reference: &SourceBlockReference) -> WasmSourceBlockReference {
    WasmSourceBlockReference {
        source: source_block_source(&reference.source),
        kind: match reference.kind {
            SourceBlockReferenceKind::BabelCall => "babelCall",
            SourceBlockReferenceKind::HeaderVar => "headerVar",
            SourceBlockReferenceKind::InlineCall => "inlineCall",
            SourceBlockReferenceKind::Noweb => "noweb",
        },
        variable: reference.variable.clone(),
        target: reference.target.clone(),
        resolved: reference.resolved,
    }
}

fn column_view_record(record: &ColumnViewRecord) -> WasmColumnViewRecord {
    WasmColumnViewRecord {
        source: column_view_source(&record.source),
        scope: match &record.scope {
            ColumnViewScope::DocumentKeyword => WasmColumnViewScope {
                kind: "documentKeyword",
                level: None,
                title: None,
                outline_path: Vec::new(),
            },
            ColumnViewScope::DocumentProperty => WasmColumnViewScope {
                kind: "documentProperty",
                level: None,
                title: None,
                outline_path: Vec::new(),
            },
            ColumnViewScope::SectionProperty {
                level,
                title,
                outline_path,
            } => WasmColumnViewScope {
                kind: "sectionProperty",
                level: Some(*level),
                title: Some(title.clone()),
                outline_path: outline_path.clone(),
            },
        },
        raw: record.raw.clone(),
        columns: record
            .columns
            .iter()
            .map(|column| WasmColumnViewColumn {
                property: column.property.clone(),
                title: column.title.clone(),
                width: column.width,
                summary_operator: column.summary_operator.clone(),
                summary_format: column.summary_format.clone(),
                raw: column.raw.clone(),
            })
            .collect(),
    }
}

fn include_directive(include: &IncludeDirective<ParsedAnnotation>) -> WasmIncludeDirective {
    WasmIncludeDirective {
        source: annotation_source(&include.ann),
        path: include.path.clone(),
        raw_path: include.raw_path.clone(),
        arguments: include.arguments.clone(),
        options: include.options.iter().map(include_option).collect(),
        raw_value: include.raw_value.clone(),
    }
}

fn include_option(option: &orgize::ast::IncludeOption) -> WasmIncludeOption {
    WasmIncludeOption {
        key: option.key.clone(),
        value: option.value.clone(),
        raw: option.raw.clone(),
    }
}

fn include_expansion_entry(
    entry: &orgize::ast::IncludeExpansionEntry<ParsedAnnotation>,
) -> WasmIncludeExpansionEntry {
    WasmIncludeExpansionEntry {
        directive: include_directive(&entry.directive),
        resolved_path: entry.resolved_path.clone(),
        line_selection: include_line_selection(&entry.line_selection),
        min_level: entry.min_level,
        mode: include_expansion_mode(&entry.mode),
        options: entry.options.iter().map(include_option).collect(),
    }
}

fn include_line_selection(selection: &IncludeLineSelection) -> WasmIncludeLineSelection {
    match selection {
        IncludeLineSelection::All => WasmIncludeLineSelection {
            kind: "all",
            start: None,
            end: None,
            raw: None,
        },
        IncludeLineSelection::Range { start, end, raw } => WasmIncludeLineSelection {
            kind: "range",
            start: *start,
            end: *end,
            raw: Some(raw.clone()),
        },
        IncludeLineSelection::Invalid { raw } => WasmIncludeLineSelection {
            kind: "invalid",
            start: None,
            end: None,
            raw: Some(raw.clone()),
        },
    }
}

fn include_expansion_mode(mode: &IncludeExpansionMode) -> WasmIncludeExpansionMode {
    match mode {
        IncludeExpansionMode::Org => WasmIncludeExpansionMode {
            kind: "org",
            language: None,
            backend: None,
            arguments: Vec::new(),
        },
        IncludeExpansionMode::Example => WasmIncludeExpansionMode {
            kind: "example",
            language: None,
            backend: None,
            arguments: Vec::new(),
        },
        IncludeExpansionMode::Source { language } => WasmIncludeExpansionMode {
            kind: "source",
            language: language.clone(),
            backend: None,
            arguments: Vec::new(),
        },
        IncludeExpansionMode::Export { backend } => WasmIncludeExpansionMode {
            kind: "export",
            language: None,
            backend: backend.clone(),
            arguments: Vec::new(),
        },
        IncludeExpansionMode::Other { arguments } => WasmIncludeExpansionMode {
            kind: "other",
            language: None,
            backend: None,
            arguments: arguments.clone(),
        },
    }
}

fn datetree_entry(entry: &DateTreeEntry) -> WasmDateTreeEntry {
    WasmDateTreeEntry {
        source: section_source(&entry.source),
        date: format!(
            "{:04}-{:02}-{:02}",
            entry.date.year, entry.date.month, entry.date.day
        ),
        year: entry.date.year,
        month: entry.date.month,
        day: entry.date.day,
        year_title: entry.year_title.clone(),
        month_title: entry.month_title.clone(),
        day_title: entry.day_title.clone(),
        outline_path: entry.outline_path.clone(),
    }
}

fn progress_stats_record(record: &orgize::ast::ProgressStatsRecord) -> WasmProgressStatsRecord {
    WasmProgressStatsRecord {
        source: section_source(&record.source),
        outline_path: record.outline_path.clone(),
        level: record.level,
        title: record.title.clone(),
        todo: record.todo.as_str(),
        descendant_todos: WasmProgressTodoSummary {
            total: record.descendant_todos.total,
            done: record.descendant_todos.done,
            open: record.descendant_todos.open,
        },
        checkboxes: WasmProgressCheckboxSummary {
            total: record.checkboxes.total,
            checked: record.checkboxes.checked,
            unchecked: record.checkboxes.unchecked,
            partial: record.checkboxes.partial,
        },
        statistic_cookies: record
            .statistic_cookies
            .iter()
            .map(|cookie| WasmProgressStatisticCookie {
                source: section_source(&cookie.source),
                raw: cookie.raw.clone(),
                kind: cookie.kind.as_str(),
                done: cookie.done,
                total: cookie.total,
                percent: cookie.percent,
            })
            .collect(),
        effort: WasmProgressEffortSummary {
            local: record.effort.local.as_ref().map(org_duration),
            subtree_total_seconds: record.effort.subtree_total_seconds,
        },
        dependencies: record
            .dependencies
            .iter()
            .map(|dependency| WasmTaskDependencyRecord {
                source: section_source(&dependency.source),
                kind: dependency.kind.as_str(),
                count: dependency.count,
                message: dependency.message.clone(),
            })
            .collect(),
    }
}

pub(crate) fn task_blocker_record(record: &TaskBlockerRecord) -> WasmTaskBlockerRecord {
    WasmTaskBlockerRecord {
        kind: record.kind.as_str(),
        blocked: task_blocker_task(&record.blocked),
        blocker: task_blocker_task(&record.blocker),
        parent: WasmTaskBlockerParent {
            source: section_source(&record.parent.source),
            ordered_property_source: section_source(&record.parent.ordered_property_source),
            outline_path: record.parent.outline_path.clone(),
            level: record.parent.level,
            title: record.parent.title.clone(),
        },
        message: record.message.clone(),
    }
}

fn task_blocker_task(task: &TaskBlockerTask) -> WasmTaskBlockerTask {
    WasmTaskBlockerTask {
        source: section_source(&task.source),
        outline_path: task.outline_path.clone(),
        level: task.level,
        title: task.title.clone(),
        todo: task.todo.as_ref().map(|todo| todo.name.clone()),
        todo_state: task.todo.as_ref().map(todo_state),
    }
}

fn source_block_source(source: &SourceBlockSource) -> WasmSourceRange {
    WasmSourceRange {
        start: source_position(source.start),
        end: source_position(source.end),
        range_start: source.range_start,
        range_end: source.range_end,
    }
}

fn column_view_source(source: &ColumnViewSource) -> WasmSourceRange {
    WasmSourceRange {
        start: source_position(source.start),
        end: source_position(source.end),
        range_start: source.range_start,
        range_end: source.range_end,
    }
}

fn source_block_header_arg_kind(kind: SourceBlockHeaderArgKind) -> &'static str {
    match kind {
        SourceBlockHeaderArgKind::Cache => "cache",
        SourceBlockHeaderArgKind::Dir => "dir",
        SourceBlockHeaderArgKind::Eval => "eval",
        SourceBlockHeaderArgKind::Exports => "exports",
        SourceBlockHeaderArgKind::Hlines => "hlines",
        SourceBlockHeaderArgKind::Noweb => "noweb",
        SourceBlockHeaderArgKind::Results => "results",
        SourceBlockHeaderArgKind::Session => "session",
        SourceBlockHeaderArgKind::Tangle => "tangle",
        SourceBlockHeaderArgKind::Var => "var",
        SourceBlockHeaderArgKind::Other => "other",
    }
}

fn source_block_header_arg_source(source: SourceBlockHeaderArgSource) -> &'static str {
    match source {
        SourceBlockHeaderArgSource::Explicit => "explicit",
        SourceBlockHeaderArgSource::Default => "default",
    }
}
