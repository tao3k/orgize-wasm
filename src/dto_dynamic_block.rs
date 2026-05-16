//! Dynamic block WebAssembly DTO projection.

use crate::{
    dto_common::section_source,
    dto_dynamic_block_model::{
        WasmDynamicBlockParameter, WasmDynamicBlockRecord, WasmDynamicBlocksResponse,
    },
};
use orgize::ast::{Document, DynamicBlockRecord, ParsedAnnotation};

pub(crate) fn dynamic_blocks_response(
    document: &Document<ParsedAnnotation>,
) -> WasmDynamicBlocksResponse {
    WasmDynamicBlocksResponse {
        schema_version: 1,
        records: dynamic_block_records(document),
    }
}

pub(crate) fn dynamic_block_records(
    document: &Document<ParsedAnnotation>,
) -> Vec<WasmDynamicBlockRecord> {
    document
        .dynamic_block_records()
        .iter()
        .map(dynamic_block_record)
        .collect()
}

fn dynamic_block_record(record: &DynamicBlockRecord) -> WasmDynamicBlockRecord {
    WasmDynamicBlockRecord {
        source: section_source(&record.source),
        name: record.name.clone(),
        writer: record.writer.as_str(),
        parameters: record
            .parameters
            .iter()
            .map(|parameter| WasmDynamicBlockParameter {
                key: parameter.key.clone(),
                value: parameter.value.clone(),
                raw: parameter.raw.clone(),
            })
            .collect(),
        content_state: record.content_state.as_str(),
        content_line_count: record.content_line_count,
    }
}
