use orgize_wasm::Org;
use serde_json::Value;

#[test]
fn wasm_agenda_view_contract_exposes_sort_and_limit_receipts() {
    let org = Org::parse(
        r#"* TODO Morning
SCHEDULED: <2026-05-15 Fri 09:00>
* TODO Deadline
DEADLINE: <2026-05-15 Fri>
* TODO Afternoon
SCHEDULED: <2026-05-15 Fri 13:00>
"#,
    );
    let agenda_view_request = r#"{"start":{"year":2026,"month":5,"day":15},"end":{"year":2026,"month":5,"day":15},"limit":2}"#;
    let agenda_view: Value = serde_json::from_str(
        &org.agenda_view_json(agenda_view_request)
            .expect("agenda view JSON should render"),
    )
    .expect("agenda view JSON should parse");
    let cards = agenda_view["cards"]
        .as_array()
        .expect("agenda view cards should be an array");
    let skipped = agenda_view["skipped"]
        .as_array()
        .expect("agenda view skipped rows should be an array");

    assert_eq!(agenda_view["schemaVersion"], 1);
    assert_eq!(agenda_view["totalCandidates"], 3);
    assert_eq!(agenda_view["limit"], 2);
    assert!(agenda_view["sortStrategy"].as_array().unwrap().is_empty());
    assert_eq!(cards.len(), 2);
    assert_eq!(cards[0]["title"], "Deadline");
    assert_eq!(cards[0]["kind"], "deadline");
    assert!(cards[0]["urgency"]["total"].as_i64().unwrap() > 0);
    assert!(
        cards[0]["urgency"]["ingredients"]
            .as_array()
            .unwrap()
            .iter()
            .any(|ingredient| ingredient["kind"] == "deadline")
    );
    assert_eq!(cards[0]["sortKeys"][0]["key"], "displayDate");
    assert_eq!(cards[0]["receipts"][0]["kind"], "queryMatched");
    assert_eq!(cards[0]["receipts"][2]["kind"], "accepted");
    assert_eq!(skipped.len(), 1);
    assert_eq!(skipped[0]["title"], "Afternoon");
    assert_eq!(skipped[0]["reason"], "limit");
    assert_eq!(skipped[0]["limit"], 2);
    assert!(skipped[0]["urgency"]["total"].as_i64().unwrap() > 0);
    assert_eq!(skipped[0]["receipts"][2]["kind"], "skippedLimit");
}

#[test]
fn wasm_agenda_sort_and_block_contracts_are_named_agent_plans() {
    let org = Org::parse(
        r#"#+TODO: TODO WAITING | DONE
* TODO [#C] Low timed
SCHEDULED: <2026-05-15 Fri 09:00>
* WAITING [#A] High timed
SCHEDULED: <2026-05-15 Fri 09:00>
* TODO Untimed deadline
DEADLINE: <2026-05-15 Fri>
"#,
    );
    let sorted_request = r#"{"start":{"year":2026,"month":5,"day":15},"end":{"year":2026,"month":5,"day":15},"limit":2,"sortStrategy":[{"key":"time","direction":"up"},{"key":"priority","direction":"down"}]}"#;
    let sorted: Value = serde_json::from_str(
        &org.agenda_view_json(sorted_request)
            .expect("sorted agenda view JSON should render"),
    )
    .expect("sorted agenda view JSON should parse");

    assert_eq!(sorted["sortStrategy"][0]["key"], "time");
    assert_eq!(sorted["sortStrategy"][1]["direction"], "down");
    assert_eq!(sorted["cards"][0]["title"], "High timed");
    assert_eq!(sorted["cards"][1]["title"], "Low timed");
    assert_eq!(sorted["skipped"][0]["title"], "Untimed deadline");
    assert!(
        sorted["cards"][0]["receipts"]
            .as_array()
            .unwrap()
            .iter()
            .any(|receipt| receipt["message"]
                .as_str()
                .unwrap()
                .contains("agenda sort strategy: time-up,priority-down"))
    );

    let block_request = r#"{"title":"Daily agent agenda","sections":[{"name":"Timed","query":{"start":{"year":2026,"month":5,"day":15},"end":{"year":2026,"month":5,"day":15},"sortStrategy":[{"key":"time","direction":"up"}]}},{"name":"Priority","query":{"start":{"year":2026,"month":5,"day":15},"end":{"year":2026,"month":5,"day":15},"limit":1,"sortStrategy":[{"key":"priority","direction":"down"}]}}]}"#;
    let block: Value = serde_json::from_str(
        &org.agenda_block_json(block_request)
            .expect("agenda block JSON should render"),
    )
    .expect("agenda block JSON should parse");

    assert_eq!(block["schemaVersion"], 1);
    assert_eq!(block["title"], "Daily agent agenda");
    assert_eq!(block["totalCandidates"], 6);
    assert_eq!(block["sections"][0]["index"], 1);
    assert_eq!(block["sections"][0]["name"], "Timed");
    assert_eq!(
        block["sections"][0]["plan"]["cards"][0]["title"],
        "High timed"
    );
    assert_eq!(block["sections"][1]["index"], 2);
    assert_eq!(block["sections"][1]["name"], "Priority");
    assert_eq!(
        block["sections"][1]["plan"]["sortStrategy"][0]["key"],
        "priority"
    );
    assert_eq!(
        block["sections"][1]["plan"]["cards"][0]["title"],
        "High timed"
    );
}
