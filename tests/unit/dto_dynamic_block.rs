use orgize_wasm::Org;
use serde_json::Value;

#[test]
fn wasm_dynamic_blocks_contract_exposes_registry_records() {
    let org = Org::parse(
        r#"#+BEGIN: clocktable :scope file :maxlevel 1
| Headline | Time |
#+END:

#+BEGIN: columnview :id local :format "%ITEM %TODO"
| ITEM | TODO |
#+END:

#+BEGIN: custom :foo bar
#+END:
"#,
    );

    let dynamic_blocks: Value =
        serde_json::from_str(&org.dynamic_blocks_json()).expect("dynamic block JSON should parse");
    let records = dynamic_blocks["records"]
        .as_array()
        .expect("dynamic block records should be an array");

    assert_eq!(dynamic_blocks["schemaVersion"], 1);
    assert_eq!(records.len(), 3);
    assert_eq!(records[0]["name"], "clocktable");
    assert_eq!(records[0]["writer"], "clocktable");
    assert_eq!(records[0]["parameters"][0]["key"], "scope");
    assert_eq!(records[0]["contentState"], "existingOutput");
    assert_eq!(records[0]["contentLineCount"], 1);
    assert_eq!(records[1]["writer"], "columnview");
    assert_eq!(records[1]["parameters"][1]["value"], "\"%ITEM %TODO\"");
    assert_eq!(records[2]["writer"], "unknown");
    assert_eq!(records[2]["contentState"], "empty");
}
