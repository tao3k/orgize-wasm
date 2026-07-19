use std::path::Path;

use serde::Deserialize;
use serde_json::json;
use wasm_bindgen::prelude::{JsValue, wasm_bindgen};

use crate::Org;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RegistrySource {
    path: String,
    source: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ContractEvaluationRequest {
    #[serde(default)]
    registry_sources: Vec<RegistrySource>,
    #[serde(default)]
    contract_ids: Vec<String>,
    #[serde(default)]
    apply_registry_contracts: bool,
    source_path: Option<String>,
}

pub(crate) fn validate_source(source: &str, source_path: Option<&str>) -> String {
    let document = orgize::Org::parse(source).document();
    let validation = orgize::ast::validate_contract_source(&document, source_path.map(Path::new));
    let diagnostics = validation
        .diagnostics
        .iter()
        .map(|diagnostic| {
            json!({
                "code": diagnostic.code,
                "path": diagnostic.path,
                "contractId": diagnostic.contract_id,
                "message": diagnostic.message,
            })
        })
        .collect::<Vec<_>>();

    json!({
        "schemaVersion": 1,
        "path": source_path.unwrap_or("<memory>"),
        "valid": validation.is_valid(),
        "contractCount": validation.registry.contracts.len(),
        "diagnostics": diagnostics,
    })
    .to_string()
}

fn assert_valid_source(source: &str, source_path: Option<&str>) -> Result<(), String> {
    let document = orgize::Org::parse(source).document();
    let validation = orgize::ast::validate_contract_source(&document, source_path.map(Path::new));
    if validation.is_valid() {
        return Ok(());
    }
    Err(invalid_contract_source_error(
        source_path.unwrap_or("<memory>"),
        &validation.diagnostics,
    ))
}

fn invalid_contract_source_error(
    path: &str,
    diagnostics: &[orgize::ast::OrgContractSourceDiagnostic],
) -> String {
    let details = diagnostics
        .iter()
        .map(|diagnostic| format!("{} {}", diagnostic.code, diagnostic.message))
        .collect::<Vec<_>>()
        .join("; ");
    format!("invalid contract source `{path}`: {details}")
}

struct ContractEvaluationOutcome {
    source_path: String,
    failed: usize,
    evaluations: Vec<orgize::ast::OrgContractEvaluation>,
}

fn evaluate_typed(source: &str, request_json: &str) -> Result<ContractEvaluationOutcome, String> {
    let request: ContractEvaluationRequest = serde_json::from_str(request_json)
        .map_err(|error| format!("invalid contract request: {error}"))?;
    let mut contracts = Vec::new();
    let mut contract_origins = std::collections::BTreeMap::new();
    let mut source_diagnostics = Vec::new();
    for registry_source in request.registry_sources {
        let registry_document = orgize::Org::parse(&registry_source.source).document();
        let validation = orgize::ast::validate_contract_source(
            &registry_document,
            Some(Path::new(&registry_source.path)),
        );
        source_diagnostics.extend(validation.diagnostics);
        for contract in validation.registry.contracts {
            if let Some(previous_path) =
                contract_origins.insert(contract.id.clone(), registry_source.path.clone())
            {
                return Err(format!(
                    "duplicate contract id `{}` in `{previous_path}` and `{}`",
                    contract.id, registry_source.path
                ));
            }
            contracts.push(contract);
        }
    }
    if !source_diagnostics.is_empty() {
        let path = source_diagnostics
            .first()
            .and_then(|diagnostic| diagnostic.path.as_deref())
            .unwrap_or("<memory>");
        return Err(invalid_contract_source_error(path, &source_diagnostics));
    }
    let registry = orgize::ast::OrgContractRegistry::new(contracts);
    let document = orgize::Org::parse(source).document();
    let source_path = request.source_path.as_deref().unwrap_or("<memory>");
    let mut contract_references = request
        .contract_ids
        .iter()
        .map(|reference| {
            orgize::ast::parse_contract_reference_from_source(
                reference,
                request.source_path.as_deref().map(Path::new),
            )
        })
        .collect::<Vec<_>>();
    if request.apply_registry_contracts {
        contract_references.extend(
            registry
                .contracts
                .iter()
                .filter(|contract| contract.scope == orgize::ast::OrgContractScope::Document)
                .map(|contract| orgize::ast::OrgContractReference {
                    raw: contract.id.clone(),
                    path: None,
                    contract_id: Some(contract.id.clone()),
                }),
        );
    }
    for raw in document
        .properties
        .iter()
        .filter(|property| property.key.eq_ignore_ascii_case("CONTRACT_ORG"))
        .map(|property| property.value.trim())
        .chain(
            document
                .metadata
                .iter()
                .filter(|keyword| keyword.key.eq_ignore_ascii_case("CONTRACT_ORG"))
                .map(|keyword| keyword.value.trim()),
        )
        .filter(|raw| !raw.is_empty())
    {
        let reference = orgize::ast::parse_contract_reference_from_source(
            raw,
            request.source_path.as_deref().map(Path::new),
        );
        if !reference.is_path_qualified_org_link() {
            return Err(format!(
                "{source_path}: CONTRACT-E010 CONTRACT_ORG `{raw}` must be a path-qualified Org link such as `[[file:../contracts/document.org][contract.id]]`"
            ));
        }
        contract_references.push(reference);
    }
    contract_references.sort_by(|left, right| left.raw.cmp(&right.raw));
    contract_references.dedup_by(|left, right| left.raw == right.raw);

    let context = orgize::ast::OrgContractEvaluationContext::with_source_path(source_path);
    let mut evaluations = Vec::new();
    for reference in contract_references {
        let raw = reference.raw.clone();
        let contract = registry.resolve(&reference).ok_or_else(|| {
            format!("{source_path}: CONTRACT_ORG `{raw}` was not found in the loaded registry")
        })?;
        if contract.scope != orgize::ast::OrgContractScope::Document {
            return Err(format!(
                "{source_path}: CONTRACT_ORG `{raw}` has subtree scope and cannot be used as a document contract"
            ));
        }
        if contract.assertions.is_empty() {
            return Err(format!(
                "{source_path}: CONTRACT_ORG `{raw}` has no valid assertions"
            ));
        }
        evaluations.push(orgize::ast::evaluate_org_contract_with_context(
            &document,
            contract,
            orgize::ast::OrgContractEvaluationScope::document(),
            &context,
        ));
    }

    let failed = evaluations
        .iter()
        .flat_map(|evaluation| &evaluation.assertions)
        .filter(|assertion| assertion.status.is_failed())
        .count();
    Ok(ContractEvaluationOutcome {
        source_path: source_path.to_string(),
        failed,
        evaluations,
    })
}

pub(crate) fn evaluate(source: &str, request_json: &str) -> Result<String, String> {
    let outcome = evaluate_typed(source, request_json)?;
    let evaluations = orgize::ast::org_contract_evaluations_to_json_value(&outcome.evaluations);
    Ok(json!({
        "schemaVersion": 1,
        "path": outcome.source_path,
        "failed": outcome.failed,
        "evaluations": evaluations,
    })
    .to_string())
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ContractValidationRequest {
    #[serde(default)]
    required_contract_ids: Vec<String>,
    source_path: Option<String>,
}

fn validate_document(source: &str, request_json: &str) -> Result<(), String> {
    let request: ContractValidationRequest = serde_json::from_str(request_json)
        .map_err(|error| format!("invalid contract validation request: {error}"))?;
    let outcome = evaluate_typed(source, request_json)?;
    let source_path = request.source_path.as_deref().unwrap_or("<memory>");

    for required_contract_id in request.required_contract_ids {
        let is_present = outcome
            .evaluations
            .iter()
            .any(|evaluation| evaluation.contract_id == required_contract_id);
        if !is_present {
            return Err(format!(
                "{source_path}: CONTRACT-E011 missing path-qualified CONTRACT_ORG reference to `{required_contract_id}`"
            ));
        }
    }

    if outcome.failed > 0 {
        let failures = outcome
            .evaluations
            .iter()
            .flat_map(|evaluation| {
                evaluation
                    .assertions
                    .iter()
                    .filter(|assertion| assertion.status.is_failed())
                    .map(|assertion| {
                        format!(
                            "{}:{} actual={} expected={:?}",
                            evaluation.contract_id,
                            assertion.assertion_id,
                            assertion.actual_count,
                            assertion.expectation
                        )
                    })
            })
            .collect::<Vec<_>>()
            .join(", ");
        return Err(format!(
            "{source_path}: CONTRACT-E012 document contracts failed {} assertion(s): {failures}",
            outcome.failed,
        ));
    }
    Ok(())
}

#[wasm_bindgen]
impl Org {
    #[wasm_bindgen(js_name = validateContractSource)]
    pub fn validate_contract_source(&self, source_path: Option<String>) -> Result<(), JsValue> {
        assert_valid_source(&self.source, source_path.as_deref()).map_err(|error| error.into())
    }

    #[wasm_bindgen(js_name = validateContracts)]
    pub fn validate_contracts(&self, request_json: &str) -> Result<(), JsValue> {
        validate_document(&self.source, request_json).map_err(|error| error.into())
    }
}

#[cfg(test)]
#[path = "../tests/unit/dto_contracts/mod.rs"]
mod tests;
