//! Property profile WebAssembly DTO projection.

use crate::{
    dto_common::section_source,
    dto_property_profile_model::{
        WasmPropertyAllowedValueRecord, WasmPropertyAllowedValueScope, WasmPropertyProfile,
        WasmPropertyProfileResponse,
    },
};
use orgize::ast::{
    Document, ParsedAnnotation, PropertyAllowedValueRecord, PropertyAllowedValueScope,
};

pub(crate) fn property_profile_response(
    document: &Document<ParsedAnnotation>,
) -> WasmPropertyProfileResponse {
    WasmPropertyProfileResponse {
        schema_version: 1,
        profile: property_profile(document),
    }
}

pub(crate) fn property_profile(document: &Document<ParsedAnnotation>) -> WasmPropertyProfile {
    let profile = document.property_profile();
    WasmPropertyProfile {
        inheritance: profile.inheritance.as_str(),
        inherited_keys: profile.inherited_keys,
        allowed_values: profile
            .allowed_values
            .into_iter()
            .map(property_allowed_value_record)
            .collect(),
    }
}

fn property_allowed_value_record(
    record: PropertyAllowedValueRecord,
) -> WasmPropertyAllowedValueRecord {
    WasmPropertyAllowedValueRecord {
        source: record.source.as_ref().map(section_source),
        scope: property_allowed_value_scope(record.scope),
        property: record.property,
        descriptor_key: record.descriptor_key,
        values: record.values,
    }
}

fn property_allowed_value_scope(scope: PropertyAllowedValueScope) -> WasmPropertyAllowedValueScope {
    match scope {
        PropertyAllowedValueScope::FixedGlobal => WasmPropertyAllowedValueScope {
            kind: "fixedGlobal",
            outline_path: Vec::new(),
            level: None,
            title: None,
        },
        PropertyAllowedValueScope::Document => WasmPropertyAllowedValueScope {
            kind: "document",
            outline_path: Vec::new(),
            level: None,
            title: None,
        },
        PropertyAllowedValueScope::Section {
            outline_path,
            level,
            title,
        } => WasmPropertyAllowedValueScope {
            kind: "section",
            outline_path,
            level: Some(level),
            title: Some(title),
        },
    }
}
