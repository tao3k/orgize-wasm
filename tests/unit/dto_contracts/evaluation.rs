use crate::dto_contracts::evaluate;

#[test]
fn returns_an_empty_contract_receipt_without_bindings() {
    let value: serde_json::Value = serde_json::from_str(
        &evaluate("* Document\n", r#"{"sourcePath":"docs/document.org"}"#).unwrap(),
    )
    .unwrap();

    assert_eq!(value["schemaVersion"], 1);
    assert_eq!(value["path"], "docs/document.org");
    assert_eq!(value["failed"], 0);
    assert_eq!(value["evaluations"], serde_json::json!([]));
}

#[test]
fn resolves_file_keyword_bindings_through_the_loaded_registry() {
    let error = evaluate(
        "#+CONTRACT_ORG: [[file:../contracts/missing.org][missing.contract]]\n* Document\n",
        r#"{"sourcePath":"docs/document.org"}"#,
    )
    .unwrap_err();

    assert!(error.contains("CONTRACT_ORG `[[file:../contracts/missing.org][missing.contract]]`"));
    assert!(error.contains("was not found"));
}

#[test]
fn rejects_duplicate_contract_ids_across_registries() {
    let contract = "* Document contract\n:PROPERTIES:\n:CONTRACT_ID: duplicate.contract\n:CONTRACT_SCOPE: document\n:END:\n";
    let request = serde_json::json!({
        "registrySources": [
            {"path": "contracts/first.org", "source": contract},
            {"path": "contracts/second.org", "source": contract}
        ],
        "sourcePath": "docs/document.org"
    });

    let error = evaluate("* Document\n", &request.to_string()).unwrap_err();

    assert!(error.contains("duplicate contract id `duplicate.contract`"));
    assert!(error.contains("contracts/first.org"));
    assert!(error.contains("contracts/second.org"));
}

#[test]
fn evaluates_a_loaded_document_contract() {
    let contract = "* Document contract\n:PROPERTIES:\n:CONTRACT_ID: document.contract\n:CONTRACT_SCOPE: document\n:END:\n** Has document text\n:PROPERTIES:\n:ASSERT_ID: document.has-paragraph\n:ASSERT_SEVERITY: error\n:END:\n#+begin_src org-elements-selector\n(:org-element (:type paragraph))\n#+end_src\n";
    let request = serde_json::json!({
        "registrySources": [
            {"path": "contracts/document.org", "source": contract}
        ],
        "applyRegistryContracts": true,
        "sourcePath": "docs/document.org"
    });

    let value: serde_json::Value =
        serde_json::from_str(&evaluate("* Document\nBody\n", &request.to_string()).unwrap())
            .unwrap();

    assert_eq!(value["failed"], 0);
    assert_eq!(value["evaluations"][0]["contractId"], "document.contract");
    assert_eq!(value["evaluations"][0]["scope"]["kind"], "document");
    assert_eq!(
        value["evaluations"][0]["assertions"][0]["assertionId"],
        "document.has-paragraph"
    );
    assert_eq!(value["evaluations"][0]["assertions"][0]["status"], "passed");

    let failure: serde_json::Value =
        serde_json::from_str(&evaluate("* Document\n", &request.to_string()).unwrap()).unwrap();
    assert_eq!(failure["failed"], 1);
    assert_eq!(
        failure["evaluations"][0]["assertions"][0]["status"],
        "failed"
    );
}

#[test]
fn rejects_a_bound_contract_without_assertions() {
    let contract = "* Empty contract\n:PROPERTIES:\n:CONTRACT_ID: empty.contract\n:CONTRACT_SCOPE: document\n:END:\n";
    let request = serde_json::json!({
        "registrySources": [
            {"path": "contracts/empty.org", "source": contract}
        ],
        "contractIds": ["empty.contract"],
        "sourcePath": "docs/document.org"
    });

    let error = evaluate("* Document\n", &request.to_string()).unwrap_err();

    assert!(error.contains("CONTRACT-E003"));
    assert!(error.contains("CONTRACT_ID `empty.contract` contains no valid assertions"));
}

#[test]
fn reports_invalid_contract_source_before_registration() {
    let value: serde_json::Value = serde_json::from_str(&crate::dto_contracts::validate_source(
        "#+TITLE: Documentation policy\n* Rules\nPlain prose\n",
        Some("docs/contracts/document-policy.org"),
    ))
    .unwrap();

    assert_eq!(value["valid"], false);
    assert_eq!(value["contractCount"], 0);
    assert_eq!(value["diagnostics"][0]["code"], "CONTRACT-E001");
    assert_eq!(
        value["diagnostics"][0]["path"],
        "docs/contracts/document-policy.org"
    );
}

#[test]
fn rejects_invalid_registry_source_before_target_evaluation() {
    let request = serde_json::json!({
        "registrySources": [{
            "path": "docs/contracts/document-policy.org",
            "source": "#+TITLE: Documentation policy\n* Rules\nPlain prose\n"
        }],
        "sourcePath": "docs/document.org"
    });

    let error = evaluate("* Document\nBody\n", &request.to_string()).unwrap_err();

    assert!(error.contains("invalid contract source"));
    assert!(error.contains("CONTRACT-E001"));
    assert!(error.contains("docs/contracts/document-policy.org"));
}

#[test]
fn rejects_a_bare_contract_org_binding() {
    let request = serde_json::json!({
        "registrySources": [{
            "path": "docs/contracts/document-policy.org",
            "source": "* Document policy\n:PROPERTIES:\n:CONTRACT_ID: tao3k.document\n:CONTRACT_KIND: org-elements\n:CONTRACT_SCOPE: document\n:END:\n** Body\n:PROPERTIES:\n:ASSERT_ID: require-paragraph\n:ASSERT_SEVERITY: error\n:END:\n#+begin_src org-elements-selector\n(:org-element (:type paragraph))\n#+end_src\n"
        }],
        "sourcePath": "docs/brand/positioning.org"
    });

    let error = evaluate(
        "#+TITLE: Positioning\n#+CONTRACT_ORG: tao3k.document\n",
        &request.to_string(),
    )
    .unwrap_err();

    assert!(error.contains("CONTRACT-E010"));
    assert!(error.contains("path-qualified Org link"));
}

#[test]
fn resolves_a_relative_file_link_contract_binding() {
    let request = serde_json::json!({
        "registrySources": [{
            "path": "docs/contracts/document-policy.org",
            "source": "* Document policy\n:PROPERTIES:\n:CONTRACT_ID: tao3k.document\n:CONTRACT_KIND: org-elements\n:CONTRACT_SCOPE: document\n:END:\n** Body\n:PROPERTIES:\n:ASSERT_ID: require-paragraph\n:ASSERT_SEVERITY: error\n:END:\n#+begin_src org-elements-selector\n(:org-element (:type paragraph))\n#+end_src\n"
        }],
        "sourcePath": "docs/brand/positioning.org"
    });

    let receipt = evaluate(
        "#+TITLE: Positioning\n#+CONTRACT_ORG: [[file:../contracts/document-policy.org][tao3k.document]]\nBody\n",
        &request.to_string(),
    )
    .unwrap();
    let receipt: serde_json::Value = serde_json::from_str(&receipt).unwrap();

    assert_eq!(receipt["failed"], 0);
    assert_eq!(receipt["evaluations"][0]["contractId"], "tao3k.document");
}

#[test]
fn applies_document_contracts_from_multiple_sources() {
    let first = "* First contract\n:PROPERTIES:\n:CONTRACT_ID: first.contract\n:CONTRACT_SCOPE: document\n:END:\n** Has text\n:PROPERTIES:\n:ASSERT_ID: first.has-paragraph\n:END:\n#+begin_src org-elements-selector\n(:org-element (:type paragraph))\n#+end_src\n";
    let second = "* Second contract\n:PROPERTIES:\n:CONTRACT_ID: second.contract\n:CONTRACT_SCOPE: document\n:END:\n** Has text\n:PROPERTIES:\n:ASSERT_ID: second.has-paragraph\n:END:\n#+begin_src org-elements-selector\n(:org-element (:type paragraph))\n#+end_src\n";
    let request = serde_json::json!({
        "registrySources": [
            {"path": "contracts/first.org", "source": first},
            {"path": "contracts/second.org", "source": second}
        ],
        "applyRegistryContracts": true,
        "sourcePath": "docs/document.org"
    });

    let value: serde_json::Value =
        serde_json::from_str(&evaluate("* Document\nBody\n", &request.to_string()).unwrap())
            .unwrap();

    assert_eq!(value["evaluations"].as_array().unwrap().len(), 2);
    assert_eq!(value["evaluations"][0]["contractId"], "first.contract");
    assert_eq!(value["evaluations"][1]["contractId"], "second.contract");
}

#[test]
fn matches_required_document_keywords() {
    let contract = r#"* Document metadata
:PROPERTIES:
:CONTRACT_ID: document.metadata
:CONTRACT_SCOPE: document
:END:
** Has title
:PROPERTIES:
:ASSERT_ID: document.has-title
:END:
#+begin_src org-elements-selector
(:org-element (:type keyword :name TITLE))
#+end_src
** Has date
:PROPERTIES:
:ASSERT_ID: document.has-date
:END:
#+begin_src org-elements-selector
(:org-element (:type keyword :name DATE))
#+end_src
** Has author
:PROPERTIES:
:ASSERT_ID: document.has-author
:END:
#+begin_src org-elements-selector
(:org-element (:type keyword :name AUTHOR))
#+end_src
** Has file tags
:PROPERTIES:
:ASSERT_ID: document.has-filetags
:END:
#+begin_src org-elements-selector
(:org-element (:type keyword :name FILETAGS))
#+end_src
"#;
    let request = serde_json::json!({
        "registrySources": [
            {"path": "docs/document-contracts.org", "source": contract}
        ],
        "applyRegistryContracts": true,
        "sourcePath": "docs/document.org"
    });
    let source = "#+TITLE: Document\n#+DATE: 2026-07-16\n#+AUTHOR: Org Zhixing\n#+FILETAGS: :docs:\n* Document\nBody\n";

    let value: serde_json::Value =
        serde_json::from_str(&evaluate(source, &request.to_string()).unwrap()).unwrap();

    assert_eq!(value["failed"], 0);
    assert_eq!(
        value["evaluations"][0]["assertions"]
            .as_array()
            .unwrap()
            .len(),
        4
    );
}

#[test]
fn resolves_file_property_bindings_through_the_loaded_registry() {
    let error = evaluate(
        ":PROPERTIES:\n:CONTRACT_ORG: [[file:../contracts/missing.org][missing.contract]]\n:END:\n* Document\n",
        r#"{"sourcePath":"docs/document.org"}"#,
    )
    .unwrap_err();

    assert!(error.contains("CONTRACT_ORG `[[file:../contracts/missing.org][missing.contract]]`"));
    assert!(error.contains("was not found"));
}
