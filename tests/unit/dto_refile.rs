use orgize_wasm::Org;
use serde_json::{json, Value};

#[test]
fn wasm_refile_contract_exposes_targets_and_non_executing_plan() {
    let org = Org::parse(
        r#"#+TITLE: Work Notes
#+TODO: TODO WAIT | DONE
* Inbox
** TODO Capture
:PROPERTIES:
:ID: task-1
:END:
* Projects
** TODO Project A :project:
** WAIT Waiting Room
"#,
    );
    let targets_request = json!({
        "sourceFile": "notes/work.org",
        "outlinePathMode": "file",
        "specs": [
            { "kind": "tag", "value": "project" },
            { "kind": "todo", "value": "WAIT" }
        ]
    })
    .to_string();
    let targets: Value = serde_json::from_str(
        &org.refile_targets_json(Some(targets_request))
            .expect("refile targets JSON should render"),
    )
    .expect("refile targets JSON should parse");

    assert_eq!(targets["schemaVersion"], 1);
    assert_eq!(targets["outlinePathMode"], "file");
    assert_eq!(targets["specs"][0]["kind"], "tag");
    assert_eq!(targets["targets"].as_array().unwrap().len(), 2);
    assert_eq!(
        targets["targets"][0]["display"],
        "work.org/Projects/Project A"
    );
    assert_eq!(targets["targets"][1]["receipts"][0]["spec"]["kind"], "todo");

    let plan_request = json!({
        "sourceFile": "notes/work.org",
        "sourceOutlinePath": ["Inbox", "Capture"],
        "targetOutlinePath": ["Projects", "Project A"],
        "action": "copy",
        "insertPosition": "firstChild"
    })
    .to_string();
    let plan: Value = serde_json::from_str(
        &org.refile_plan_json(&plan_request)
            .expect("refile plan JSON should render"),
    )
    .expect("refile plan JSON should parse");

    assert_eq!(plan["schemaVersion"], 1);
    assert_eq!(plan["plan"]["action"], "copy");
    assert_eq!(plan["plan"]["insertPosition"], "firstChild");
    assert_eq!(plan["plan"]["source"]["title"], "Capture");
    assert_eq!(plan["plan"]["target"]["title"], "Project A");
    assert_eq!(plan["plan"]["warnings"][0]["kind"], "copyMayDuplicateId");
    assert!(plan["plan"]["receipts"]
        .as_array()
        .unwrap()
        .iter()
        .any(|receipt| receipt["kind"] == "nonMutating"));

    let create_request = json!({
        "sourceOutlinePath": ["Inbox", "Capture"],
        "targetOutlinePath": ["Projects", "Project B"],
        "parentCreation": "confirm"
    })
    .to_string();
    let create_plan: Value = serde_json::from_str(
        &org.refile_plan_json(&create_request)
            .expect("refile parent creation plan JSON should render"),
    )
    .expect("refile parent creation plan JSON should parse");

    assert_eq!(create_plan["plan"]["parentCreation"], "confirm");
    assert!(create_plan["plan"]["target"].is_null());
    assert_eq!(
        create_plan["plan"]["createdTarget"]["existingParent"]["title"],
        "Projects"
    );
    assert_eq!(
        create_plan["plan"]["createdTarget"]["targetOutlinePath"][1],
        "Project B"
    );
    assert_eq!(create_plan["plan"]["createdTarget"]["nodes"][0]["level"], 2);
    assert_eq!(
        create_plan["plan"]["createdTarget"]["nodes"][0]["display"],
        "Projects/Project B"
    );
    assert_eq!(
        create_plan["plan"]["createdTarget"]["requiresConfirmation"],
        true
    );
    assert!(create_plan["plan"]["receipts"]
        .as_array()
        .unwrap()
        .iter()
        .any(|receipt| receipt["kind"] == "parentCreationRequiresConfirmation"));
}
