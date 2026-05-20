//! Stable TypeScript-facing DTO models for Org Crypt projections.

use crate::dto_shared_model::WasmSourceRange;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmCryptResponse {
    pub(crate) schema_version: u8,
    pub(crate) records: Vec<WasmCryptRecord>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmCryptRecord {
    pub(crate) source: WasmSourceRange,
    pub(crate) outline_path: Vec<String>,
    pub(crate) level: usize,
    pub(crate) title: String,
    pub(crate) tag: String,
    pub(crate) has_direct_tag: bool,
    pub(crate) has_inherited_tag: bool,
    pub(crate) crypt_key: Option<WasmCryptKey>,
    pub(crate) encrypted_payload: bool,
    pub(crate) body_is_opaque: bool,
    pub(crate) warnings: Vec<WasmCryptWarning>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmCryptKey {
    pub(crate) source: WasmSourceRange,
    pub(crate) value: String,
    pub(crate) inherited: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmCryptWarning {
    pub(crate) kind: &'static str,
    pub(crate) message: String,
}
