use orgize_wasm::Org;

#[test]
fn wasm_html_projection_preserves_html_export_blocks() {
    let org = Org::parse(
        r#"
#+begin_export html
<div class="videoWrapper"><iframe src="https://www.youtube.com/embed/vb1-lHR7kRM"></iframe></div>
#+end_export

#+begin_export latex
\LaTeX{}
#+end_export
"#,
    );
    let rendered = org.html();

    assert!(rendered.contains(r#"<div class="videoWrapper"><iframe src="https://www.youtube.com/embed/vb1-lHR7kRM"></iframe></div>"#));
    assert!(!rendered.contains(r#"\LaTeX{}"#));
}
