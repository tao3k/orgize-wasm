//! JSON request parsing for refile wasm bindings.

use orgize::ast::{
    RefileAction, RefileInsertPosition, RefileOutlinePathMode, RefileParentCreationMode,
    RefilePlanRequest, RefileTargetQuery, RefileTargetSpec,
};
use serde::Deserialize;
use wasm_bindgen::JsValue;

#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RefileTargetsJsonRequest {
    source_file: Option<String>,
    outline_path_mode: Option<String>,
    specs: Option<Vec<RefileTargetSpecJson>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RefileTargetSpecJson {
    kind: String,
    value: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RefilePlanJsonRequest {
    source_file: Option<String>,
    source_outline_path: Vec<String>,
    target_outline_path: Vec<String>,
    action: Option<String>,
    insert_position: Option<String>,
    parent_creation: Option<String>,
}

pub(crate) fn parse_optional_refile_targets_request(
    request_json: Option<&str>,
) -> Result<RefileTargetQuery, JsValue> {
    let Some(request_json) = request_json.map(str::trim).filter(|text| !text.is_empty()) else {
        return RefileTargetsJsonRequest::default().into_query();
    };
    serde_json::from_str::<RefileTargetsJsonRequest>(request_json)
        .map_err(|error| JsValue::from_str(&format!("invalid refile targets request: {error}")))?
        .into_query()
}

pub(crate) fn parse_refile_plan_request(request_json: &str) -> Result<RefilePlanRequest, JsValue> {
    serde_json::from_str::<RefilePlanJsonRequest>(request_json)
        .map_err(|error| JsValue::from_str(&format!("invalid refile plan request: {error}")))?
        .into_plan_request()
}

impl RefileTargetsJsonRequest {
    fn into_query(self) -> Result<RefileTargetQuery, JsValue> {
        let mut query = RefileTargetQuery::new();
        if let Some(source_file) = self.source_file {
            query = query.source_file(source_file);
        }
        if let Some(mode) = self.outline_path_mode {
            query = query.outline_path_mode(refile_outline_path_mode(&mode)?);
        }
        for spec in self.specs.unwrap_or_default() {
            query = query.spec(spec.into_spec()?);
        }
        Ok(query)
    }
}

impl RefileTargetSpecJson {
    fn into_spec(self) -> Result<RefileTargetSpec, JsValue> {
        let value = self.value.unwrap_or_default();
        match self.kind.as_str() {
            "all" => Ok(RefileTargetSpec::All),
            "tag" => Ok(RefileTargetSpec::Tag(value)),
            "todo" => Ok(RefileTargetSpec::Todo(value)),
            "level" => parse_refile_level("level", &value).map(RefileTargetSpec::Level),
            "maxLevel" => parse_refile_level("maxLevel", &value).map(RefileTargetSpec::MaxLevel),
            "regexp" => Ok(RefileTargetSpec::Regexp(value)),
            other => Err(JsValue::from_str(&format!(
                "invalid refile target spec kind: {other}"
            ))),
        }
    }
}

impl RefilePlanJsonRequest {
    fn into_plan_request(self) -> Result<RefilePlanRequest, JsValue> {
        let mut request =
            RefilePlanRequest::new(self.source_outline_path, self.target_outline_path);
        if let Some(source_file) = self.source_file {
            request = request.source_file(source_file);
        }
        if let Some(action) = self.action {
            request = request.action(refile_action(&action)?);
        }
        if let Some(insert_position) = self.insert_position {
            request = request.insert_position(refile_insert_position(&insert_position)?);
        }
        if let Some(parent_creation) = self.parent_creation {
            request = request.parent_creation(refile_parent_creation(&parent_creation)?);
        }
        Ok(request)
    }
}

fn parse_refile_level(kind: &str, value: &str) -> Result<usize, JsValue> {
    value.parse::<usize>().map_err(|error| {
        JsValue::from_str(&format!("invalid refile {kind} value `{value}`: {error}"))
    })
}

fn refile_outline_path_mode(value: &str) -> Result<RefileOutlinePathMode, JsValue> {
    match value {
        "none" => Ok(RefileOutlinePathMode::None),
        "outline" => Ok(RefileOutlinePathMode::Outline),
        "file" => Ok(RefileOutlinePathMode::File),
        "fullFilePath" => Ok(RefileOutlinePathMode::FullFilePath),
        "bufferName" => Ok(RefileOutlinePathMode::BufferName),
        "title" => Ok(RefileOutlinePathMode::Title),
        other => Err(JsValue::from_str(&format!(
            "invalid refile outline path mode: {other}"
        ))),
    }
}

fn refile_action(value: &str) -> Result<RefileAction, JsValue> {
    match value {
        "move" => Ok(RefileAction::Move),
        "copy" => Ok(RefileAction::Copy),
        "goto" => Ok(RefileAction::Goto),
        other => Err(JsValue::from_str(&format!(
            "invalid refile action: {other}"
        ))),
    }
}

fn refile_insert_position(value: &str) -> Result<RefileInsertPosition, JsValue> {
    match value {
        "lastChild" => Ok(RefileInsertPosition::LastChild),
        "firstChild" => Ok(RefileInsertPosition::FirstChild),
        other => Err(JsValue::from_str(&format!(
            "invalid refile insert position: {other}"
        ))),
    }
}

fn refile_parent_creation(value: &str) -> Result<RefileParentCreationMode, JsValue> {
    match value {
        "never" => Ok(RefileParentCreationMode::Never),
        "plan" | "allow" => Ok(RefileParentCreationMode::Plan),
        "confirm" => Ok(RefileParentCreationMode::Confirm),
        other => Err(JsValue::from_str(&format!(
            "invalid refile parent creation mode: {other}"
        ))),
    }
}
