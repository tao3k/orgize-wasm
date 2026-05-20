use orgize_wasm::Org;
use serde_json::Value;

#[test]
fn wasm_runtime_metadata_contract_exposes_mobile_feed_and_timer_metadata() {
    let org = Org::parse(
        r#"#+READONLY
#+ALLPRIORITIES: A B C
* [[file:notes.org][notes.org]]
* Feed Inbox
:FEEDSTATUS:
(("guid-1" t "hash-1"))
:END:
0:00:12 Intro notes
* Mobile edit :FLAGGED:
:PROPERTIES:
:ORIGINAL_ID: id:mobile-item
:MOBILE_NOTE: review
:END:
"#,
    );

    let runtime: Value = serde_json::from_str(&org.runtime_metadata_json())
        .expect("runtime metadata JSON should parse");

    assert_eq!(runtime["schemaVersion"], 1);
    assert_eq!(runtime["feeds"][0]["entryCount"], 1);
    assert_eq!(runtime["timers"][0]["totalSeconds"], 12);
    assert_eq!(runtime["mobile"]["readonly"].as_array().unwrap().len(), 1);
    assert_eq!(runtime["mobile"]["allPriorities"][0]["values"][0], "A");
    assert_eq!(runtime["mobile"]["indexLinks"][0]["file"], "notes.org");
    assert_eq!(
        runtime["mobile"]["flaggedSections"][0]["originalId"],
        "id:mobile-item"
    );
    assert_eq!(runtime["boundaries"][3]["kind"], "orgPersistCache");
}

#[test]
fn wasm_snapshot_contract_includes_runtime_metadata_projection() {
    let org = Org::parse("#+READONLY\n* [[file:notes.org][notes.org]]\n");
    let snapshot: Value = serde_json::from_str(&org.snapshot_json(Some("runtime.org".to_string())))
        .expect("snapshot JSON should parse");

    assert_eq!(snapshot["schemaVersion"], 1);
    assert_eq!(
        snapshot["runtimeMetadata"]["mobile"]["readonly"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    assert_eq!(
        snapshot["runtimeMetadata"]["mobile"]["indexLinks"][0]["file"],
        "notes.org"
    );
}
