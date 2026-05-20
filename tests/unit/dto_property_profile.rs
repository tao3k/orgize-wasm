use orgize_wasm::Org;
use serde_json::Value;

#[test]
fn wasm_property_profile_contract_exposes_allowed_values() {
    let org = Org::parse(
        r#"#+PROPERTY: Effort_ALL 0 0:30 "1 hour"
* Project
:PROPERTIES:
:Owner_ALL: "Sarah Connor" Jim ""
:PROPERTY_SCHEMA: {{{property_schema(wendao.capture.v1)}}}
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
    assert!(
        profile["inheritedKeys"]
            .as_array()
            .unwrap()
            .iter()
            .any(|key| key == "Owner")
    );
    assert!(
        allowed_values
            .iter()
            .any(|record| record["descriptorKey"] == "VISIBILITY_ALL"
                && record["scope"]["kind"] == "fixedGlobal")
    );

    let owner = allowed_values
        .iter()
        .find(|record| record["descriptorKey"] == "Owner_ALL")
        .expect("section Owner_ALL record");
    assert_eq!(owner["scope"]["kind"], "section");
    assert_eq!(owner["scope"]["title"], "Project");
    assert_eq!(owner["values"][0], "Sarah Connor");
    assert_eq!(owner["values"][2], "");

    let with_registry: Value = serde_json::from_str(
        &org.property_profile_with_schemas_json(capture_schema_registry_request())
            .expect("property profile with loaded schema registry"),
    )
    .expect("property profile with schema registry JSON should parse");
    let applications = with_registry["profile"]["schemaApplications"]
        .as_array()
        .expect("schema applications should be an array");
    assert_eq!(applications[0]["reference"]["kind"], "macro");
    assert_eq!(
        applications[0]["reference"]["normalized"],
        "wendao.capture.v1"
    );
    assert_eq!(applications[0]["contractId"], "wendao.capture.v1");
    assert_eq!(
        applications[0]["findings"][0]["kind"],
        "missingRequiredProperty"
    );
    insta::assert_snapshot!(
        "wasm_property_profile_with_schema_registry",
        serde_json::to_string_pretty(&serde_json::json!({
            "schemaVersion": with_registry["schemaVersion"],
            "schemaApplications": applications,
        }))
        .expect("stable JSON snapshot")
    );

    let snapshot: Value = serde_json::from_str(
        &org.snapshot_with_schemas_json(
            capture_schema_registry_request(),
            Some("capture.org".to_string()),
        )
        .expect("snapshot with loaded schema registry"),
    )
    .expect("snapshot with schema registry JSON should parse");
    assert!(
        snapshot["lint"]
            .as_array()
            .expect("snapshot lint findings")
            .iter()
            .any(|finding| finding["code"] == "ORG040")
    );
    insta::assert_snapshot!(
        "wasm_snapshot_with_schema_registry",
        serde_json::to_string_pretty(&serde_json::json!({
            "propertyProfile": {
                "schemaApplications": snapshot["propertyProfile"]["schemaApplications"],
            },
            "lint": snapshot["lint"],
        }))
        .expect("stable snapshot JSON")
    );
}

fn capture_schema_registry_request() -> &'static str {
    r#"{
      "contracts": [{
        "id": "wendao.capture.v1",
        "aliases": ["file:schemas/capture.schema.json#wendao.capture.v1"],
        "allowUnknownProperties": false,
        "fields": [
          {"key": "CAPTURE_KIND", "required": true, "valueRule": {"kind": "oneOf", "values": ["idea", "note"]}},
          {"key": "Owner", "valueRule": {"kind": "nonEmpty"}}
        ]
      }]
    }"#
}
