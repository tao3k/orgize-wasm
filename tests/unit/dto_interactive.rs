use orgize_wasm::Org;
use serde_json::Value;

const INTERACTIVE: &str = r#"
#+BEGIN_SRC org-contract :type agent-interactive
id: principle-pressure
method: choice
stage: presentation
group: PRINCIPLE
create: deferred
target: tao3k.principles
info: Choose the pressure that should be evaluated.
categories: 1=EVIDENCE,2=AUTHORITY,?=detail
details:
|n|id|contract|full|use-if|
|1|EVIDENCE|tao3k.evidence|Evidence before confidence|provenance is missing|
|2|AUTHORITY|tao3k.human-agency|Human agency|consequential authority is missing|
#+END_SRC
"#;

#[test]
fn wasm_projects_org_interactive_choices_as_stable_json() {
    let org = Org::parse(INTERACTIVE);
    let response: Value = serde_json::from_str(
        &org.org_interactive_json()
            .expect("valid Org-Interactive should project"),
    )
    .expect("Org-Interactive JSON should parse");

    assert_eq!(response["schemaVersion"], 1);
    assert_eq!(response["choices"][0]["id"], "principle-pressure");
    assert_eq!(response["choices"][0]["target"], "tao3k.principles");
    assert_eq!(response["choices"][0]["categories"][2]["detail"], true);
    assert_eq!(response["choices"][0]["entries"][1]["id"], "AUTHORITY");
    assert_eq!(
        response["choices"][0]["entries"][1]["useIf"],
        "consequential authority is missing"
    );
}
