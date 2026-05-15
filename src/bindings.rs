//! `wasm-bindgen` facade for parsing and rendering Org documents.

use crate::dto_projection;
use orgize::{
    ast::{AgendaDate, AgendaQuery, AgentMemoryQuery, AgentPlanningQuery, MemoryQuery, ParsedAst},
    export::{from_fn, Container, Event},
    rowan::ast::AstNode,
    Org as Inner,
};
use std::cell::{Ref, RefCell};
use std::fmt::Write;

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
/// WebAssembly wrapper around [`orgize::Org`].
pub struct Org {
    inner: Inner,
    source: String,
    document: RefCell<Option<ParsedAst>>,
}

#[wasm_bindgen]
impl Org {
    #[wasm_bindgen(constructor)]
    pub fn parse(input: &str) -> Self {
        Org {
            inner: Inner::parse(input),
            source: input.to_string(),
            document: RefCell::new(None),
        }
    }

    pub fn html(&self) -> String {
        self.inner.to_html()
    }

    pub fn latex(&self) -> String {
        self.inner.to_latex()
    }

    pub fn markdown(&self) -> String {
        self.inner.to_markdown()
    }

    pub fn org(&self) -> String {
        self.inner.to_org()
    }

    pub fn syntax(&self) -> String {
        format!("{:#?}", self.inner.syntax_document().syntax())
    }

    pub fn semantic(&self) -> String {
        let document = self.document();
        let macro_expansions = document.macro_expansions();
        if macro_expansions.is_empty() {
            format!("{document:#?}")
        } else {
            format!("{document:#?}\n\nMacro expansions:\n{macro_expansions:#?}")
        }
    }

    pub fn agenda(&self) -> String {
        let match_expression = r#"+demo|CATEGORY="agenda-demo""#;
        let query = AgendaQuery::new(AgendaDate::new(2026, 5, 10), AgendaDate::new(2026, 5, 24))
            .include_closed(true)
            .include_inactive_timestamps(true)
            .include_diary_timestamps(true)
            .match_expression(match_expression)
            .expect("demo agenda match expression");
        format!(
            "Agenda match query: {match_expression}\n\n{:#?}",
            self.document().to_bare().agenda_entries(&query)
        )
    }

    #[wasm_bindgen(js_name = agentPlanning)]
    pub fn agent_planning(&self) -> String {
        let query = AgentPlanningQuery::new(
            AgendaQuery::new(AgendaDate::new(2026, 5, 10), AgendaDate::new(2026, 5, 24))
                .include_closed(true),
        );
        self.document()
            .agent_planning_snapshot(&query)
            .to_compact_text("wasm-demo.org")
    }

    #[wasm_bindgen(js_name = agentMemory)]
    pub fn agent_memory(&self) -> String {
        let query = AgentMemoryQuery::new(MemoryQuery::new().require_tag("memory"));
        self.document()
            .agent_memory_snapshot(&query)
            .to_compact_text("wasm-demo.org")
    }

    pub fn update(&mut self, s: &str) {
        self.source = s.to_string();
        self.inner = Inner::parse(s);
        self.document.replace(None);
    }

    #[wasm_bindgen(js_name = outlineJson)]
    pub fn outline_json(&self) -> String {
        let document = self.document();
        dto_projection::outline_json(&document)
    }

    #[wasm_bindgen(js_name = metadataJson)]
    pub fn metadata_json(&self) -> String {
        let document = self.document();
        dto_projection::metadata_json(&document)
    }

    #[wasm_bindgen(js_name = lintJson)]
    pub fn lint_json(&self) -> String {
        let document = self.document();
        dto_projection::lint_json(&document, &self.source)
    }

    #[wasm_bindgen(js_name = sectionIndexJson)]
    pub fn section_index_json(&self, source_file: Option<String>) -> String {
        let document = self.document();
        dto_projection::section_index_json(&document, source_file.as_deref())
    }

    #[wasm_bindgen(js_name = viewIndexJson)]
    pub fn view_index_json(&self, source_file: Option<String>) -> String {
        let document = self.document();
        dto_projection::view_index_json(&document, source_file.as_deref())
    }

    #[wasm_bindgen(js_name = attachmentsJson)]
    pub fn attachments_json(&self, source_file: Option<String>) -> String {
        let document = self.document();
        dto_projection::attachments_json(&document, source_file.as_deref())
    }

    #[wasm_bindgen(js_name = sourceBlocksJson)]
    pub fn source_blocks_json(&self) -> String {
        let document = self.document();
        dto_projection::source_blocks_json(&document)
    }

    #[wasm_bindgen(js_name = columnViewsJson)]
    pub fn column_views_json(&self) -> String {
        let document = self.document();
        dto_projection::column_views_json(&document)
    }

    #[wasm_bindgen(js_name = includeExpansionJson)]
    pub fn include_expansion_json(&self, base_dir: Option<String>) -> String {
        let document = self.document();
        dto_projection::include_expansion_json(&document, base_dir.as_deref())
    }

    #[wasm_bindgen(js_name = datetreeJson)]
    pub fn datetree_json(&self) -> String {
        let document = self.document();
        dto_projection::datetree_json(&document)
    }

    #[wasm_bindgen(js_name = snapshotJson)]
    pub fn snapshot_json(&self, source_file: Option<String>) -> String {
        let document = self.document();
        dto_projection::snapshot_json(&document, &self.source, source_file.as_deref())
    }

    pub fn traverse(&self) -> String {
        let mut result = String::new();
        let mut ident = 0;
        let mut handler = from_fn(|event| {
            let (name, range) = match &event {
                Event::Enter(container) => match container {
                    Container::Document(x) => ("Document", x.text_range()),
                    Container::Section(x) => ("Section", x.text_range()),
                    Container::Paragraph(x) => ("Paragraph", x.text_range()),
                    Container::Headline(x) => ("Headline", x.text_range()),
                    Container::OrgTable(x) => ("OrgTable", x.text_range()),
                    Container::OrgTableRow(x) => ("OrgTableRow", x.text_range()),
                    Container::OrgTableCell(x) => ("OrgTableCell", x.text_range()),
                    Container::TableEl(x) => ("TableEl", x.text_range()),
                    Container::List(x) => ("List", x.text_range()),
                    Container::ListItem(x) => ("ListItem", x.text_range()),
                    Container::Drawer(x) => ("Drawer", x.text_range()),
                    Container::DynBlock(x) => ("DynBlock", x.text_range()),
                    Container::FnDef(x) => ("FnDef", x.text_range()),
                    Container::Comment(x) => ("Comment", x.text_range()),
                    Container::FixedWidth(x) => ("FixedWidth", x.text_range()),
                    Container::SpecialBlock(x) => ("SpecialBlock", x.text_range()),
                    Container::QuoteBlock(x) => ("QuoteBlock", x.text_range()),
                    Container::CenterBlock(x) => ("CenterBlock", x.text_range()),
                    Container::VerseBlock(x) => ("VerseBlock", x.text_range()),
                    Container::CommentBlock(x) => ("CommentBlock", x.text_range()),
                    Container::ExampleBlock(x) => ("ExampleBlock", x.text_range()),
                    Container::ExportBlock(x) => ("ExportBlock", x.text_range()),
                    Container::SourceBlock(x) => ("SourceBlock", x.text_range()),
                    Container::Link(x) => ("Link", x.text_range()),
                    Container::RadioTarget(x) => ("RadioTarget", x.text_range()),
                    Container::FnRef(x) => ("FnRef", x.text_range()),
                    Container::Target(x) => ("Target", x.text_range()),
                    Container::Bold(x) => ("Bold", x.text_range()),
                    Container::Strike(x) => ("Strike", x.text_range()),
                    Container::Italic(x) => ("Italic", x.text_range()),
                    Container::Underline(x) => ("Underline", x.text_range()),
                    Container::Verbatim(x) => ("Verbatim", x.text_range()),
                    Container::Code(x) => ("Code", x.text_range()),
                    Container::Superscript(x) => ("Superscript", x.text_range()),
                    Container::Subscript(x) => ("Subscript", x.text_range()),
                    Container::BabelCall(x) => ("BabelCall", x.text_range()),
                    Container::PropertyDrawer(x) => ("PropertyDrawer", x.text_range()),
                    Container::AffiliatedKeyword(x) => ("AffiliatedKeyword", x.text_range()),
                    Container::Keyword(x) => ("Keyword", x.text_range()),
                    _ => unreachable!(),
                },
                Event::Leave(_) => {
                    ident -= 2;
                    return;
                }
                Event::Text(x) => ("Text", x.text_range()),
                Event::Macros(x) => ("Macros", x.text_range()),
                Event::Cookie(x) => ("Cookie", x.text_range()),
                Event::Citation(x) => ("Citation", x.text_range()),
                Event::InlineCall(x) => ("InlineCall", x.text_range()),
                Event::InlineSrc(x) => ("InlineSrc", x.text_range()),
                Event::Clock(x) => ("Clock", x.text_range()),
                Event::LineBreak(x) => ("LineBreak", x.text_range()),
                Event::Snippet(x) => ("Snippet", x.text_range()),
                Event::Rule(x) => ("Rule", x.text_range()),
                Event::Timestamp(x) => ("Timestamp", x.text_range()),
                Event::LatexFragment(x) => ("LatexFragment", x.text_range()),
                Event::LatexEnvironment(x) => ("LatexEnvironment", x.text_range()),
                Event::Entity(x) => ("Entity", x.text_range()),
                _ => unreachable!(),
            };

            let _ = writeln!(
                &mut result,
                "{:ident$}{}@{}..{}",
                "",
                name,
                u32::from(range.start()),
                u32::from(range.end())
            );

            if let Event::Enter(_) = event {
                ident += 2;
            }
        });
        self.inner.traverse(&mut handler);
        result
    }

    #[wasm_bindgen(getter, js_name = "buildTime")]
    pub fn build_time() -> String {
        env!("CARGO_BUILD_TIME").into()
    }

    #[wasm_bindgen(getter, js_name = "gitHash")]
    pub fn git_hash() -> String {
        env!("CARGO_GIT_HASH").into()
    }
}

impl Org {
    fn document(&self) -> Ref<'_, ParsedAst> {
        if self.document.borrow().is_none() {
            self.document.replace(Some(self.inner.document()));
        }
        Ref::map(self.document.borrow(), |document| {
            document
                .as_ref()
                .expect("semantic document cache should be initialized")
        })
    }
}
