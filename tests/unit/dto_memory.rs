use orgize_wasm::Org;
use serde_json::Value;

#[test]
fn wasm_memory_contract_exposes_records_cards_evidence_and_authority() {
    let org = Org::parse(
        r#"#+TODO: TODO NEXT WAIT | DONE
* NEXT Active agent memory :record:memory:ATTACH:
SCHEDULED: <2026-05-20 Wed ++1w>
:PROPERTIES:
:ID: active-memory
:STYLE: habit
:END:
See [[id:background-memory][background memory]].
:LOGBOOK:
- State "NEXT" from "TODO" [2026-05-19 Tue]
:END:

* DONE Historical agent memory :record:memory:
CLOSED: [2026-05-18 Mon]
:PROPERTIES:
:ID: historical-memory
:END:

* Background project fact :record:
:PROPERTIES:
:CUSTOM_ID: background-memory
:END:
"#,
    );

    let memory: Value =
        serde_json::from_str(&org.memory_json(None).expect("memory JSON should render"))
            .expect("memory JSON should parse");
    let records = memory["records"]
        .as_array()
        .expect("records should be an array");
    let cards = memory["cards"]
        .as_array()
        .expect("cards should be an array");

    assert_eq!(memory["schemaVersion"], 1);
    assert_eq!(records.len(), 3);
    assert_eq!(memory["stats"]["totalRecords"], 3);
    assert_eq!(memory["stats"]["currentRecords"], 1);
    assert_eq!(memory["stats"]["closedRecords"], 1);
    assert_eq!(memory["stats"]["backgroundRecords"], 1);
    assert_eq!(cards.len(), 3);
    assert_eq!(cards[0]["decision"]["code"], "MEM001");
    assert_eq!(cards[0]["decision"]["severity"], "action");
    assert_eq!(cards[0]["decision"]["kind"], "current");
    assert_eq!(cards[0]["title"], "Active agent memory");
    assert_eq!(cards[0]["todoState"], "todo");
    assert!(
        cards[0]["evidence"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["kind"]["code"] == "scheduled")
    );
    assert!(
        cards[0]["authority"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["kind"] == "temporal")
    );
    assert!(
        memory["evidenceKinds"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["code"] == "identity")
    );
    assert!(
        memory["authorityKinds"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["code"] == "background")
    );
}

#[test]
fn wasm_memory_projection_supports_profile_filters() {
    let org = Org::parse(
        r#"#+TODO: TODO NEXT WAIT | DONE
* NEXT Visible memory :memory:
* DONE Hidden memory :memory:
CLOSED: [2026-05-18 Mon]
"#,
    );

    let memory: Value = serde_json::from_str(
        &org.memory_json(Some(
            r#"{"requiredTags":["memory"],"includeClosed":false}"#.to_string(),
        ))
        .expect("memory JSON should render"),
    )
    .expect("memory JSON should parse");
    let records = memory["records"]
        .as_array()
        .expect("records should be an array");

    assert_eq!(records.len(), 1);
    assert_eq!(records[0]["title"], "Visible memory");
    assert_eq!(records[0]["state"], "current");
}

#[test]
fn wasm_snapshot_contract_includes_memory_projection() {
    let org = Org::parse(
        r#"#+TODO: TODO NEXT WAIT | DONE
* NEXT Snapshot memory :memory:
:PROPERTIES:
:ID: snapshot-memory
:END:
"#,
    );
    let snapshot: Value = serde_json::from_str(&org.snapshot_json(Some("memory.org".to_string())))
        .expect("snapshot JSON should parse");

    assert_eq!(snapshot["schemaVersion"], 1);
    assert_eq!(snapshot["memory"]["records"][0]["title"], "Snapshot memory");
    assert_eq!(snapshot["memory"]["cards"][0]["decision"]["code"], "MEM001");
}
