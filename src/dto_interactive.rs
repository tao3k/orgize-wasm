//! Org-Interactive DTO projection helpers.

use crate::{
    dto_common::source_position,
    dto_interactive_model::{
        WasmOrgInteractiveCategory, WasmOrgInteractiveChoice, WasmOrgInteractiveChoiceEntry,
        WasmOrgInteractiveResponse,
    },
    dto_shared_model::WasmSourceRange,
};
use orgize::ast::{Document, OrgInteractiveChoice, ParsedAnnotation};

pub(crate) fn org_interactive_response(
    document: &Document<ParsedAnnotation>,
) -> Result<WasmOrgInteractiveResponse, String> {
    document
        .org_interactive_choices()
        .map(|choices| WasmOrgInteractiveResponse {
            schema_version: 1,
            choices: choices.iter().map(org_interactive_choice).collect(),
        })
        .map_err(|error| error.to_string())
}

fn org_interactive_choice(choice: &OrgInteractiveChoice) -> WasmOrgInteractiveChoice {
    WasmOrgInteractiveChoice {
        source: WasmSourceRange {
            start: source_position(choice.source.start),
            end: source_position(choice.source.end),
            range_start: choice.source.range_start,
            range_end: choice.source.range_end,
        },
        id: choice.id.clone(),
        method: choice.method.clone(),
        stage: choice.stage.clone(),
        group: choice.group.clone(),
        target: choice.target.clone(),
        create: choice.create.clone(),
        info: choice.info.clone(),
        categories: choice
            .categories
            .iter()
            .map(|category| WasmOrgInteractiveCategory {
                key: category.key.clone(),
                value: category.value.clone(),
                detail: category.detail,
            })
            .collect(),
        entries: choice
            .entries
            .iter()
            .map(|entry| WasmOrgInteractiveChoiceEntry {
                number: entry.number.clone(),
                id: entry.id.clone(),
                contract: entry.contract.clone(),
                full: entry.full.clone(),
                use_if: entry.use_if.clone(),
            })
            .collect(),
    }
}
