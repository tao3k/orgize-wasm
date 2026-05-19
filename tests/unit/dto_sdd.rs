use orgize_wasm::Org;
use serde_json::Value;

#[test]
fn wasm_sdd_contract_exposes_org_native_architecture_nodes() {
    let org = Org::parse(
        r#"* Browser parser SDD :sdd:
:PROPERTIES:
:ID: 018f3f9c-8d3e-7b2a-9c91-4f5b2e7a2c11
:SDD_KIND: system
:SDD_STATUS: review
:SDD_CONCERN: Browser consumers need source-grounded parser semantics.
:END:
** WASM projection capability :sdd:
:PROPERTIES:
:ID: 018f3f9c-7a91-73b4-b3f2-12c4c4d80d77
:SDD_KIND: capability
:SDD_PARENT: [[id:018f3f9c-8d3e-7b2a-9c91-4f5b2e7a2c11][Browser parser SDD]]
:SDD_CAPABILITY: typed-browser-projections
:SDD_STATUS: accepted
:END:
"#,
    );

    let sdd: Value = serde_json::from_str(&org.sdd_json()).expect("SDD JSON should parse");
    let records = sdd["records"]
        .as_array()
        .expect("SDD records should be an array");

    assert_eq!(sdd["schemaVersion"], 1);
    assert_eq!(records.len(), 2);
    assert_eq!(records[0]["kind"], "system");
    assert_eq!(records[0]["kindKnown"], true);
    assert_eq!(records[0]["status"], "review");
    assert_eq!(
        records[0]["concern"],
        "Browser consumers need source-grounded parser semantics."
    );
    assert_eq!(records[1]["kind"], "capability");
    assert_eq!(records[1]["status"], "accepted");
    assert_eq!(
        records[1]["parent"]["targetId"],
        "018f3f9c-8d3e-7b2a-9c91-4f5b2e7a2c11"
    );
    assert_eq!(records[1]["parent"]["label"], "Browser parser SDD");
    assert_eq!(records[1]["capability"], "typed-browser-projections");

    let compact = org.sdd();
    assert!(compact.contains("[SDD] wasm-demo.org"));
    assert!(compact.contains("- system review: Browser parser SDD"));
    assert!(compact.contains("- capability accepted: WASM projection capability"));
}

#[test]
fn wasm_snapshot_contract_includes_sdd_projection() {
    let org = Org::parse(
        r#"* SDD root :sdd:
:PROPERTIES:
:ID: 018f3f9c-8d3e-7b2a-9c91-4f5b2e7a2c11
:SDD_KIND: system
:SDD_CONCERN: Snapshot consumers need SDD in the first projection.
:END:
"#,
    );
    let snapshot: Value = serde_json::from_str(&org.snapshot_json(Some("sdd.org".to_string())))
        .expect("snapshot JSON should parse");

    assert_eq!(snapshot["schemaVersion"], 1);
    assert_eq!(snapshot["sdd"][0]["title"], "SDD root");
    assert_eq!(
        snapshot["sdd"][0]["id"],
        "018f3f9c-8d3e-7b2a-9c91-4f5b2e7a2c11"
    );
}
