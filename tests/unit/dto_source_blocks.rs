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

#+begin_src sh :var rows=load_data :var scoped=load_data(limit=1) :var literal=42 :var quoted="load_data" :var missing=missing_call()
echo "$rows"
#+end_src

#+CALL: load_data()
#+CALL: setup()
Inline call_load_data() and call_missing_inline().
"#,
    );
    let payload: Value =
        serde_json::from_str(&org.source_blocks_json()).expect("source blocks JSON should parse");
    let references = payload["references"].as_array().expect("references");

    assert_eq!(references.len(), 10);
    assert_eq!(
        references
            .iter()
            .map(|reference| json!({
                "kind": reference["kind"],
                "variable": reference["variable"],
                "target": reference["target"],
                "resolved": reference["resolved"],
            }))
            .collect::<Vec<_>>(),
        [
            json!({"kind": "noweb", "variable": null, "target": "load_data", "resolved": true}),
            json!({"kind": "noweb", "variable": null, "target": "setup", "resolved": true}),
            json!({"kind": "noweb", "variable": null, "target": "missing", "resolved": false}),
            json!({"kind": "headerVar", "variable": "rows", "target": "load_data", "resolved": true}),
            json!({"kind": "headerVar", "variable": "scoped", "target": "load_data", "resolved": true}),
            json!({"kind": "headerVar", "variable": "missing", "target": "missing_call", "resolved": false}),
            json!({"kind": "babelCall", "variable": null, "target": "load_data", "resolved": true}),
            json!({"kind": "babelCall", "variable": null, "target": "setup", "resolved": false}),
            json!({"kind": "inlineCall", "variable": null, "target": "load_data", "resolved": true}),
            json!({"kind": "inlineCall", "variable": null, "target": "missing_inline", "resolved": false}),
        ]
    );
}
