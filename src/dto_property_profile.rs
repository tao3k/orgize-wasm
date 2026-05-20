//! Property profile WebAssembly DTO projection.

use crate::{
    dto_common::section_source,
    dto_property_profile_model::{
        WasmPropertyAllowedValueRecord, WasmPropertyAllowedValueScope, WasmPropertyProfile,
        WasmPropertyProfileResponse, WasmPropertySchemaApplication,
        WasmPropertySchemaContractInput, WasmPropertySchemaFieldInput, WasmPropertySchemaFinding,
        WasmPropertySchemaReference, WasmPropertySchemaRegistryRequest, WasmPropertySchemaScope,
        WasmPropertySchemaValueRuleInput,
    },
};
use orgize::ast::{
    Document, ParsedAnnotation, PropertyAllowedValueRecord, PropertyAllowedValueScope,
    PropertySchemaApplication, PropertySchemaContract, PropertySchemaField, PropertySchemaFinding,
    PropertySchemaReference, PropertySchemaRegistry, PropertySchemaScope, PropertySchemaValueRule,
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
    property_profile_from_core(profile)
}

pub(crate) fn property_profile_with_schema_registry(
    document: &Document<ParsedAnnotation>,
    registry: &PropertySchemaRegistry,
) -> WasmPropertyProfile {
    property_profile_from_core(document.property_profile_with_schema_registry(registry))
}

pub(crate) fn property_profile_with_schema_registry_response(
    document: &Document<ParsedAnnotation>,
    request: WasmPropertySchemaRegistryRequest,
) -> WasmPropertyProfileResponse {
    let registry = property_schema_registry(request);
    WasmPropertyProfileResponse {
        schema_version: 1,
        profile: property_profile_with_schema_registry(document, &registry),
    }
}

fn property_profile_from_core(profile: orgize::ast::PropertyProfile) -> WasmPropertyProfile {
    WasmPropertyProfile {
        inheritance: profile.inheritance.as_str(),
        inherited_keys: profile.inherited_keys,
        allowed_values: profile
            .allowed_values
            .into_iter()
            .map(property_allowed_value_record)
            .collect(),
        schema_applications: profile
            .schema_applications
            .into_iter()
            .map(property_schema_application)
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

fn property_schema_application(
    application: PropertySchemaApplication,
) -> WasmPropertySchemaApplication {
    WasmPropertySchemaApplication {
        source: section_source(&application.source),
        scope: property_schema_scope(application.scope),
        reference: property_schema_reference(application.reference),
        contract_id: application.contract_id,
        findings: application
            .findings
            .into_iter()
            .map(property_schema_finding)
            .collect(),
    }
}

fn property_schema_scope(scope: PropertySchemaScope) -> WasmPropertySchemaScope {
    match scope {
        PropertySchemaScope::Document => WasmPropertySchemaScope {
            kind: "document",
            outline_path: Vec::new(),
            level: None,
            title: None,
        },
        PropertySchemaScope::Section {
            outline_path,
            level,
            title,
        } => WasmPropertySchemaScope {
            kind: "section",
            outline_path,
            level: Some(level),
            title: Some(title),
        },
    }
}

fn property_schema_reference(reference: PropertySchemaReference) -> WasmPropertySchemaReference {
    WasmPropertySchemaReference {
        raw: reference.raw,
        normalized: reference.normalized,
        kind: reference.kind.as_str(),
    }
}

fn property_schema_finding(finding: PropertySchemaFinding) -> WasmPropertySchemaFinding {
    WasmPropertySchemaFinding {
        source: section_source(&finding.source),
        kind: finding.kind.as_str(),
        property: finding.property,
        actual: finding.actual,
        expected: finding.expected,
        message: finding.message,
    }
}

pub(crate) fn property_schema_registry(
    request: WasmPropertySchemaRegistryRequest,
) -> PropertySchemaRegistry {
    PropertySchemaRegistry::new(request.contracts.into_iter().map(property_schema_contract))
}

fn property_schema_contract(input: WasmPropertySchemaContractInput) -> PropertySchemaContract {
    let mut contract = PropertySchemaContract::new(input.id)
        .allow_unknown_properties(input.allow_unknown_properties);
    for alias in input.aliases {
        contract = contract.alias(alias);
    }
    for field in input.fields {
        contract = contract.field(property_schema_field(field));
    }
    contract
}

fn property_schema_field(input: WasmPropertySchemaFieldInput) -> PropertySchemaField {
    if input.required {
        PropertySchemaField::required(input.key, property_schema_value_rule(input.value_rule))
    } else {
        PropertySchemaField::optional(input.key, property_schema_value_rule(input.value_rule))
    }
}

fn property_schema_value_rule(input: WasmPropertySchemaValueRuleInput) -> PropertySchemaValueRule {
    match input {
        WasmPropertySchemaValueRuleInput::Any => PropertySchemaValueRule::Any,
        WasmPropertySchemaValueRuleInput::NonEmpty => PropertySchemaValueRule::NonEmpty,
        WasmPropertySchemaValueRuleInput::OneOf { values } => {
            PropertySchemaValueRule::OneOf(values)
        }
    }
}
