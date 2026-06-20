use orgize_wasm::Org;
use serde_json::Value;

#[test]
fn wasm_replace_range_updates_source_and_cached_projection() {
    let mut org = Org::parse("* TODO Build wasm\n");

    assert!(org.outline_json().contains("\"todo\":\"TODO\""));
    assert_eq!(org.source_len_bytes(), 18);

    assert!(org.replace_range(2, 6, "DONE").is_ok());

    assert_eq!(org.source_len_bytes(), 18);
    assert!(org.org().contains("* DONE Build wasm"));
    assert!(org.outline_json().contains("\"todo\":\"DONE\""));
}

#[test]
fn wasm_format_returns_conservative_format_response() {
    let org = Org::parse("* TODO Build wasm  \n\n");

    let response: Value =
        serde_json::from_str(&org.format(None).expect("format should render JSON"))
            .expect("format response should parse");

    assert_eq!(response["schemaVersion"], 1);
    assert_eq!(response["output"], "* TODO Build wasm\n");
    assert_eq!(response["changed"], true);
}

#[test]
fn wasm_format_accepts_options() {
    let org = Org::parse("* TODO Build wasm  \n");

    let response: Value = serde_json::from_str(
        &org.format(Some(
            r#"{"trimTrailingWhitespace":false,"finalNewline":false}"#.to_string(),
        ))
        .expect("format with options should render JSON"),
    )
    .expect("format response should parse");

    assert_eq!(response["output"], "* TODO Build wasm  ");
    assert_eq!(response["changed"], true);
}
