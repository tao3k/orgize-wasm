//! Clock rollup and clocktable WebAssembly DTO projection.

use crate::{
    dto_clock_model::{
        WasmClockEffortSummary, WasmClockIssueClock, WasmClockIssueFinding,
        WasmClockIssuesResponse, WasmClockRollupRecord, WasmClockRollupResponse, WasmClockSummary,
        WasmClockTableMatchFilter, WasmClockTableParameter, WasmClockTablePlan,
        WasmClockTablePlansResponse, WasmClockTablePropertyColumns, WasmClockTablePropertyValue,
        WasmClockTableRow, WasmClockTableScope, WasmClockTableTimeBound, WasmClockTableTimeWindow,
        WasmClockTableWarning,
    },
    dto_common::{org_duration, section_source},
};
use orgize::ast::{
    ClockIssueClock, ClockIssueFinding, ClockIssueProfile, ClockRollupRecord, ClockSummary,
    ClockTablePlan, ClockTableTimeBound, ClockTableTimeWindow, Document, ParsedAnnotation,
};

pub(crate) fn clock_rollups_response(
    document: &Document<ParsedAnnotation>,
) -> WasmClockRollupResponse {
    WasmClockRollupResponse {
        schema_version: 1,
        records: clock_rollup_records(document),
    }
}

pub(crate) fn clock_rollup_records(
    document: &Document<ParsedAnnotation>,
) -> Vec<WasmClockRollupRecord> {
    document
        .clock_rollup_records()
        .iter()
        .map(clock_rollup_record)
        .collect()
}

pub(crate) fn clock_table_plans_response(
    document: &Document<ParsedAnnotation>,
) -> WasmClockTablePlansResponse {
    WasmClockTablePlansResponse {
        schema_version: 1,
        plans: clock_table_plans(document),
    }
}

pub(crate) fn clock_table_plans(document: &Document<ParsedAnnotation>) -> Vec<WasmClockTablePlan> {
    document
        .clock_table_plans()
        .iter()
        .map(clock_table_plan)
        .collect()
}

pub(crate) fn clock_issues_response(
    document: &Document<ParsedAnnotation>,
    profile: &ClockIssueProfile,
) -> WasmClockIssuesResponse {
    WasmClockIssuesResponse {
        schema_version: 1,
        findings: clock_issue_findings(document, profile),
    }
}

pub(crate) fn clock_issue_findings(
    document: &Document<ParsedAnnotation>,
    profile: &ClockIssueProfile,
) -> Vec<WasmClockIssueFinding> {
    document
        .clock_issue_findings_with_profile(profile)
        .iter()
        .map(clock_issue_finding)
        .collect()
}

fn clock_rollup_record(record: &ClockRollupRecord) -> WasmClockRollupRecord {
    WasmClockRollupRecord {
        source: section_source(&record.source),
        outline_path: record.outline_path.clone(),
        level: record.level,
        title: record.title.clone(),
        local_clock: clock_summary(record.local_clock),
        subtree_clock: clock_summary(record.subtree_clock),
        effort: WasmClockEffortSummary {
            local: record.effort.local.as_ref().map(org_duration),
            subtree_total_seconds: record.effort.subtree_total_seconds,
            delta_seconds: record.effort.delta_seconds,
            status: record.effort.status.as_str(),
        },
    }
}

fn clock_summary(summary: ClockSummary) -> WasmClockSummary {
    WasmClockSummary {
        entries: summary.entries,
        closed_entries: summary.closed_entries,
        running_entries: summary.running_entries,
        unparsed_entries: summary.unparsed_entries,
        total_seconds: summary.total_seconds,
    }
}

fn clock_issue_finding(finding: &ClockIssueFinding) -> WasmClockIssueFinding {
    WasmClockIssueFinding {
        source: section_source(&finding.source),
        outline_path: finding.outline_path.clone(),
        level: finding.level,
        title: finding.title.clone(),
        kind: finding.kind.as_str(),
        message: finding.message.clone(),
        clock: clock_issue_clock(&finding.clock),
        previous_clock: finding.previous_clock.as_ref().map(clock_issue_clock),
        duration_seconds: finding.duration_seconds,
        threshold_seconds: finding.threshold_seconds,
    }
}

fn clock_issue_clock(clock: &ClockIssueClock) -> WasmClockIssueClock {
    WasmClockIssueClock {
        source: section_source(&clock.source),
        raw: clock.raw.clone(),
        start: clock.start.map(clock_table_time_bound),
        end: clock.end.map(clock_table_time_bound),
        duration_seconds: clock.duration_seconds,
    }
}

fn clock_table_plan(plan: &ClockTablePlan) -> WasmClockTablePlan {
    WasmClockTablePlan {
        source: section_source(&plan.source),
        name: plan.name.clone(),
        parameters: plan
            .parameters
            .iter()
            .map(|parameter| WasmClockTableParameter {
                key: parameter.key.clone(),
                value: parameter.value.clone(),
                raw: parameter.raw.clone(),
            })
            .collect(),
        scope: WasmClockTableScope {
            kind: plan.scope.kind.as_str(),
            value: plan.scope.value.clone(),
        },
        max_level: plan.max_level,
        tstart: plan.tstart.clone(),
        tend: plan.tend.clone(),
        time_window: plan.time_window.as_ref().map(clock_table_time_window),
        match_filter: plan
            .match_filter
            .as_ref()
            .map(|filter| WasmClockTableMatchFilter {
                expression: filter.expression.clone(),
            }),
        property_columns: plan.property_columns.as_ref().map(|columns| {
            WasmClockTablePropertyColumns {
                names: columns.names.clone(),
                inherit: columns.inherit,
            }
        }),
        rows: plan
            .rows
            .iter()
            .map(|row| WasmClockTableRow {
                source: section_source(&row.source),
                outline_path: row.outline_path.clone(),
                level: row.level,
                table_level: row.table_level,
                title: row.title.clone(),
                clock: clock_summary(row.clock),
                effort_total_seconds: row.effort_total_seconds,
                effort_delta_seconds: row.effort_delta_seconds,
                effort_status: row.effort_status.as_str(),
                property_values: row
                    .property_values
                    .iter()
                    .map(|property| WasmClockTablePropertyValue {
                        name: property.name.clone(),
                        value: property.value.clone(),
                        inherited: property.inherited,
                    })
                    .collect(),
            })
            .collect(),
        warnings: plan
            .warnings
            .iter()
            .map(|warning| WasmClockTableWarning {
                kind: warning.kind.as_str(),
                message: warning.message.clone(),
            })
            .collect(),
    }
}

fn clock_table_time_window(window: &ClockTableTimeWindow) -> WasmClockTableTimeWindow {
    WasmClockTableTimeWindow {
        source: window.source.as_str(),
        start: window.start.map(clock_table_time_bound),
        end_exclusive: window.end_exclusive.map(clock_table_time_bound),
    }
}

fn clock_table_time_bound(bound: ClockTableTimeBound) -> WasmClockTableTimeBound {
    WasmClockTableTimeBound {
        year: bound.year,
        month: bound.month,
        day: bound.day,
        hour: bound.hour,
        minute: bound.minute,
    }
}
