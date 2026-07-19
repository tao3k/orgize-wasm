use crate::dto_contracts::{assert_valid_source, validate_document};

const DOCUMENT_CONTRACT: &str = "* Document policy\n:PROPERTIES:\n:CONTRACT_ID: tao3k.document\n:CONTRACT_KIND: org-elements\n:CONTRACT_SCOPE: document\n:END:\n** Body\n:PROPERTIES:\n:ASSERT_ID: require-paragraph\n:ASSERT_SEVERITY: error\n:END:\n#+begin_src org-elements-selector\n(:org-element (:type paragraph))\n#+end_src\n";

fn validation_request() -> String {
    serde_json::json!({
        "registrySources": [{
            "path": "docs/contracts/document-policy.org",
            "source": DOCUMENT_CONTRACT
        }],
        "requiredContractIds": ["tao3k.document"],
        "sourcePath": "content/document.org"
    })
    .to_string()
}

#[test]
fn rejects_a_plain_org_document_as_a_contract_source() {
    let error = assert_valid_source("#+TITLE: Notes\nBody\n", Some("docs/contracts/notes.org"))
        .unwrap_err();

    assert!(error.contains("CONTRACT-E001"));
    assert!(error.contains("docs/contracts/notes.org"));
}

#[test]
fn requires_the_declared_contract_binding() {
    let error = validate_document("#+TITLE: Document\nBody\n", &validation_request()).unwrap_err();

    assert!(error.contains("CONTRACT-E011"));
    assert!(error.contains("tao3k.document"));
}

#[test]
fn accepts_a_resolved_file_link() {
    validate_document(
        "#+TITLE: Document\n#+CONTRACT_ORG: [[file:../docs/contracts/document-policy.org][tao3k.document]]\nBody\n",
        &validation_request(),
    )
    .unwrap();
}

#[test]
fn rejects_failed_assertions() {
    let error = validate_document(
        "#+TITLE: Document\n#+CONTRACT_ORG: [[file:../docs/contracts/document-policy.org][tao3k.document]]\n",
        &validation_request(),
    )
    .unwrap_err();

    assert!(error.contains("CONTRACT-E012"));
}
