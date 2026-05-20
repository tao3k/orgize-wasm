//! Stable TypeScript-facing DTO models for source-block projections.

use crate::dto_shared_model::WasmSourceRange;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmSourceBlocksResponse {
    pub(crate) schema_version: u8,
    pub(crate) records: Vec<WasmSourceBlockRecord>,
    pub(crate) references: Vec<WasmSourceBlockReference>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmSourceBlockRecord {
    pub(crate) source: WasmSourceRange,
    pub(crate) kind: &'static str,
    pub(crate) name: Option<String>,
    pub(crate) language: Option<String>,
    pub(crate) parameters: Option<String>,
    pub(crate) header_args: Vec<WasmSourceBlockHeaderArg>,
    pub(crate) code_refs: Vec<WasmSourceBlockCodeRef>,
    pub(crate) tangle: Option<WasmSourceBlockTangle>,
    pub(crate) result_options: WasmSourceBlockResultOptions,
    pub(crate) execution: WasmSourceBlockExecutionPlan,
    pub(crate) result: Option<WasmSourceBlockResult>,
    pub(crate) value: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmSourceBlockReference {
    pub(crate) source: WasmSourceRange,
    pub(crate) kind: &'static str,
    pub(crate) variable: Option<String>,
    pub(crate) target: String,
    pub(crate) resolved: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmSourceBlockHeaderArg {
    pub(crate) key: String,
    pub(crate) value: Option<String>,
    pub(crate) raw: String,
    pub(crate) kind: &'static str,
    pub(crate) source: &'static str,
    pub(crate) tokens: Vec<String>,
    pub(crate) variable: Option<WasmSourceBlockHeaderVar>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmSourceBlockHeaderVar {
    pub(crate) name: String,
    pub(crate) assignment: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmSourceBlockCodeRef {
    pub(crate) line: usize,
    pub(crate) column: usize,
    pub(crate) end_column: usize,
    pub(crate) name: String,
    pub(crate) raw: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmSourceBlockTangle {
    pub(crate) raw: String,
    pub(crate) mode: &'static str,
    pub(crate) target: Option<String>,
    pub(crate) mkdirp: WasmSourceBlockTangleMkdirp,
    pub(crate) comments: WasmSourceBlockTangleComments,
    pub(crate) shebang: Option<String>,
    pub(crate) noweb: WasmSourceBlockTangleNoweb,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmSourceBlockTangleMkdirp {
    pub(crate) raw: String,
    pub(crate) enabled: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmSourceBlockTangleComments {
    pub(crate) raw: String,
    pub(crate) mode: &'static str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmSourceBlockTangleNoweb {
    pub(crate) raw: String,
    pub(crate) mode: &'static str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmSourceBlockResult {
    pub(crate) source: WasmSourceRange,
    pub(crate) kind: &'static str,
    pub(crate) hash: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) keyword_value: String,
    pub(crate) value: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmSourceBlockResultOptions {
    pub(crate) raw: String,
    pub(crate) source: &'static str,
    pub(crate) tokens: Vec<String>,
    pub(crate) collection: Option<&'static str>,
    pub(crate) format: Option<&'static str>,
    pub(crate) handling: &'static str,
    pub(crate) value_type: &'static str,
    pub(crate) unknown: Vec<String>,
    pub(crate) file: Option<WasmSourceBlockResultFile>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmSourceBlockResultFile {
    pub(crate) target: String,
    pub(crate) description: Option<String>,
    pub(crate) extension: Option<String>,
    pub(crate) file_mode: Option<String>,
    pub(crate) output_dir: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmSourceBlockExecutionPlan {
    pub(crate) eval: WasmSourceBlockEval,
    pub(crate) exports: WasmSourceBlockExports,
    pub(crate) cache: WasmSourceBlockCache,
    pub(crate) session: WasmSourceBlockSession,
    pub(crate) directory: Option<WasmSourceBlockDirectory>,
    pub(crate) hlines: WasmSourceBlockBooleanHeader,
    pub(crate) noweb: WasmSourceBlockNowebPlan,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmSourceBlockEval {
    pub(crate) raw: String,
    pub(crate) source: &'static str,
    pub(crate) policy: &'static str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmSourceBlockExports {
    pub(crate) raw: String,
    pub(crate) source: &'static str,
    pub(crate) policy: &'static str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmSourceBlockCache {
    pub(crate) raw: String,
    pub(crate) source: &'static str,
    pub(crate) enabled: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmSourceBlockSession {
    pub(crate) raw: String,
    pub(crate) source: &'static str,
    pub(crate) name: Option<String>,
    pub(crate) active: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmSourceBlockDirectory {
    pub(crate) raw: String,
    pub(crate) source: &'static str,
    pub(crate) target: String,
    pub(crate) kind: &'static str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmSourceBlockBooleanHeader {
    pub(crate) raw: String,
    pub(crate) source: &'static str,
    pub(crate) enabled: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmSourceBlockNowebPlan {
    pub(crate) raw: String,
    pub(crate) source: &'static str,
    pub(crate) tokens: Vec<String>,
    pub(crate) eval: &'static str,
    pub(crate) export: &'static str,
    pub(crate) tangle: &'static str,
}
