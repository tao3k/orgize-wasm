//! JSON request parsing for Agent capture wasm bindings.

use orgize::ast::{
    AgendaDate, AgendaTime, AgentCaptureInsertPosition, AgentCaptureKind, AgentCaptureLink,
    AgentCaptureMemoryPolicy, AgentCaptureProperty, AgentCaptureRequest, AgentCaptureSource,
    AgentCaptureSourceKind, AgentCaptureTarget, AgentCaptureTimestamp,
};
use serde::Deserialize;
use wasm_bindgen::JsValue;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AgentCaptureJsonRequest {
    kind: String,
    title: String,
    body: Option<String>,
    target: Option<AgentCaptureTargetJson>,
    source: Option<AgentCaptureSourceJson>,
    captured_at: Option<AgentCaptureTimestampJson>,
    tags: Option<Vec<String>>,
    properties: Option<Vec<AgentCapturePropertyJson>>,
    quote: Option<String>,
    links: Option<Vec<AgentCaptureLinkJson>>,
    memory_policy: Option<String>,
    requires_confirmation: Option<bool>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AgentCaptureTargetJson {
    kind: String,
    source_file: Option<String>,
    outline_path: Option<Vec<String>>,
    date: Option<AgentCaptureDateJson>,
    insert_position: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AgentCaptureSourceJson {
    kind: Option<String>,
    actor: Option<String>,
    uri: Option<String>,
    label: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AgentCaptureTimestampJson {
    year: u16,
    month: u8,
    day: u8,
    hour: Option<u8>,
    minute: Option<u8>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AgentCaptureDateJson {
    year: u16,
    month: u8,
    day: u8,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AgentCapturePropertyJson {
    key: String,
    value: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AgentCaptureLinkJson {
    url: String,
    label: Option<String>,
}

impl AgentCaptureJsonRequest {
    pub(crate) fn into_request(self) -> Result<AgentCaptureRequest, JsValue> {
        let mut request = AgentCaptureRequest::new(agent_capture_kind(&self.kind)?, self.title);
        if let Some(body) = self.body {
            request.body = Some(body);
        }
        if let Some(target) = self.target {
            request.target = target.into_target()?;
        }
        if let Some(source) = self.source {
            request.source = source.into_source()?;
        }
        if let Some(captured_at) = self.captured_at {
            request.captured_at = Some(captured_at.into_timestamp());
        }
        request.tags = self.tags.unwrap_or_default();
        request.properties = self
            .properties
            .unwrap_or_default()
            .into_iter()
            .map(|property| AgentCaptureProperty {
                key: property.key,
                value: property.value,
            })
            .collect();
        request.quote = self.quote;
        request.links = self
            .links
            .unwrap_or_default()
            .into_iter()
            .map(|link| AgentCaptureLink {
                url: link.url,
                label: link.label,
            })
            .collect();
        if let Some(memory_policy) = self.memory_policy {
            request.memory_policy = agent_capture_memory_policy(&memory_policy)?;
        }
        if let Some(requires_confirmation) = self.requires_confirmation {
            request.requires_confirmation = requires_confirmation;
        }
        Ok(request)
    }
}

impl AgentCaptureTargetJson {
    fn into_target(self) -> Result<AgentCaptureTarget, JsValue> {
        let kind = self.kind;
        let outline_path = self.outline_path;
        let mut target = match kind.as_str() {
            "inbox" => AgentCaptureTarget::inbox(),
            "datetree" => {
                let date = self
                    .date
                    .ok_or_else(|| JsValue::from_str("capture datetree target requires date"))?;
                AgentCaptureTarget::datetree(date.into_agenda_date())
            }
            "outlinePath" => {
                let path = outline_path.clone().ok_or_else(|| {
                    JsValue::from_str("capture outlinePath target requires outlinePath")
                })?;
                AgentCaptureTarget::outline_path(path)
            }
            "currentSection" => AgentCaptureTarget::current_section(),
            other => {
                return Err(JsValue::from_str(&format!(
                    "invalid capture target kind: {other}"
                )));
            }
        };
        if let Some(source_file) = self.source_file {
            target = target.source_file(source_file);
        }
        if matches!(kind.as_str(), "inbox" | "currentSection") {
            if let Some(outline_path) = outline_path {
                target.outline_path = outline_path;
            }
        }
        if let Some(insert_position) = self.insert_position {
            target = target.insert_position(agent_capture_insert_position(&insert_position)?);
        }
        Ok(target)
    }
}

impl AgentCaptureSourceJson {
    fn into_source(self) -> Result<AgentCaptureSource, JsValue> {
        Ok(AgentCaptureSource {
            kind: match self.kind.as_deref().unwrap_or("conversation") {
                "conversation" => AgentCaptureSourceKind::Conversation,
                "url" => AgentCaptureSourceKind::Url,
                "file" => AgentCaptureSourceKind::File,
                "selection" => AgentCaptureSourceKind::Selection,
                "article" => AgentCaptureSourceKind::Article,
                "code" => AgentCaptureSourceKind::Code,
                "other" => AgentCaptureSourceKind::Other,
                other => {
                    return Err(JsValue::from_str(&format!(
                        "invalid capture source kind: {other}"
                    )));
                }
            },
            actor: self.actor,
            uri: self.uri,
            label: self.label,
        })
    }
}

impl AgentCaptureTimestampJson {
    fn into_timestamp(self) -> AgentCaptureTimestamp {
        let date = AgendaDate::new(self.year, self.month, self.day);
        match (self.hour, self.minute) {
            (Some(hour), Some(minute)) => {
                AgentCaptureTimestamp::with_time(date, AgendaTime { hour, minute })
            }
            _ => AgentCaptureTimestamp::new(date),
        }
    }
}

impl AgentCaptureDateJson {
    fn into_agenda_date(self) -> AgendaDate {
        AgendaDate::new(self.year, self.month, self.day)
    }
}

fn agent_capture_kind(value: &str) -> Result<AgentCaptureKind, JsValue> {
    match value {
        "idea" => Ok(AgentCaptureKind::Idea),
        "articleNote" | "article-note" => Ok(AgentCaptureKind::ArticleNote),
        "task" => Ok(AgentCaptureKind::Task),
        "decision" => Ok(AgentCaptureKind::Decision),
        "preference" => Ok(AgentCaptureKind::Preference),
        "correction" => Ok(AgentCaptureKind::Correction),
        "memoryCandidate" | "memory-candidate" => Ok(AgentCaptureKind::MemoryCandidate),
        "evidence" => Ok(AgentCaptureKind::Evidence),
        "note" => Ok(AgentCaptureKind::Note),
        other => Err(JsValue::from_str(&format!("invalid capture kind: {other}"))),
    }
}

fn agent_capture_memory_policy(value: &str) -> Result<AgentCaptureMemoryPolicy, JsValue> {
    match value {
        "none" => Ok(AgentCaptureMemoryPolicy::None),
        "candidate" => Ok(AgentCaptureMemoryPolicy::Candidate),
        "background" => Ok(AgentCaptureMemoryPolicy::Background),
        "decision" => Ok(AgentCaptureMemoryPolicy::Decision),
        "transient" => Ok(AgentCaptureMemoryPolicy::Transient),
        "supersedes" => Ok(AgentCaptureMemoryPolicy::Supersedes),
        other => Err(JsValue::from_str(&format!(
            "invalid capture memory policy: {other}"
        ))),
    }
}

fn agent_capture_insert_position(value: &str) -> Result<AgentCaptureInsertPosition, JsValue> {
    match value {
        "append" => Ok(AgentCaptureInsertPosition::Append),
        "prepend" => Ok(AgentCaptureInsertPosition::Prepend),
        "firstChild" => Ok(AgentCaptureInsertPosition::FirstChild),
        "lastChild" => Ok(AgentCaptureInsertPosition::LastChild),
        other => Err(JsValue::from_str(&format!(
            "invalid capture insert position: {other}"
        ))),
    }
}
