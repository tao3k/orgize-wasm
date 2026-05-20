//! Org Crypt WebAssembly DTO projection.

use crate::{
    dto_common::section_source,
    dto_crypt_model::{WasmCryptKey, WasmCryptRecord, WasmCryptResponse, WasmCryptWarning},
};
use orgize::ast::{CryptState, Document, ParsedAnnotation};

pub(crate) fn crypt_response(document: &Document<ParsedAnnotation>) -> WasmCryptResponse {
    WasmCryptResponse {
        schema_version: 1,
        records: crypt_records(document),
    }
}

pub(crate) fn crypt_records(document: &Document<ParsedAnnotation>) -> Vec<WasmCryptRecord> {
    document.crypt_states().iter().map(crypt_record).collect()
}

fn crypt_record(state: &CryptState) -> WasmCryptRecord {
    WasmCryptRecord {
        source: section_source(&state.source),
        outline_path: state.outline_path.clone(),
        level: state.level,
        title: state.title.clone(),
        tag: state.tag.as_str().to_string(),
        has_direct_tag: state.has_direct_tag,
        has_inherited_tag: state.has_inherited_tag,
        crypt_key: state.crypt_key.as_ref().map(|key| WasmCryptKey {
            source: section_source(&key.source),
            value: key.value.clone(),
            inherited: key.inherited,
        }),
        encrypted_payload: state.encrypted_payload,
        body_is_opaque: state.body_is_opaque,
        warnings: state
            .warnings
            .iter()
            .map(|warning| WasmCryptWarning {
                kind: warning.kind.as_str(),
                message: warning.message.clone(),
            })
            .collect(),
    }
}
