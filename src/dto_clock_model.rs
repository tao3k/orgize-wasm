//! Stable TypeScript-facing DTO models for clock projections.

use crate::dto_shared_model::{WasmOrgDuration, WasmSourceRange};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmClockRollupResponse {
    pub(crate) schema_version: u8,
    pub(crate) records: Vec<WasmClockRollupRecord>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmClockTablePlansResponse {
    pub(crate) schema_version: u8,
    pub(crate) plans: Vec<WasmClockTablePlan>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmClockIssuesResponse {
    pub(crate) schema_version: u8,
    pub(crate) findings: Vec<WasmClockIssueFinding>,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmClockIssueProfileRequest {
    #[serde(default, deserialize_with = "deserialize_optional_u64")]
    pub(crate) max_duration_seconds: Option<Option<u64>>,
    #[serde(default, deserialize_with = "deserialize_optional_u64")]
    pub(crate) min_duration_seconds: Option<Option<u64>>,
    #[serde(default, deserialize_with = "deserialize_optional_u64")]
    pub(crate) max_gap_seconds: Option<Option<u64>>,
    pub(crate) gap_ok_around_minutes: Option<Vec<u16>>,
}

fn deserialize_optional_u64<'de, D>(deserializer: D) -> Result<Option<Option<u64>>, D::Error>
where
    D: Deserializer<'de>,
{
    Option::<u64>::deserialize(deserializer).map(Some)
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmClockRollupRecord {
    pub(crate) source: WasmSourceRange,
    pub(crate) outline_path: Vec<String>,
    pub(crate) level: usize,
    pub(crate) title: String,
    pub(crate) local_clock: WasmClockSummary,
    pub(crate) subtree_clock: WasmClockSummary,
    pub(crate) effort: WasmClockEffortSummary,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmClockSummary {
    pub(crate) entries: usize,
    pub(crate) closed_entries: usize,
    pub(crate) running_entries: usize,
    pub(crate) unparsed_entries: usize,
    pub(crate) total_seconds: u64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmClockEffortSummary {
    pub(crate) local: Option<WasmOrgDuration>,
    pub(crate) subtree_total_seconds: u64,
    pub(crate) delta_seconds: i64,
    pub(crate) status: &'static str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmClockIssueFinding {
    pub(crate) source: WasmSourceRange,
    pub(crate) outline_path: Vec<String>,
    pub(crate) level: usize,
    pub(crate) title: String,
    pub(crate) kind: &'static str,
    pub(crate) message: String,
    pub(crate) clock: WasmClockIssueClock,
    pub(crate) previous_clock: Option<WasmClockIssueClock>,
    pub(crate) duration_seconds: Option<u64>,
    pub(crate) threshold_seconds: Option<u64>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmClockIssueClock {
    pub(crate) source: WasmSourceRange,
    pub(crate) raw: String,
    pub(crate) start: Option<WasmClockTableTimeBound>,
    pub(crate) end: Option<WasmClockTableTimeBound>,
    pub(crate) duration_seconds: Option<u64>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmClockTablePlan {
    pub(crate) source: WasmSourceRange,
    pub(crate) name: String,
    pub(crate) parameters: Vec<WasmClockTableParameter>,
    pub(crate) scope: WasmClockTableScope,
    pub(crate) max_level: usize,
    pub(crate) tstart: Option<String>,
    pub(crate) tend: Option<String>,
    pub(crate) time_window: Option<WasmClockTableTimeWindow>,
    pub(crate) match_filter: Option<WasmClockTableMatchFilter>,
    pub(crate) property_columns: Option<WasmClockTablePropertyColumns>,
    pub(crate) rows: Vec<WasmClockTableRow>,
    pub(crate) warnings: Vec<WasmClockTableWarning>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmClockTableParameter {
    pub(crate) key: String,
    pub(crate) value: Option<String>,
    pub(crate) raw: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmClockTableScope {
    pub(crate) kind: &'static str,
    pub(crate) value: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmClockTableTimeWindow {
    pub(crate) source: &'static str,
    pub(crate) start: Option<WasmClockTableTimeBound>,
    pub(crate) end_exclusive: Option<WasmClockTableTimeBound>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmClockTableTimeBound {
    pub(crate) year: u16,
    pub(crate) month: u8,
    pub(crate) day: u8,
    pub(crate) hour: u8,
    pub(crate) minute: u8,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmClockTableMatchFilter {
    pub(crate) expression: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmClockTablePropertyColumns {
    pub(crate) names: Vec<String>,
    pub(crate) inherit: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmClockTableRow {
    pub(crate) source: WasmSourceRange,
    pub(crate) outline_path: Vec<String>,
    pub(crate) level: usize,
    pub(crate) table_level: usize,
    pub(crate) title: String,
    pub(crate) clock: WasmClockSummary,
    pub(crate) effort_total_seconds: u64,
    pub(crate) effort_delta_seconds: i64,
    pub(crate) effort_status: &'static str,
    pub(crate) property_values: Vec<WasmClockTablePropertyValue>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmClockTablePropertyValue {
    pub(crate) name: String,
    pub(crate) value: Option<String>,
    pub(crate) inherited: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmClockTableWarning {
    pub(crate) kind: &'static str,
    pub(crate) message: String,
}
