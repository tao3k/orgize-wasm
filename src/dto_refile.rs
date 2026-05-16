//! Refile target and plan DTO projection.

use crate::{
    dto_common::section_source,
    dto_refile_model::{
        WasmRefileCreateParentNode, WasmRefileCreateParentPlan, WasmRefilePlan,
        WasmRefilePlanReceipt, WasmRefilePlanResponse, WasmRefilePlanSection, WasmRefileTarget,
        WasmRefileTargetIndexResponse, WasmRefileTargetReceipt, WasmRefileTargetSpec,
        WasmRefileWarning,
    },
};
use orgize::ast::{
    Document, ParsedAnnotation, RefileCreateParentNode, RefileCreateParentPlan, RefilePlan,
    RefilePlanRequest, RefilePlanSection, RefileTarget, RefileTargetIndex, RefileTargetQuery,
    RefileTargetReceipt, RefileTargetSpec, RefileWarning,
};

pub(crate) fn refile_target_index_response(
    document: &Document<ParsedAnnotation>,
    query: &RefileTargetQuery,
) -> WasmRefileTargetIndexResponse {
    refile_target_index(&document.refile_target_index(query))
}

pub(crate) fn refile_target_index(index: &RefileTargetIndex) -> WasmRefileTargetIndexResponse {
    WasmRefileTargetIndexResponse {
        schema_version: 1,
        source_file: index.source_file.clone(),
        outline_path_mode: index.outline_path_mode.as_str(),
        specs: index.specs.iter().map(refile_target_spec).collect(),
        targets: index.targets.iter().map(refile_target).collect(),
        warnings: index.warnings.iter().map(refile_warning).collect(),
    }
}

pub(crate) fn refile_targets(
    document: &Document<ParsedAnnotation>,
    query: &RefileTargetQuery,
) -> Vec<WasmRefileTarget> {
    document
        .refile_target_index(query)
        .targets
        .iter()
        .map(refile_target)
        .collect()
}

pub(crate) fn refile_plan_response(
    document: &Document<ParsedAnnotation>,
    request: &RefilePlanRequest,
) -> WasmRefilePlanResponse {
    WasmRefilePlanResponse {
        schema_version: 1,
        plan: refile_plan(&document.refile_plan(request)),
    }
}

fn refile_plan(plan: &RefilePlan) -> WasmRefilePlan {
    WasmRefilePlan {
        source_file: plan.source_file.clone(),
        action: plan.action.as_str(),
        insert_position: plan.insert_position.as_str(),
        parent_creation: plan.parent_creation.as_str(),
        source: plan.source.as_ref().map(refile_plan_section),
        target: plan.target.as_ref().map(refile_target),
        created_target: plan.created_target.as_ref().map(refile_create_parent_plan),
        receipts: plan
            .receipts
            .iter()
            .map(|receipt| WasmRefilePlanReceipt {
                kind: receipt.kind.as_str(),
                message: receipt.message.clone(),
            })
            .collect(),
        warnings: plan.warnings.iter().map(refile_warning).collect(),
    }
}

fn refile_create_parent_plan(plan: &RefileCreateParentPlan) -> WasmRefileCreateParentPlan {
    WasmRefileCreateParentPlan {
        source_file: plan.source_file.clone(),
        existing_parent: refile_target(&plan.existing_parent),
        target_outline_path: plan.target_outline_path.clone(),
        nodes: plan.nodes.iter().map(refile_create_parent_node).collect(),
        requires_confirmation: plan.requires_confirmation,
    }
}

fn refile_create_parent_node(node: &RefileCreateParentNode) -> WasmRefileCreateParentNode {
    WasmRefileCreateParentNode {
        title: node.title.clone(),
        level: node.level,
        outline_path: node.outline_path.clone(),
        display: node.display.clone(),
    }
}

fn refile_plan_section(section: &RefilePlanSection) -> WasmRefilePlanSection {
    WasmRefilePlanSection {
        source_file: section.source_file.clone(),
        source: section_source(&section.source),
        level: section.level,
        title: section.title.clone(),
        outline_path: section.outline_path.clone(),
        local_ids: section.local_ids.clone(),
    }
}

fn refile_target(target: &RefileTarget) -> WasmRefileTarget {
    WasmRefileTarget {
        source_file: target.source_file.clone(),
        source: section_source(&target.source),
        level: target.level,
        title: target.title.clone(),
        outline_path: target.outline_path.clone(),
        display: target.display.clone(),
        receipts: target.receipts.iter().map(refile_target_receipt).collect(),
    }
}

fn refile_target_receipt(receipt: &RefileTargetReceipt) -> WasmRefileTargetReceipt {
    WasmRefileTargetReceipt {
        spec: refile_target_spec(&receipt.spec),
        message: receipt.message.clone(),
    }
}

fn refile_target_spec(spec: &RefileTargetSpec) -> WasmRefileTargetSpec {
    WasmRefileTargetSpec {
        kind: spec.kind().as_str(),
        value: spec.value(),
    }
}

fn refile_warning(warning: &RefileWarning) -> WasmRefileWarning {
    WasmRefileWarning {
        kind: warning.kind.as_str(),
        message: warning.message.clone(),
    }
}
