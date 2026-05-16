use orgize_wasm::Org;
use serde_json::{json, Value};

#[test]
fn wasm_capture_plan_contract_renders_agent_native_org_preview() {
    let org = Org::parse("");
    let request = json!({
        "kind": "articleNote",
        "title": "Agent capture protocol",
        "body": "Capture should be planned for Agent consumption.",
        "target": {
            "kind": "datetree",
            "sourceFile": "notes/inbox.org",
            "date": { "year": 2026, "month": 5, "day": 16 }
        },
        "source": {
            "kind": "article",
            "actor": "user",
            "uri": "https://example.test/agent-capture",
            "label": "Agent capture article"
        },
        "capturedAt": {
            "year": 2026,
            "month": 5,
            "day": 16,
            "hour": 10,
            "minute": 24
        },
        "tags": ["reading", "memory_candidate"],
        "properties": [
            { "key": "decision impact", "value": "high" }
        ],
        "quote": "Selected source excerpt.",
        "links": [
            { "url": "id:related-node", "label": "related node" }
        ],
        "memoryPolicy": "candidate"
    })
    .to_string();

    let response: Value = serde_json::from_str(
        &org.capture_plan_json(&request)
            .expect("capture plan JSON should render"),
    )
    .expect("capture plan JSON should parse");

    assert_eq!(response["schemaVersion"], 1);
    assert_eq!(response["plan"]["target"]["kind"], "datetree");
    assert_eq!(response["plan"]["target"]["sourceFile"], "notes/inbox.org");
    assert_eq!(
        response["plan"]["target"]["date"],
        json!({ "year": 2026, "month": 5, "day": 16 })
    );
    assert!(response["plan"]["orgEntry"]
        .as_str()
        .unwrap()
        .contains(":CAPTURE_KIND: articleNote"));
    assert!(response["plan"]["orgEntry"]
        .as_str()
        .unwrap()
        .contains("[[https://example.test/agent-capture][Agent capture article]]"));
    assert_eq!(response["plan"]["application"]["action"], "insertOrgEntry");
    assert_eq!(
        response["plan"]["application"]["target"]["sourceFile"],
        "notes/inbox.org"
    );
    assert!(response["plan"]["application"]["preconditions"]
        .as_array()
        .unwrap()
        .iter()
        .any(|precondition| precondition["kind"] == "writeLock"));
    assert!(response["plan"]["receipts"]
        .as_array()
        .unwrap()
        .iter()
        .any(|receipt| receipt["kind"] == "nonMutating"));
    assert!(response["plan"]["receipts"]
        .as_array()
        .unwrap()
        .iter()
        .any(|receipt| receipt["kind"] == "applicationPlan"));
    assert_eq!(
        response["plan"]["warnings"][0]["kind"],
        "sanitizedPropertyKey"
    );
}
