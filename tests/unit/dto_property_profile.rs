use orgize_wasm::Org;
use serde_json::Value;

#[test]
fn wasm_property_profile_contract_exposes_allowed_values() {
    let org = Org::parse(
        r#"#+PROPERTY: Effort_ALL 0 0:30 "1 hour"
* Project
:PROPERTIES:
:Owner_ALL: "Sarah Connor" Jim ""
:Owner: Sarah Connor
:END:
"#,
    );

    let property_profile: Value = serde_json::from_str(&org.property_profile_json())
        .expect("property profile JSON should parse");
    let profile = &property_profile["profile"];
    let allowed_values = profile["allowedValues"]
        .as_array()
        .expect("allowed values should be an array");

    assert_eq!(property_profile["schemaVersion"], 1);
    assert_eq!(profile["inheritance"], "all");
    assert!(profile["inheritedKeys"]
        .as_array()
        .unwrap()
        .iter()
        .any(|key| key == "Owner"));
    assert!(allowed_values
        .iter()
        .any(|record| record["descriptorKey"] == "VISIBILITY_ALL"
            && record["scope"]["kind"] == "fixedGlobal"));

    let owner = allowed_values
        .iter()
        .find(|record| record["descriptorKey"] == "Owner_ALL")
        .expect("section Owner_ALL record");
    assert_eq!(owner["scope"]["kind"], "section");
    assert_eq!(owner["scope"]["title"], "Project");
    assert_eq!(owner["values"][0], "Sarah Connor");
    assert_eq!(owner["values"][2], "");
}
