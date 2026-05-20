use orgize_wasm::Org;
use serde_json::Value;

#[test]
fn wasm_crypt_contract_exposes_opaque_subtree_advice() {
    let org = Org::parse(
        r#"* Secret note :crypt:
:PROPERTIES:
:CRYPTKEY: 0x0123456789012345678901234567890123456789
:END:
-----BEGIN PGP MESSAGE-----
opaque
-----END PGP MESSAGE-----
** Inherited child
Plain child.
* Key only
:PROPERTIES:
:CRYPTKEY: 0xfeed
:END:
"#,
    );

    let crypt: Value = serde_json::from_str(&org.crypt_json()).expect("crypt JSON should parse");
    let records = crypt["records"]
        .as_array()
        .expect("crypt records should be an array");

    assert_eq!(crypt["schemaVersion"], 1);
    assert_eq!(records.len(), 3);
    assert_eq!(records[0]["title"], "Secret note");
    assert_eq!(records[0]["bodyIsOpaque"], true);
    assert_eq!(records[0]["encryptedPayload"], true);
    assert_eq!(records[0]["cryptKey"]["inherited"], false);
    assert_eq!(records[1]["hasInheritedTag"], true);
    assert_eq!(records[1]["warnings"][0]["kind"], "inheritedCryptTag");
    assert_eq!(records[2]["bodyIsOpaque"], false);
    assert_eq!(records[2]["warnings"][0]["kind"], "cryptKeyWithoutCryptTag");
}

#[test]
fn wasm_snapshot_contract_includes_crypt_projection() {
    let org = Org::parse("* Secret note :crypt:\nPlain body.\n");
    let snapshot: Value = serde_json::from_str(&org.snapshot_json(Some("crypt.org".to_string())))
        .expect("snapshot JSON should parse");

    assert_eq!(snapshot["schemaVersion"], 1);
    assert_eq!(snapshot["crypt"][0]["title"], "Secret note");
    assert_eq!(
        snapshot["crypt"][0]["warnings"][0]["kind"],
        "plaintextCryptBody"
    );
}
