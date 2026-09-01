use std::path::Path;

#[test]
fn orgize_wasm_is_clean_under_asp_rust_policy() {
    asp_rust::assert_asp_rust_clean(Path::new(env!("CARGO_MANIFEST_DIR")));
}
