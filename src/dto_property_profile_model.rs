//! Stable TypeScript-facing DTO models for property profile projections.

use crate::dto_shared_model::WasmSourceRange;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmPropertyProfileResponse {
    pub(crate) schema_version: u8,
    pub(crate) profile: WasmPropertyProfile,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmPropertyProfile {
    pub(crate) inheritance: &'static str,
    pub(crate) inherited_keys: Vec<String>,
    pub(crate) allowed_values: Vec<WasmPropertyAllowedValueRecord>,
    pub(crate) schema_applications: Vec<WasmPropertySchemaApplication>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmPropertyAllowedValueRecord {
    pub(crate) source: Option<WasmSourceRange>,
    pub(crate) scope: WasmPropertyAllowedValueScope,
    pub(crate) property: String,
    pub(crate) descriptor_key: String,
    pub(crate) values: Vec<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmPropertyAllowedValueScope {
    pub(crate) kind: &'static str,
    pub(crate) outline_path: Vec<String>,
    pub(crate) level: Option<usize>,
    pub(crate) title: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmPropertySchemaApplication {
    pub(crate) source: WasmSourceRange,
    pub(crate) scope: WasmPropertySchemaScope,
    pub(crate) reference: WasmPropertySchemaReference,
    pub(crate) contract_id: Option<String>,
    pub(crate) findings: Vec<WasmPropertySchemaFinding>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmPropertySchemaScope {
    pub(crate) kind: &'static str,
    pub(crate) outline_path: Vec<String>,
    pub(crate) level: Option<usize>,
    pub(crate) title: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmPropertySchemaReference {
    pub(crate) raw: String,
    pub(crate) normalized: String,
    pub(crate) kind: &'static str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmPropertySchemaFinding {
    pub(crate) source: WasmSourceRange,
    pub(crate) kind: &'static str,
    pub(crate) property: Option<String>,
    pub(crate) actual: Option<String>,
    pub(crate) expected: Vec<String>,
    pub(crate) message: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmPropertySchemaRegistryRequest {
    #[serde(default)]
    pub(crate) contracts: Vec<WasmPropertySchemaContractInput>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmPropertySchemaContractInput {
    pub(crate) id: String,
    #[serde(default)]
    pub(crate) aliases: Vec<String>,
    #[serde(default)]
    pub(crate) fields: Vec<WasmPropertySchemaFieldInput>,
    #[serde(default = "default_allow_unknown_properties")]
    pub(crate) allow_unknown_properties: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmPropertySchemaFieldInput {
    pub(crate) key: String,
    #[serde(default)]
    pub(crate) required: bool,
    #[serde(default)]
    pub(crate) value_rule: WasmPropertySchemaValueRuleInput,
}

#[derive(Default, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub(crate) enum WasmPropertySchemaValueRuleInput {
    #[default]
    Any,
    NonEmpty,
    OneOf {
        values: Vec<String>,
    },
}

fn default_allow_unknown_properties() -> bool {
    true
}
