use orgize_wasm::Org;
use serde_json::Value;

#[test]
fn wasm_section_index_contract_exposes_display_titles() {
    let org = Org::parse(
        r#"* [[https://example.com/wallpaper][Wallpaper]] :ATTACH:
SCHEDULED: <2020-12-19 Sat>-<2020-12-19 Sat>
"#,
    );
    let payload: Value =
        serde_json::from_str(&org.section_index_json(None)).expect("section index JSON");

    assert_eq!(
        payload["records"][0]["title"],
        "[[https://example.com/wallpaper][Wallpaper]]"
    );
    assert_eq!(payload["records"][0]["titleText"], "Wallpaper");
    assert_eq!(payload["records"][0]["outlinePathText"][0], "Wallpaper");
    assert_eq!(
        payload["records"][0]["planning"]["scheduled"]["raw"],
        "<2020-12-19 Sat>-<2020-12-19 Sat>"
    );
}
