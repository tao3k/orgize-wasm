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
    assert_eq!(snapshot["progressStats"][0]["title"], "2026");
    assert_eq!(snapshot["clockRollups"][0]["title"], "2026");
    assert_eq!(
        snapshot["clockRollups"][0]["subtreeClock"]["totalSeconds"],
        0
    );
    assert!(snapshot["dynamicBlocks"].as_array().unwrap().is_empty());
    assert_eq!(snapshot["propertyProfile"]["inheritance"], "all");
    assert_eq!(
        snapshot["propertyProfile"]["allowedValues"][0]["descriptorKey"],
        "VISIBILITY_ALL"
    );
    assert_eq!(snapshot["refileTargets"][0]["title"], "2026");
    assert_eq!(
        snapshot["refileTargets"][0]["receipts"][0]["spec"]["kind"],
        "level"
    );
    assert!(snapshot["clockTablePlans"].as_array().unwrap().is_empty());
    assert!(snapshot["clockIssues"].as_array().unwrap().is_empty());
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
fn wasm_org_elements_contract_exposes_tag_vocabulary_and_source_blocks() {
    let org = Org::parse(
        r#"#+TAGS: EMACS (e) COURSE (c) EXERCISE (ex) READ(r)
#+PYTHON: print("explicit host execution only")

* TODO Browser binding :EMACS:READ:
See [[https://example.test][example]] at <2026-05-19 Tue>.
- [X] done item
#+begin_src python :results output :var topic="org-elements"
print(topic)
#+end_src
"#,
    );

    let metadata: Value =
        serde_json::from_str(&org.metadata_json()).expect("metadata JSON should parse");
    assert_eq!(metadata["tagDefinitions"][0]["name"], "EMACS");
    assert_eq!(metadata["tagDefinitions"][0]["shortcut"], "e");
    assert_eq!(metadata["tagDefinitions"][2]["shortcut"], "ex");
    assert_eq!(metadata["tagDefinitions"][3]["shortcut"], "r");

    let payload: Value = serde_json::from_str(&org.org_elements_json()).expect("Org elements JSON");
    assert_eq!(payload["schemaVersion"], 1);
    assert_eq!(payload["sections"][0]["title"], "Browser binding");
    assert_eq!(payload["sections"][0]["todo"], "TODO");
    assert_eq!(payload["sections"][0]["tags"][0], "EMACS");
    let section_elements = payload["sections"][0]["elements"]
        .as_array()
        .expect("section elements");
    assert!(section_elements
        .iter()
        .any(|element| element["kind"] == "paragraph"));
    assert!(section_elements
        .iter()
        .any(|element| element["kind"] == "plain-list"));
    assert!(section_elements
        .iter()
        .any(|element| element["kind"] == "src-block" && element["language"] == "python"));
    assert!(payload["index"]
        .as_array()
        .expect("flat node index")
        .iter()
        .any(|node| node["category"] == "object"
            && node["kind"] == "link"
            && node["summary"]["path"] == "https://example.test"));
    let index_only: Value =
        serde_json::from_str(&org.org_elements_index_json()).expect("Org elements index JSON");
    assert_eq!(
        index_only.as_array().expect("index array").len(),
        payload["index"].as_array().expect("payload index").len()
    );
    assert_eq!(payload["sourceBlocks"][0]["language"], "python");
    assert_eq!(
        payload["sourceBlocks"][0]["normalizedHeaderArgs"]
            .as_array()
            .expect("normalized args")
            .iter()
            .find(|arg| arg["kind"] == "var")
            .and_then(|arg| arg["variable"]["name"].as_str()),
        Some("topic")
    );
}

#[test]
fn wasm_progress_stats_contract_exposes_agent_planning_rollups() {
    let org = Org::parse(
        r#"* TODO Parent [1/2] [50%]
:PROPERTIES:
:Effort: 1:00
:ORDERED: t
:END:
- [X] done item
- [ ] open item
** DONE Child done
:PROPERTIES:
:Effort: 0:30
:END:
** TODO Child open
- [-] partial item
"#,
    );
    let progress_stats: Value =
        serde_json::from_str(&org.progress_stats_json()).expect("progress stats JSON should parse");
    let records = progress_stats["records"]
        .as_array()
        .expect("progress stats records should be an array");

    assert_eq!(progress_stats["schemaVersion"], 1);
    assert_eq!(records.len(), 3);
    assert_eq!(records[0]["title"], "Parent [1/2] [50%]");
    assert_eq!(records[0]["todo"], "todo");
    assert_eq!(records[0]["descendantTodos"]["total"], 2);
    assert_eq!(records[0]["descendantTodos"]["done"], 1);
    assert_eq!(records[0]["descendantTodos"]["open"], 1);
    assert_eq!(records[0]["checkboxes"]["total"], 3);
    assert_eq!(records[0]["checkboxes"]["checked"], 1);
    assert_eq!(records[0]["checkboxes"]["unchecked"], 1);
    assert_eq!(records[0]["checkboxes"]["partial"], 1);
    assert_eq!(records[0]["statisticCookies"][0]["kind"], "fraction");
    assert_eq!(records[0]["statisticCookies"][1]["percent"], 50);
    assert_eq!(records[0]["effort"]["local"]["raw"], "1:00");
    assert_eq!(records[0]["effort"]["subtreeTotalSeconds"], 5_400);
    assert_eq!(records[0]["dependencies"][0]["kind"], "openDescendantTodo");
    assert_eq!(records[0]["dependencies"][1]["kind"], "openCheckbox");
    assert_eq!(records[0]["dependencies"][2]["kind"], "orderedProperty");
    assert_eq!(records[1]["title"], "Child done");
    assert!(records[1]["dependencies"].as_array().unwrap().is_empty());
}

#[test]
fn wasm_task_blockers_contract_exposes_ordered_sibling_evidence() {
    let org = Org::parse(
        r#"* TODO Project
:PROPERTIES:
:ORDERED: t
:END:
** TODO First
SCHEDULED: <2026-05-15 Fri>
** TODO Second
SCHEDULED: <2026-05-15 Fri>
"#,
    );
    let task_blockers: Value =
        serde_json::from_str(&org.task_blockers_json()).expect("task blockers JSON should parse");
    let records = task_blockers["records"]
        .as_array()
        .expect("task blocker records should be an array");

    assert_eq!(task_blockers["schemaVersion"], 1);
    assert_eq!(records.len(), 1);
    assert_eq!(records[0]["kind"], "orderedPreviousSibling");
    assert_eq!(records[0]["blocked"]["title"], "Second");
    assert_eq!(records[0]["blocked"]["todoState"], "todo");
    assert_eq!(records[0]["blocker"]["title"], "First");
    assert_eq!(records[0]["parent"]["title"], "Project");
    assert_eq!(
        records[0]["parent"]["orderedPropertySource"]["start"]["line"],
        3
    );

    let agenda_view_request =
        r#"{"start":{"year":2026,"month":5,"day":15},"end":{"year":2026,"month":5,"day":15}}"#;
    let agenda_view: Value = serde_json::from_str(
        &org.agenda_view_json(agenda_view_request)
            .expect("agenda view JSON should render"),
    )
    .expect("agenda view JSON should parse");
    let cards = agenda_view["cards"]
        .as_array()
        .expect("agenda view cards should be an array");
    let second = cards
        .iter()
        .find(|card| card["title"] == "Second")
        .expect("Second agenda card should exist");

    assert_eq!(second["blockers"][0]["kind"], "orderedPreviousSibling");
    assert_eq!(second["blockers"][0]["blocker"]["title"], "First");
    assert!(second["receipts"]
        .as_array()
        .unwrap()
        .iter()
        .any(|receipt| receipt["kind"] == "blockedByOrderedSibling"));
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
    assert_eq!(sparse_tree["totalCandidates"], 2);
    assert_eq!(cards.len(), 1);
    assert_eq!(cards[0]["title"], "Agent memory");
    assert_eq!(cards[0]["priority"]["effective"], "A");
    assert_eq!(cards[0]["matches"][0]["kind"], "tag");
    assert_eq!(cards[0]["receipts"][0]["kind"], "candidate");
    assert!(cards[0]["receipts"]
        .as_array()
        .unwrap()
        .iter()
        .any(|receipt| receipt["kind"] == "matchExpressionMatched"));
    assert!(cards[0]["receipts"]
        .as_array()
        .unwrap()
        .iter()
        .any(|receipt| receipt["kind"] == "textMatched"));
    assert_eq!(cards[0]["links"][0]["path"], "id:old-memory");
    assert!(cards[0]["preview"]
        .as_str()
        .unwrap()
        .contains("current memory"));

    let explained: Value = serde_json::from_str(
        &org.sparse_tree_explain_json(
            Some("memory.org".to_string()),
            Some(r#"+agent+TODO="TODO""#.to_string()),
            Some("current memory".to_string()),
            Some(false),
        )
        .expect("sparse-tree explain query should be valid"),
    )
    .expect("sparse-tree explain JSON should parse");
    let skipped = explained["skipped"]
        .as_array()
        .expect("sparse-tree skipped rows should be an array");
    assert_eq!(skipped.len(), 1);
    assert_eq!(skipped[0]["title"], "Old memory");
    assert_eq!(skipped[0]["reason"], "archived");
    assert_eq!(skipped[0]["receipts"][1]["kind"], "skippedArchived");
}
