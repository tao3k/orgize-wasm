//! Source-block DTO projection helpers.

use crate::{
    dto_common::source_position,
    dto_shared_model::WasmSourceRange,
    dto_source_block_model::{
        WasmSourceBlockBooleanHeader, WasmSourceBlockCache, WasmSourceBlockCodeRef,
        WasmSourceBlockDirectory, WasmSourceBlockEval, WasmSourceBlockExecutionPlan,
        WasmSourceBlockExports, WasmSourceBlockHeaderArg, WasmSourceBlockHeaderVar,
        WasmSourceBlockNowebPlan, WasmSourceBlockRecord, WasmSourceBlockReference,
        WasmSourceBlockResult, WasmSourceBlockResultFile, WasmSourceBlockResultOptions,
        WasmSourceBlockSession, WasmSourceBlockTangle, WasmSourceBlockTangleComments,
        WasmSourceBlockTangleMkdirp, WasmSourceBlockTangleNoweb, WasmSourceBlocksResponse,
    },
};
use orgize::ast::{
    Document, ParsedAnnotation, SourceBlockBooleanHeader, SourceBlockCache, SourceBlockDirectory,
    SourceBlockDirectoryKind, SourceBlockEval, SourceBlockEvalPolicy, SourceBlockExecutionPlan,
    SourceBlockExports, SourceBlockExportsPolicy, SourceBlockHeaderArgKind,
    SourceBlockHeaderArgSource, SourceBlockNowebAction, SourceBlockNowebPlan, SourceBlockRecord,
    SourceBlockRecordKind, SourceBlockReference, SourceBlockReferenceKind,
    SourceBlockResultCollection, SourceBlockResultFormat, SourceBlockResultHandling,
    SourceBlockResultKind, SourceBlockResultOptions, SourceBlockResultValueType,
    SourceBlockSession, SourceBlockSource, SourceBlockTangleCommentsMode, SourceBlockTangleMode,
    SourceBlockTangleNowebMode,
};

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

fn source_block_references(document: &Document<ParsedAnnotation>) -> Vec<WasmSourceBlockReference> {
    document
        .source_block_references()
        .iter()
        .map(source_block_reference)
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
            mode: source_block_tangle_mode(tangle.mode),
            target: tangle.target.clone(),
            mkdirp: WasmSourceBlockTangleMkdirp {
                raw: tangle.mkdirp.raw.clone(),
                enabled: tangle.mkdirp.enabled,
            },
            comments: WasmSourceBlockTangleComments {
                raw: tangle.comments.raw.clone(),
                mode: source_block_tangle_comments_mode(tangle.comments.mode),
            },
            shebang: tangle.shebang.clone(),
            noweb: WasmSourceBlockTangleNoweb {
                raw: tangle.noweb.raw.clone(),
                mode: source_block_tangle_noweb_mode(tangle.noweb.mode),
            },
        }),
        result_options: source_block_result_options(&record.result_options),
        execution: source_block_execution_plan(&record.execution),
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

fn source_block_result_options(options: &SourceBlockResultOptions) -> WasmSourceBlockResultOptions {
    WasmSourceBlockResultOptions {
        raw: options.raw.clone(),
        source: source_block_header_arg_source(options.source),
        tokens: options.tokens.clone(),
        collection: options.collection.map(source_block_result_collection),
        format: options.format.map(source_block_result_format),
        handling: source_block_result_handling(options.handling),
        value_type: source_block_result_value_type(options.value_type),
        unknown: options.unknown.clone(),
        file: options.file.as_ref().map(|file| WasmSourceBlockResultFile {
            target: file.target.clone(),
            description: file.description.clone(),
            extension: file.extension.clone(),
            file_mode: file.file_mode.as_ref().map(|mode| mode.raw.clone()),
            output_dir: file.output_dir.clone(),
        }),
    }
}

fn source_block_execution_plan(
    execution: &SourceBlockExecutionPlan,
) -> WasmSourceBlockExecutionPlan {
    WasmSourceBlockExecutionPlan {
        eval: source_block_eval(&execution.eval),
        exports: source_block_exports(&execution.exports),
        cache: source_block_cache(&execution.cache),
        session: source_block_session(&execution.session),
        directory: execution.directory.as_ref().map(source_block_directory),
        hlines: source_block_boolean_header(&execution.hlines),
        noweb: source_block_noweb_plan(&execution.noweb),
    }
}

fn source_block_eval(eval: &SourceBlockEval) -> WasmSourceBlockEval {
    WasmSourceBlockEval {
        raw: eval.raw.clone(),
        source: source_block_header_arg_source(eval.source),
        policy: source_block_eval_policy(eval.policy),
    }
}

fn source_block_exports(exports: &SourceBlockExports) -> WasmSourceBlockExports {
    WasmSourceBlockExports {
        raw: exports.raw.clone(),
        source: source_block_header_arg_source(exports.source),
        policy: source_block_exports_policy(exports.policy),
    }
}

fn source_block_cache(cache: &SourceBlockCache) -> WasmSourceBlockCache {
    WasmSourceBlockCache {
        raw: cache.raw.clone(),
        source: source_block_header_arg_source(cache.source),
        enabled: cache.enabled,
    }
}

fn source_block_session(session: &SourceBlockSession) -> WasmSourceBlockSession {
    WasmSourceBlockSession {
        raw: session.raw.clone(),
        source: source_block_header_arg_source(session.source),
        name: session.name.clone(),
        active: session.active,
    }
}

fn source_block_directory(directory: &SourceBlockDirectory) -> WasmSourceBlockDirectory {
    WasmSourceBlockDirectory {
        raw: directory.raw.clone(),
        source: source_block_header_arg_source(directory.source),
        target: directory.target.clone(),
        kind: source_block_directory_kind(directory.kind),
    }
}

fn source_block_boolean_header(header: &SourceBlockBooleanHeader) -> WasmSourceBlockBooleanHeader {
    WasmSourceBlockBooleanHeader {
        raw: header.raw.clone(),
        source: source_block_header_arg_source(header.source),
        enabled: header.enabled,
    }
}

fn source_block_noweb_plan(noweb: &SourceBlockNowebPlan) -> WasmSourceBlockNowebPlan {
    WasmSourceBlockNowebPlan {
        raw: noweb.raw.clone(),
        source: source_block_header_arg_source(noweb.source),
        tokens: noweb.tokens.clone(),
        eval: source_block_noweb_action(noweb.eval),
        export: source_block_noweb_action(noweb.export),
        tangle: source_block_noweb_action(noweb.tangle),
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

fn source_block_header_arg_kind(kind: SourceBlockHeaderArgKind) -> &'static str {
    match kind {
        SourceBlockHeaderArgKind::Cache => "cache",
        SourceBlockHeaderArgKind::Dir => "dir",
        SourceBlockHeaderArgKind::Eval => "eval",
        SourceBlockHeaderArgKind::Exports => "exports",
        SourceBlockHeaderArgKind::File => "file",
        SourceBlockHeaderArgKind::FileDesc => "fileDesc",
        SourceBlockHeaderArgKind::FileExt => "fileExt",
        SourceBlockHeaderArgKind::FileMode => "fileMode",
        SourceBlockHeaderArgKind::Hlines => "hlines",
        SourceBlockHeaderArgKind::Noweb => "noweb",
        SourceBlockHeaderArgKind::OutputDir => "outputDir",
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

fn source_block_tangle_mode(mode: SourceBlockTangleMode) -> &'static str {
    match mode {
        SourceBlockTangleMode::Yes => "yes",
        SourceBlockTangleMode::No => "no",
        SourceBlockTangleMode::File => "file",
    }
}

fn source_block_tangle_comments_mode(mode: SourceBlockTangleCommentsMode) -> &'static str {
    match mode {
        SourceBlockTangleCommentsMode::No => "no",
        SourceBlockTangleCommentsMode::Link => "link",
        SourceBlockTangleCommentsMode::Yes => "yes",
        SourceBlockTangleCommentsMode::Org => "org",
        SourceBlockTangleCommentsMode::Both => "both",
        SourceBlockTangleCommentsMode::Noweb => "noweb",
        SourceBlockTangleCommentsMode::Other => "other",
    }
}

fn source_block_tangle_noweb_mode(mode: SourceBlockTangleNowebMode) -> &'static str {
    match mode {
        SourceBlockTangleNowebMode::Disabled => "disabled",
        SourceBlockTangleNowebMode::Expand => "expand",
        SourceBlockTangleNowebMode::Strip => "strip",
    }
}

fn source_block_result_collection(collection: SourceBlockResultCollection) -> &'static str {
    match collection {
        SourceBlockResultCollection::File => "file",
        SourceBlockResultCollection::List => "list",
        SourceBlockResultCollection::Vector => "vector",
        SourceBlockResultCollection::Table => "table",
        SourceBlockResultCollection::Scalar => "scalar",
        SourceBlockResultCollection::Verbatim => "verbatim",
    }
}

fn source_block_result_format(format: SourceBlockResultFormat) -> &'static str {
    match format {
        SourceBlockResultFormat::Raw => "raw",
        SourceBlockResultFormat::Html => "html",
        SourceBlockResultFormat::Latex => "latex",
        SourceBlockResultFormat::Org => "org",
        SourceBlockResultFormat::Code => "code",
        SourceBlockResultFormat::Pp => "pp",
        SourceBlockResultFormat::Drawer => "drawer",
        SourceBlockResultFormat::Link => "link",
        SourceBlockResultFormat::Graphics => "graphics",
    }
}

fn source_block_result_handling(handling: SourceBlockResultHandling) -> &'static str {
    match handling {
        SourceBlockResultHandling::Replace => "replace",
        SourceBlockResultHandling::Silent => "silent",
        SourceBlockResultHandling::None => "none",
        SourceBlockResultHandling::Discard => "discard",
        SourceBlockResultHandling::Append => "append",
        SourceBlockResultHandling::Prepend => "prepend",
    }
}

fn source_block_result_value_type(value_type: SourceBlockResultValueType) -> &'static str {
    match value_type {
        SourceBlockResultValueType::Value => "value",
        SourceBlockResultValueType::Output => "output",
    }
}

fn source_block_eval_policy(policy: SourceBlockEvalPolicy) -> &'static str {
    match policy {
        SourceBlockEvalPolicy::Yes => "yes",
        SourceBlockEvalPolicy::No => "no",
        SourceBlockEvalPolicy::NoExport => "noExport",
        SourceBlockEvalPolicy::StripExport => "stripExport",
        SourceBlockEvalPolicy::NeverExport => "neverExport",
        SourceBlockEvalPolicy::Eval => "eval",
        SourceBlockEvalPolicy::Never => "never",
        SourceBlockEvalPolicy::Query => "query",
        SourceBlockEvalPolicy::Other => "other",
    }
}

fn source_block_exports_policy(policy: SourceBlockExportsPolicy) -> &'static str {
    match policy {
        SourceBlockExportsPolicy::Code => "code",
        SourceBlockExportsPolicy::Results => "results",
        SourceBlockExportsPolicy::Both => "both",
        SourceBlockExportsPolicy::None => "none",
        SourceBlockExportsPolicy::Other => "other",
    }
}

fn source_block_directory_kind(kind: SourceBlockDirectoryKind) -> &'static str {
    match kind {
        SourceBlockDirectoryKind::Path => "path",
        SourceBlockDirectoryKind::Attachment => "attachment",
    }
}

fn source_block_noweb_action(action: SourceBlockNowebAction) -> &'static str {
    match action {
        SourceBlockNowebAction::Disabled => "disabled",
        SourceBlockNowebAction::Expand => "expand",
        SourceBlockNowebAction::Strip => "strip",
    }
}
