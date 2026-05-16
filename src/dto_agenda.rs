//! Agenda-oriented WebAssembly DTO projection.

use crate::{
    dto_common::{section_source, todo_state},
    dto_document::task_blocker_record,
    dto_model::{
        WasmAgendaBlockSectionPlan, WasmAgendaBlockViewResponse, WasmAgendaViewCard,
        WasmAgendaViewReceipt, WasmAgendaViewResponse, WasmAgendaViewSkip, WasmAgendaViewSortSpec,
        WasmAgendaViewSortValue,
    },
};
use orgize::ast::{
    AgendaBlockViewPlan, AgendaBlockViewQuery, AgendaDate, AgendaTime, AgendaViewPlan,
    AgendaViewQuery, AgendaViewSkipReason, Document, ParsedAnnotation,
};

pub(crate) fn agenda_view_response(
    document: &Document<ParsedAnnotation>,
    query: &AgendaViewQuery,
) -> WasmAgendaViewResponse {
    agenda_view_plan_response(&document.agenda_view_plan(query))
}

pub(crate) fn agenda_block_view_response(
    document: &Document<ParsedAnnotation>,
    query: &AgendaBlockViewQuery,
) -> WasmAgendaBlockViewResponse {
    agenda_block_plan_response(&document.agenda_block_view_plan(query))
}

fn agenda_view_plan_response(plan: &AgendaViewPlan) -> WasmAgendaViewResponse {
    WasmAgendaViewResponse {
        schema_version: 1,
        total_candidates: plan.total_candidates,
        limit: plan.limit,
        sort_strategy: plan
            .sort_strategy
            .iter()
            .map(agenda_view_sort_spec)
            .collect(),
        cards: plan.cards.iter().map(agenda_view_card).collect(),
        skipped: plan.skipped.iter().map(agenda_view_skip).collect(),
    }
}

fn agenda_block_plan_response(plan: &AgendaBlockViewPlan) -> WasmAgendaBlockViewResponse {
    WasmAgendaBlockViewResponse {
        schema_version: 1,
        title: plan.title.clone(),
        total_candidates: plan.total_candidates,
        sections: plan
            .sections
            .iter()
            .map(|section| WasmAgendaBlockSectionPlan {
                index: section.index,
                name: section.name.clone(),
                plan: agenda_view_plan_response(&section.plan),
            })
            .collect(),
    }
}

fn agenda_view_card(card: &orgize::ast::AgendaViewCard) -> WasmAgendaViewCard {
    WasmAgendaViewCard {
        source: section_source(&card.source),
        sorted_position: card.sorted_position,
        kind: card.kind.as_str(),
        display_date: agenda_date(card.display_date),
        target_date: agenda_date(card.target_date),
        target_end_date: card.target_end_date.map(agenda_date),
        time: card.time.map(agenda_time),
        end_time: card.end_time.map(agenda_time),
        title: card.title.clone(),
        category: card
            .category
            .as_ref()
            .map(|category| category.as_str().to_string()),
        todo: card.todo.as_ref().map(|todo| todo.name.clone()),
        todo_state: card.todo.as_ref().map(todo_state),
        effective_tags: card.effective_tags.clone(),
        blockers: card.blockers.iter().map(task_blocker_record).collect(),
        sort_keys: card.sort_keys.iter().map(agenda_view_sort_value).collect(),
        receipts: card.receipts.iter().map(agenda_view_receipt).collect(),
    }
}

fn agenda_view_skip(skip: &orgize::ast::AgendaViewSkip) -> WasmAgendaViewSkip {
    WasmAgendaViewSkip {
        source: section_source(&skip.source),
        sorted_position: skip.sorted_position,
        title: skip.title.clone(),
        reason: skip.reason.as_str(),
        limit: match skip.reason {
            AgendaViewSkipReason::Limit { limit } => Some(limit),
        },
        blockers: skip.blockers.iter().map(task_blocker_record).collect(),
        sort_keys: skip.sort_keys.iter().map(agenda_view_sort_value).collect(),
        receipts: skip.receipts.iter().map(agenda_view_receipt).collect(),
    }
}

fn agenda_view_sort_value(
    sort_value: &orgize::ast::AgendaViewSortValue,
) -> WasmAgendaViewSortValue {
    WasmAgendaViewSortValue {
        key: sort_value.key.as_str(),
        value: sort_value.value.clone(),
    }
}

fn agenda_view_sort_spec(sort_spec: &orgize::ast::AgendaViewSortSpec) -> WasmAgendaViewSortSpec {
    WasmAgendaViewSortSpec {
        key: sort_spec.key.as_str(),
        direction: sort_spec.direction.as_str(),
    }
}

fn agenda_view_receipt(receipt: &orgize::ast::AgendaViewReceipt) -> WasmAgendaViewReceipt {
    WasmAgendaViewReceipt {
        kind: receipt.kind.as_str(),
        message: receipt.message.clone(),
    }
}

fn agenda_date(date: AgendaDate) -> String {
    format!("{:04}-{:02}-{:02}", date.year, date.month, date.day)
}

fn agenda_time(time: AgendaTime) -> String {
    format!("{:02}:{:02}", time.hour, time.minute)
}
