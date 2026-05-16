use orgize_wasm::Org;
use serde_json::Value;

#[test]
fn wasm_clock_rollup_contract_exposes_clocktable_plans() {
    let org = Org::parse(
        r#"#+BEGIN: clocktable :scope file :maxlevel 1
#+END:
* TODO Build API
:PROPERTIES:
:Effort: 2:00
:END:
CLOCK: [2026-05-15 Fri 09:00]--[2026-05-15 Fri 10:30] =>  1:30
** TODO Parser
:PROPERTIES:
:Effort: 0:45
:END:
CLOCK: [2026-05-15 Fri 10:30]--[2026-05-15 Fri 11:00] =>  0:30
"#,
    );
    let clock_rollups: Value =
        serde_json::from_str(&org.clock_rollups_json()).expect("clock rollups JSON should parse");
    let records = clock_rollups["records"]
        .as_array()
        .expect("clock rollup records should be an array");

    assert_eq!(clock_rollups["schemaVersion"], 1);
    assert_eq!(records.len(), 2);
    assert_eq!(records[0]["title"], "Build API");
    assert_eq!(records[0]["localClock"]["totalSeconds"], 5_400);
    assert_eq!(records[0]["subtreeClock"]["entries"], 2);
    assert_eq!(records[0]["subtreeClock"]["totalSeconds"], 7_200);
    assert_eq!(records[0]["effort"]["subtreeTotalSeconds"], 9_900);
    assert_eq!(records[0]["effort"]["deltaSeconds"], -2_700);
    assert_eq!(records[0]["effort"]["status"], "underEffort");

    let clock_tables: Value = serde_json::from_str(&org.clock_table_plans_json())
        .expect("clock table plans JSON should parse");
    let plans = clock_tables["plans"]
        .as_array()
        .expect("clock table plans should be an array");
    assert_eq!(clock_tables["schemaVersion"], 1);
    assert_eq!(plans.len(), 1);
    assert_eq!(plans[0]["scope"]["kind"], "file");
    assert_eq!(plans[0]["maxLevel"], 1);
    assert!(plans[0]["timeWindow"].is_null());
    assert_eq!(plans[0]["rows"].as_array().unwrap().len(), 1);
    assert_eq!(plans[0]["rows"][0]["title"], "Build API");
    assert_eq!(plans[0]["rows"][0]["clock"]["totalSeconds"], 7_200);
    assert!(plans[0]["warnings"].as_array().unwrap().is_empty());
}

#[test]
fn wasm_clocktable_contract_exposes_applied_time_window() {
    let org = Org::parse(
        r#"#+BEGIN: clocktable :scope file :maxlevel 1 :tstart "<2026-05-15 Fri 10:00>" :tend "<2026-05-15 Fri 11:00>"
#+END:
* TODO Build API
CLOCK: [2026-05-15 Fri 09:00]--[2026-05-15 Fri 10:30] =>  1:30
CLOCK: [2026-05-15 Fri 10:30]--[2026-05-15 Fri 11:00] =>  0:30
"#,
    );

    let clock_tables: Value = serde_json::from_str(&org.clock_table_plans_json())
        .expect("clock table plans JSON should parse");
    let plan = &clock_tables["plans"][0];
    assert_eq!(plan["timeWindow"]["source"], "tstartTend");
    assert_eq!(plan["timeWindow"]["start"]["hour"], 10);
    assert_eq!(plan["timeWindow"]["endExclusive"]["hour"], 11);
    assert_eq!(plan["rows"][0]["clock"]["entries"], 2);
    assert_eq!(plan["rows"][0]["clock"]["totalSeconds"], 3_600);
}

#[test]
fn wasm_clocktable_contract_exposes_applied_match_filter() {
    let org = Org::parse(
        r#"#+BEGIN: clocktable :scope file :maxlevel 1 :match "+client-internal"
#+END:
* TODO Client :client:
CLOCK: [2026-05-15 Fri 09:00]--[2026-05-15 Fri 10:00] =>  1:00
** TODO Internal :internal:
CLOCK: [2026-05-15 Fri 10:00]--[2026-05-15 Fri 11:00] =>  1:00
"#,
    );

    let clock_tables: Value = serde_json::from_str(&org.clock_table_plans_json())
        .expect("clock table plans JSON should parse");
    let plan = &clock_tables["plans"][0];
    assert_eq!(plan["matchFilter"]["expression"], "+client-internal");
    assert_eq!(plan["rows"].as_array().unwrap().len(), 1);
    assert_eq!(plan["rows"][0]["title"], "Client");
    assert_eq!(plan["rows"][0]["clock"]["entries"], 1);
    assert_eq!(plan["rows"][0]["clock"]["totalSeconds"], 3_600);
}

#[test]
fn wasm_clocktable_contract_exposes_property_columns() {
    let org = Org::parse(
        r#"#+BEGIN: clocktable :scope file :maxlevel 2 :properties ("Owner" "Phase") :inherit-props t
#+END:
* TODO Project
:PROPERTIES:
:Owner: Ada
:END:
CLOCK: [2026-05-15 Fri 09:00]--[2026-05-15 Fri 10:00] =>  1:00
** TODO Child
:PROPERTIES:
:Phase: Build
:END:
CLOCK: [2026-05-15 Fri 10:00]--[2026-05-15 Fri 11:00] =>  1:00
"#,
    );

    let clock_tables: Value = serde_json::from_str(&org.clock_table_plans_json())
        .expect("clock table plans JSON should parse");
    let plan = &clock_tables["plans"][0];
    assert_eq!(plan["propertyColumns"]["names"][0], "Owner");
    assert_eq!(plan["propertyColumns"]["names"][1], "Phase");
    assert_eq!(plan["propertyColumns"]["inherit"], true);
    assert_eq!(plan["rows"].as_array().unwrap().len(), 2);
    assert_eq!(plan["rows"][0]["propertyValues"][0]["value"], "Ada");
    assert_eq!(plan["rows"][0]["propertyValues"][0]["inherited"], false);
    assert!(plan["rows"][0]["propertyValues"][1]["value"].is_null());
    assert_eq!(plan["rows"][1]["propertyValues"][0]["value"], "Ada");
    assert_eq!(plan["rows"][1]["propertyValues"][0]["inherited"], true);
    assert_eq!(plan["rows"][1]["propertyValues"][1]["value"], "Build");
}

#[test]
fn wasm_clock_issues_contract_exposes_agent_diagnostics() {
    let org = Org::parse(
        r#"* TODO Work
CLOCK: [2026-05-15 Fri 09:00]--[2026-05-15 Fri 09:10] =>  0:10
CLOCK: [2026-05-15 Fri 09:05]--[2026-05-15 Fri 09:20] =>  0:15
CLOCK: [2026-05-15 Fri 09:40]--[2026-05-15 Fri 09:50] =>  0:10
"#,
    );

    let clock_issues: Value = serde_json::from_str(
        &org.clock_issues_json(Some(
            r#"{"maxGapSeconds":600,"gapOkAroundMinutes":[]}"#.to_string(),
        ))
        .expect("clock issues JSON should render"),
    )
    .expect("clock issues JSON should parse");
    let findings = clock_issues["findings"]
        .as_array()
        .expect("clock issue findings should be an array");

    assert_eq!(clock_issues["schemaVersion"], 1);
    assert_eq!(findings.len(), 2);
    assert_eq!(findings[0]["kind"], "overlap");
    assert_eq!(findings[0]["durationSeconds"], 300);
    assert_eq!(findings[0]["previousClock"]["durationSeconds"], 600);
    assert_eq!(findings[0]["clock"]["start"]["hour"], 9);
    assert_eq!(findings[1]["kind"], "gap");
    assert_eq!(findings[1]["thresholdSeconds"], 600);

    let without_gaps: Value = serde_json::from_str(
        &org.clock_issues_json(Some(r#"{"maxGapSeconds":null}"#.to_string()))
            .expect("clock issues JSON should render without gaps"),
    )
    .expect("clock issues JSON should parse without gaps");
    assert_eq!(without_gaps["findings"].as_array().unwrap().len(), 1);
}
