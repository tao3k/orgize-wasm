use orgize_wasm::Org;
use serde_json::{Value, json};

#[test]
fn wasm_source_blocks_contract_exposes_literate_references() {
    let org = Org::parse(
        r#"#+NAME: load_data
#+begin_src sh :noweb-ref setup
echo load
#+end_src

#+begin_src sh
<<load_data>>
<<setup()>>
<<missing>>
#+end_src

#+CALL: load_data()
Inline call_load_data() and call_missing_inline().
"#,
    );
    let payload: Value =
        serde_json::from_str(&org.source_blocks_json()).expect("source blocks JSON should parse");
    let references = payload["references"].as_array().expect("references");

    assert_eq!(references.len(), 6);
    assert_eq!(
        references
            .iter()
            .map(|reference| json!({
                "kind": reference["kind"],
                "target": reference["target"],
                "resolved": reference["resolved"],
            }))
            .collect::<Vec<_>>(),
        [
            json!({"kind": "noweb", "target": "load_data", "resolved": true}),
            json!({"kind": "noweb", "target": "setup", "resolved": true}),
            json!({"kind": "noweb", "target": "missing", "resolved": false}),
            json!({"kind": "babelCall", "target": "load_data", "resolved": true}),
            json!({"kind": "inlineCall", "target": "load_data", "resolved": true}),
            json!({"kind": "inlineCall", "target": "missing_inline", "resolved": false}),
        ]
    );
}
