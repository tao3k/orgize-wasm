use orgize_wasm::Org;
use serde_json::{json, Value};

#[test]
fn wasm_snapshot_contract_includes_agent_source_projections() {
    let org = Org::parse(
        r#"#+COLUMNS: %25ITEM(Task) %TODO
#+INCLUDE: "partials/header.org" :lines "2-5" :minlevel 2

#+NAME: demo
#+begin_src sh :results output
echo hi
#+end_src

#+RESULTS:
: hi

* 2026
** 2026-05 May
*** 2026-05-15 Friday
Capture row.
"#,
    );
    let snapshot: Value = serde_json::from_str(&org.snapshot_json(Some("demo.org".to_string())))
        .expect("snapshot JSON should parse");

    assert_eq!(snapshot["schemaVersion"], 1);
    assert_eq!(snapshot["sourceBlocks"][0]["kind"], "block");
    assert_eq!(snapshot["sourceBlocks"][0]["name"], "demo");
    assert_eq!(snapshot["sourceBlocks"][0]["result"]["value"], "hi");
    assert_eq!(
        snapshot["columnViews"][0]["scope"]["kind"],
        "documentKeyword"
    );
    assert_eq!(snapshot["columnViews"][0]["columns"][0]["property"], "ITEM");
    assert_eq!(
        snapshot["includeExpansion"][0]["lineSelection"]["kind"],
        "range"
    );
    assert_eq!(snapshot["includeExpansion"][0]["minLevel"], 2);
    assert_eq!(snapshot["datetree"][0]["date"], "2026-05-15");
}

#[test]
fn wasm_individual_projection_contracts_are_stable_json_objects() {
    let org = Org::parse(
        r#"#+INCLUDE: ./demo.rs src rust :lines "10-"
* TODO [#64] 2026
** 2026-05 May
*** 2026-05-15 Friday
Value src_sh[:exports both]{echo hi}
"#,
    );
    let outline: Value =
        serde_json::from_str(&org.outline_json()).expect("outline JSON should parse");
    let source_blocks: Value =
        serde_json::from_str(&org.source_blocks_json()).expect("source block JSON should parse");
    let column_views: Value =
        serde_json::from_str(&org.column_views_json()).expect("column view JSON should parse");
    let include_expansion: Value =
        serde_json::from_str(&org.include_expansion_json(Some("/site".to_string())))
            .expect("include expansion JSON should parse");
    let datetree: Value =
        serde_json::from_str(&org.datetree_json()).expect("datetree JSON should parse");

    assert_eq!(outline["schemaVersion"], 1);
    assert_eq!(outline["nodes"][0]["priority"]["effective"], "64");
    assert_eq!(outline["nodes"][0]["priority"]["score"], 3000);
    assert_eq!(outline["nodes"][0]["priority"]["rangeStatus"], "outOfRange");
    assert_eq!(outline["nodes"][0]["priority"]["profile"]["highest"], "A");
    assert_eq!(outline["nodes"][0]["priority"]["profile"]["default"], "B");
    assert_eq!(outline["nodes"][0]["priority"]["profile"]["lowest"], "C");
    assert_eq!(source_blocks["schemaVersion"], 1);
    assert_eq!(source_blocks["records"][0]["kind"], "inlineSource");
    assert_eq!(column_views["schemaVersion"], 1);
    assert_eq!(column_views["records"].as_array().unwrap().len(), 0);
    assert_eq!(include_expansion["schemaVersion"], 1);
    assert_eq!(
        include_expansion["entries"][0]["resolvedPath"],
        "/site/demo.rs"
    );
    assert_eq!(include_expansion["entries"][0]["mode"]["kind"], "source");
    assert_eq!(include_expansion["entries"][0]["mode"]["language"], "rust");
    assert_eq!(datetree["schemaVersion"], 1);
    assert_eq!(
        datetree["records"][0]["outlinePath"][2],
        "2026-05-15 Friday"
    );
}

#[test]
fn wasm_view_index_contract_is_compact_for_first_paint() {
    let org = Org::parse(
        r#"#+FILETAGS: :zhixing:user:
#+FILETAGS: :zhixing:user:
* Blog post :blog:
Body for the blog card.

* Plain parent
Body that should not enter the first-paint view index.

* TODO Agenda item :agenda:
SCHEDULED: <2026-05-15 Fri>
"#,
    );
    let view_index: Value =
        serde_json::from_str(&org.view_index_json(Some("demo.org".to_string())))
            .expect("view index JSON should parse");
    let records = view_index["records"]
        .as_array()
        .expect("view index records should be an array");

    assert_eq!(view_index["schemaVersion"], 1);
    assert_eq!(records.len(), 2);
    assert_eq!(records[0]["title"], "Blog post");
    assert_eq!(records[0]["bodyPreview"], "Body for the blog card.");
    assert_eq!(
        records[0]["effectiveTags"],
        json!(["zhixing", "user", "blog"])
    );
    assert!(records[0]["planning"].as_object().unwrap().is_empty());
    assert!(records[0]["rangeStart"].is_number());
    assert!(records[0].get("source").is_none());
    assert!(records[0].get("links").is_none());
    assert_eq!(records[1]["planning"]["scheduled"], "<2026-05-15 Fri>");
}

#[test]
fn wasm_sparse_tree_contract_exposes_agent_search_cards() {
    let org = Org::parse(
        r#"* TODO [#A] Agent memory :agent:
The current memory links to [[id:old-memory][old memory]].
* DONE Old memory :agent:ARCHIVE:
CLOSED: [2026-05-12 Tue]
"#,
    );
    let sparse_tree: Value = serde_json::from_str(
        &org.sparse_tree_json(
            Some("memory.org".to_string()),
            Some(r#"+agent+TODO="TODO"+PRIORITY="A""#.to_string()),
            Some("current memory".to_string()),
            Some(false),
        )
        .expect("sparse-tree query should be valid"),
    )
    .expect("sparse-tree JSON should parse");

    let cards = sparse_tree["cards"]
        .as_array()
        .expect("sparse-tree cards should be an array");
    assert_eq!(sparse_tree["schemaVersion"], 1);
    assert_eq!(cards.len(), 1);
    assert_eq!(cards[0]["title"], "Agent memory");
    assert_eq!(cards[0]["priority"]["effective"], "A");
    assert_eq!(cards[0]["matches"][0]["kind"], "tag");
    assert_eq!(cards[0]["links"][0]["path"], "id:old-memory");
    assert!(cards[0]["preview"]
        .as_str()
        .unwrap()
        .contains("current memory"));
}
