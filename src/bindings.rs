//! `wasm-bindgen` facade for parsing and rendering Org documents.

use crate::{
    dto_capture_request::AgentCaptureJsonRequest,
    dto_clock_model::WasmClockIssueProfileRequest,
    dto_projection,
    dto_refile_request::{parse_optional_refile_targets_request, parse_refile_plan_request},
};
use orgize::{
    ast::{
        AgendaBlockViewQuery, AgendaDate, AgendaQuery, AgendaViewQuery, AgendaViewSortDirection,
        AgendaViewSortKey, AgendaViewSortSpec, AgentMemoryQuery, AgentPlanningQuery,
        ClockIssueProfile, MemoryQuery, ParsedAst,
    },
    export::{from_fn, Container, Event},
    rowan::ast::AstNode,
    Org as Inner,
};
use serde::Deserialize;
use std::cell::{Ref, RefCell};
use std::fmt::Write;

use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[wasm_bindgen]
/// WebAssembly wrapper around [`orgize::Org`].
pub struct Org {
    inner: Inner,
    source: String,
    document: RefCell<Option<ParsedAst>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AgendaViewJsonRequest {
    start: AgendaViewJsonDate,
    end: AgendaViewJsonDate,
    limit: Option<u32>,
    sort_strategy: Option<Vec<AgendaViewSortSpecJson>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AgendaViewJsonDate {
    year: u16,
    month: u8,
    day: u8,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AgendaViewSortSpecJson {
    key: String,
    direction: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AgendaBlockJsonRequest {
    title: Option<String>,
    sections: Vec<AgendaBlockSectionJsonRequest>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AgendaBlockSectionJsonRequest {
    name: String,
    query: AgendaViewJsonRequest,
}

impl AgendaViewJsonDate {
    fn into_agenda_date(self) -> AgendaDate {
        AgendaDate::new(self.year, self.month, self.day)
    }
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

    #[wasm_bindgen(js_name = sparseTreeJson)]
    pub fn sparse_tree_json(
        &self,
        source_file: Option<String>,
        match_expression: Option<String>,
        text: Option<String>,
        include_archived: Option<bool>,
    ) -> Result<String, JsValue> {
        let document = self.document();
        dto_projection::sparse_tree_json(
            &document,
            source_file.as_deref(),
            match_expression.as_deref(),
            text.as_deref(),
            include_archived,
        )
        .map_err(|error| JsValue::from_str(&error))
    }

    #[wasm_bindgen(js_name = sparseTreeExplainJson)]
    pub fn sparse_tree_explain_json(
        &self,
        source_file: Option<String>,
        match_expression: Option<String>,
        text: Option<String>,
        include_archived: Option<bool>,
    ) -> Result<String, JsValue> {
        let document = self.document();
        dto_projection::sparse_tree_explain_json(
            &document,
            source_file.as_deref(),
            match_expression.as_deref(),
            text.as_deref(),
            include_archived,
        )
        .map_err(|error| JsValue::from_str(&error))
    }

    #[wasm_bindgen(js_name = agendaViewJson)]
    pub fn agenda_view_json(&self, request_json: &str) -> Result<String, JsValue> {
        let request: AgendaViewJsonRequest = serde_json::from_str(request_json)
            .map_err(|error| JsValue::from_str(&format!("invalid agenda view request: {error}")))?;
        let document = self.document();
        Ok(dto_projection::agenda_view_json(
            &document,
            &request.into_query()?,
        ))
    }

    #[wasm_bindgen(js_name = agendaBlockJson)]
    pub fn agenda_block_json(&self, request_json: &str) -> Result<String, JsValue> {
        let request: AgendaBlockJsonRequest =
            serde_json::from_str(request_json).map_err(|error| {
                JsValue::from_str(&format!("invalid agenda block request: {error}"))
            })?;
        let document = self.document();
        Ok(dto_projection::agenda_block_json(
            &document,
            &request.into_query()?,
        ))
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

    #[wasm_bindgen(js_name = dynamicBlocksJson)]
    pub fn dynamic_blocks_json(&self) -> String {
        let document = self.document();
        dto_projection::dynamic_blocks_json(&document)
    }

    #[wasm_bindgen(js_name = propertyProfileJson)]
    pub fn property_profile_json(&self) -> String {
        let document = self.document();
        dto_projection::property_profile_json(&document)
    }

    #[wasm_bindgen(js_name = capturePlanJson)]
    pub fn capture_plan_json(&self, request_json: &str) -> Result<String, JsValue> {
        let request: AgentCaptureJsonRequest =
            serde_json::from_str(request_json).map_err(|error| {
                JsValue::from_str(&format!("invalid capture plan request: {error}"))
            })?;
        Ok(dto_projection::capture_plan_json(&request.into_request()?))
    }

    #[wasm_bindgen(js_name = refileTargetsJson)]
    pub fn refile_targets_json(&self, request_json: Option<String>) -> Result<String, JsValue> {
        let document = self.document();
        Ok(dto_projection::refile_targets_json(
            &document,
            &parse_optional_refile_targets_request(request_json.as_deref())?,
        ))
    }

    #[wasm_bindgen(js_name = refilePlanJson)]
    pub fn refile_plan_json(&self, request_json: &str) -> Result<String, JsValue> {
        let document = self.document();
        Ok(dto_projection::refile_plan_json(
            &document,
            &parse_refile_plan_request(request_json)?,
        ))
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

    #[wasm_bindgen(js_name = progressStatsJson)]
    pub fn progress_stats_json(&self) -> String {
        let document = self.document();
        dto_projection::progress_stats_json(&document)
    }

    #[wasm_bindgen(js_name = clockRollupsJson)]
    pub fn clock_rollups_json(&self) -> String {
        let document = self.document();
        dto_projection::clock_rollups_json(&document)
    }

    #[wasm_bindgen(js_name = clockTablePlansJson)]
    pub fn clock_table_plans_json(&self) -> String {
        let document = self.document();
        dto_projection::clock_table_plans_json(&document)
    }

    #[wasm_bindgen(js_name = clockIssuesJson)]
    pub fn clock_issues_json(&self, request_json: Option<String>) -> Result<String, JsValue> {
        let request = parse_optional_clock_issue_request(request_json.as_deref())?;
        let document = self.document();
        Ok(dto_projection::clock_issues_json(
            &document,
            &request.into_profile(),
        ))
    }

    #[wasm_bindgen(js_name = taskBlockersJson)]
    pub fn task_blockers_json(&self) -> String {
        let document = self.document();
        dto_projection::task_blockers_json(&document)
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

impl AgendaViewJsonRequest {
    fn into_query(self) -> Result<AgendaViewQuery, JsValue> {
        let mut query = AgendaViewQuery::new(AgendaQuery::new(
            self.start.into_agenda_date(),
            self.end.into_agenda_date(),
        ));
        if let Some(limit) = self.limit {
            query = query.limit(limit as usize);
        }
        for sort_spec in self.sort_strategy.unwrap_or_default() {
            query = query.sort_by(sort_spec.into_spec()?);
        }
        Ok(query)
    }
}

impl AgendaViewSortSpecJson {
    fn into_spec(self) -> Result<AgendaViewSortSpec, JsValue> {
        Ok(AgendaViewSortSpec::new(
            agenda_view_sort_key(&self.key)?,
            agenda_view_sort_direction(&self.direction)?,
        ))
    }
}

impl AgendaBlockJsonRequest {
    fn into_query(self) -> Result<AgendaBlockViewQuery, JsValue> {
        let mut query =
            AgendaBlockViewQuery::new(self.title.unwrap_or_else(|| "Agenda block".to_string()));
        for section in self.sections {
            query = query.section(section.name, section.query.into_query()?);
        }
        Ok(query)
    }
}

impl WasmClockIssueProfileRequest {
    fn into_profile(self) -> ClockIssueProfile {
        let mut profile = ClockIssueProfile::org_default();
        if let Some(value) = self.max_duration_seconds {
            profile = match value {
                Some(seconds) => profile.max_duration_seconds(seconds),
                None => profile.without_max_duration(),
            };
        }
        if let Some(value) = self.min_duration_seconds {
            profile = match value {
                Some(seconds) => profile.min_duration_seconds(seconds),
                None => profile.without_min_duration(),
            };
        }
        if let Some(value) = self.max_gap_seconds {
            profile = match value {
                Some(seconds) => profile.max_gap_seconds(seconds),
                None => profile.without_max_gap(),
            };
        }
        if let Some(minutes) = self.gap_ok_around_minutes {
            profile = profile.gap_ok_around_minutes(minutes);
        }
        profile
    }
}

fn agenda_view_sort_key(value: &str) -> Result<AgendaViewSortKey, JsValue> {
    match value {
        "displayDate" | "display-date" => Ok(AgendaViewSortKey::DisplayDate),
        "time" => Ok(AgendaViewSortKey::Time),
        "kind" => Ok(AgendaViewSortKey::Kind),
        "level" => Ok(AgendaViewSortKey::Level),
        "title" | "alpha" => Ok(AgendaViewSortKey::Title),
        "targetDate" | "target-date" | "timestamp" => Ok(AgendaViewSortKey::TargetDate),
        "scheduledDate" | "scheduled-date" | "scheduled" => Ok(AgendaViewSortKey::ScheduledDate),
        "deadlineDate" | "deadline-date" | "deadline" => Ok(AgendaViewSortKey::DeadlineDate),
        "priority" => Ok(AgendaViewSortKey::Priority),
        "category" => Ok(AgendaViewSortKey::Category),
        "todoState" | "todo-state" | "todo" => Ok(AgendaViewSortKey::TodoState),
        other => Err(JsValue::from_str(&format!(
            "invalid agenda view sort key: {other}"
        ))),
    }
}

fn agenda_view_sort_direction(value: &str) -> Result<AgendaViewSortDirection, JsValue> {
    match value {
        "up" => Ok(AgendaViewSortDirection::Up),
        "down" => Ok(AgendaViewSortDirection::Down),
        "keep" => Ok(AgendaViewSortDirection::Keep),
        other => Err(JsValue::from_str(&format!(
            "invalid agenda view sort direction: {other}"
        ))),
    }
}

fn parse_optional_clock_issue_request(
    request_json: Option<&str>,
) -> Result<WasmClockIssueProfileRequest, JsValue> {
    let Some(request_json) = request_json.map(str::trim).filter(|text| !text.is_empty()) else {
        return Ok(WasmClockIssueProfileRequest::default());
    };
    serde_json::from_str(request_json)
        .map_err(|error| JsValue::from_str(&format!("invalid clock issue request: {error}")))
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
