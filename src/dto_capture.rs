//! Agent capture plan DTO projection.

use crate::dto_capture_model::{
    WasmAgentCaptureApplication, WasmAgentCaptureApplicationPrecondition, WasmAgentCaptureDate,
    WasmAgentCapturePlan, WasmAgentCapturePlanResponse, WasmAgentCaptureReceipt,
    WasmAgentCaptureTarget, WasmAgentCaptureWarning,
};
use orgize::ast::{
    agent_capture_plan, AgendaDate, AgentCaptureApplication, AgentCapturePlan, AgentCaptureRequest,
    AgentCaptureTarget,
};

pub(crate) fn agent_capture_plan_response(
    request: &AgentCaptureRequest,
) -> WasmAgentCapturePlanResponse {
    WasmAgentCapturePlanResponse {
        schema_version: 1,
        plan: wasm_agent_capture_plan(&agent_capture_plan(request)),
    }
}

fn wasm_agent_capture_plan(plan: &AgentCapturePlan) -> WasmAgentCapturePlan {
    WasmAgentCapturePlan {
        target: WasmAgentCaptureTarget {
            kind: plan.target.kind.as_str(),
            source_file: plan.target.source_file.clone(),
            outline_path: plan.target.outline_path.clone(),
            date: plan.target.date.map(capture_date),
            insert_position: plan.target.insert_position.as_str(),
        },
        org_entry: plan.org_entry.clone(),
        application: wasm_agent_capture_application(&plan.application),
        receipts: plan
            .receipts
            .iter()
            .map(|receipt| WasmAgentCaptureReceipt {
                kind: receipt.kind.as_str(),
                message: receipt.message.clone(),
            })
            .collect(),
        warnings: plan
            .warnings
            .iter()
            .map(|warning| WasmAgentCaptureWarning {
                kind: warning.kind.as_str(),
                message: warning.message.clone(),
            })
            .collect(),
        requires_confirmation: plan.requires_confirmation,
    }
}

fn wasm_agent_capture_application(
    application: &AgentCaptureApplication,
) -> WasmAgentCaptureApplication {
    WasmAgentCaptureApplication {
        action: application.action.as_str(),
        target: wasm_agent_capture_target(&application.target),
        preconditions: application
            .preconditions
            .iter()
            .map(|precondition| WasmAgentCaptureApplicationPrecondition {
                kind: precondition.kind.as_str(),
                message: precondition.message.clone(),
            })
            .collect(),
    }
}

fn wasm_agent_capture_target(target: &AgentCaptureTarget) -> WasmAgentCaptureTarget {
    WasmAgentCaptureTarget {
        kind: target.kind.as_str(),
        source_file: target.source_file.clone(),
        outline_path: target.outline_path.clone(),
        date: target.date.map(capture_date),
        insert_position: target.insert_position.as_str(),
    }
}

fn capture_date(date: AgendaDate) -> WasmAgentCaptureDate {
    WasmAgentCaptureDate {
        year: date.year,
        month: date.month,
        day: date.day,
    }
}
