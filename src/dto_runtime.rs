//! Runtime metadata WebAssembly DTO projection.

use crate::{
    dto_common::section_source,
    dto_runtime_model::{
        WasmFeedStatusRecord, WasmMobileFlaggedSection, WasmMobileIndexLink, WasmMobileOriginalId,
        WasmMobilePriorityDeclaration, WasmMobileProperty, WasmMobileReadonlyKeyword,
        WasmMobileSyncMetadata, WasmRuntimeMetadataBoundary, WasmRuntimeMetadataResponse,
        WasmRuntimeMetadataWarning, WasmTimerRecord,
    },
};
use orgize::ast::{Document, ParsedAnnotation, RuntimeMetadataPlan};

pub(crate) fn runtime_metadata_response(
    document: &Document<ParsedAnnotation>,
) -> WasmRuntimeMetadataResponse {
    runtime_metadata_response_from_plan(&document.runtime_metadata_plan())
}

pub(crate) fn runtime_metadata_response_from_plan(
    plan: &RuntimeMetadataPlan,
) -> WasmRuntimeMetadataResponse {
    WasmRuntimeMetadataResponse {
        schema_version: 1,
        feeds: plan
            .feeds
            .iter()
            .map(|feed| WasmFeedStatusRecord {
                source: section_source(&feed.source),
                section_title: feed.section_title.clone(),
                drawer: feed.drawer.as_str().to_string(),
                raw: feed.raw.clone(),
                entry_count: feed.entry_count,
                readable: feed.readable,
            })
            .collect(),
        timers: plan
            .timers
            .iter()
            .map(|timer| WasmTimerRecord {
                source: section_source(&timer.source),
                outline_path: timer.outline_path.clone(),
                context: timer.context.as_str(),
                raw: timer.raw.clone(),
                total_seconds: timer.total_seconds,
            })
            .collect(),
        mobile: WasmMobileSyncMetadata {
            readonly: plan
                .mobile
                .readonly
                .iter()
                .map(|readonly| WasmMobileReadonlyKeyword {
                    source: section_source(&readonly.source),
                    value: readonly.value.clone(),
                })
                .collect(),
            all_priorities: plan
                .mobile
                .all_priorities
                .iter()
                .map(|priorities| WasmMobilePriorityDeclaration {
                    source: section_source(&priorities.source),
                    values: priorities.values.clone(),
                    raw: priorities.raw.clone(),
                })
                .collect(),
            index_links: plan
                .mobile
                .index_links
                .iter()
                .map(|link| WasmMobileIndexLink {
                    source: section_source(&link.source),
                    title: link.title.clone(),
                    file: link.file.clone(),
                    description: link.description.clone(),
                })
                .collect(),
            flagged_sections: plan
                .mobile
                .flagged_sections
                .iter()
                .map(|section| WasmMobileFlaggedSection {
                    source: section_source(&section.source),
                    outline_path: section.outline_path.clone(),
                    title: section.title.clone(),
                    original_id: section.original_id.clone(),
                    mobile_properties: section
                        .mobile_properties
                        .iter()
                        .map(|property| WasmMobileProperty {
                            source: section_source(&property.source),
                            key: property.key.clone(),
                            value: property.value.clone(),
                        })
                        .collect(),
                })
                .collect(),
            original_ids: plan
                .mobile
                .original_ids
                .iter()
                .map(|original_id| WasmMobileOriginalId {
                    source: section_source(&original_id.source),
                    outline_path: original_id.outline_path.clone(),
                    title: original_id.title.clone(),
                    value: original_id.value.clone(),
                })
                .collect(),
        },
        boundaries: plan
            .boundaries
            .iter()
            .map(|boundary| WasmRuntimeMetadataBoundary {
                kind: boundary.kind.as_str(),
                message: boundary.message.clone(),
            })
            .collect(),
        warnings: plan
            .warnings
            .iter()
            .map(|warning| WasmRuntimeMetadataWarning {
                kind: warning.kind.as_str(),
                message: warning.message.clone(),
            })
            .collect(),
    }
}
