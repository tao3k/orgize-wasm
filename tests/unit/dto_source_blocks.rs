use orgize_wasm::Org;
use serde_json::{Value, json};

#[test]
fn wasm_source_blocks_contract_exposes_literate_references() {
    let org = Org::parse(
        r#"#+PROPERTY: header-args :session shared
#+PROPERTY: header-args:sh :dir ./scripts
#+NAME: load_data
#+begin_src sh :noweb-ref setup :tangle scripts/setup.sh :mkdirp yes :comments noweb :shebang #!/bin/sh :noweb strip-tangle
echo load
#+end_src

#+begin_src sh
<<load_data>>
<<setup()>>
<<missing>>
#+end_src

#+HEADER: :var topic="demo" :results file link output :file "reports/demo.txt" :file-desc "Demo output" :file-mode 0644 :output-dir public
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
    let records = payload["records"].as_array().expect("records");
    let header_arg_record = records
        .iter()
        .find(|record| record["value"].as_str() == Some("echo \"$rows\"\n"))
        .expect("record with affiliated header args");
    let header_args = header_arg_record["headerArgs"]
        .as_array()
        .expect("source block header args");

    assert!(header_args.iter().any(|arg| {
        arg["kind"] == "var"
            && arg["source"] == "explicit"
            && arg["variable"]["name"] == "topic"
            && arg["variable"]["assignment"] == "\"demo\""
    }));
    assert!(header_args.iter().any(|arg| {
        arg["key"] == "session" && arg["source"] == "explicit" && arg["value"] == "shared"
    }));
    assert!(header_args.iter().any(|arg| {
        arg["key"] == "dir" && arg["source"] == "explicit" && arg["value"] == "./scripts"
    }));
    assert!(header_args.iter().any(|arg| {
        arg["key"] == "results" && arg["source"] == "explicit" && arg["value"] == "file link output"
    }));
    assert!(header_args.iter().any(|arg| {
        arg["kind"] == "file"
            && arg["source"] == "explicit"
            && arg["value"] == "\"reports/demo.txt\""
    }));
    let result_options = header_arg_record["resultOptions"]
        .as_object()
        .expect("result options");
    assert_eq!(result_options["collection"], "file");
    assert_eq!(result_options["format"], "link");
    assert_eq!(result_options["handling"], "replace");
    assert_eq!(result_options["valueType"], "output");
    assert_eq!(result_options["file"]["target"], "reports/demo.txt");
    assert_eq!(result_options["file"]["description"], "Demo output");
    assert_eq!(result_options["file"]["fileMode"], "0644");
    assert_eq!(result_options["file"]["outputDir"], "public");

    let tangle = records[0]["tangle"].as_object().expect("tangle metadata");
    assert_eq!(tangle["target"], "scripts/setup.sh");
    assert_eq!(tangle["mkdirp"]["enabled"], true);
    assert_eq!(tangle["comments"]["mode"], "noweb");
    assert_eq!(tangle["shebang"], "#!/bin/sh");
    assert_eq!(tangle["noweb"]["mode"], "strip");

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
