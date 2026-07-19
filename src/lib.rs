//! WebAssembly bindings for `orgize`.

mod bindings;
mod dto_agenda;
mod dto_attachment_inventory;
mod dto_attachment_inventory_model;
mod dto_capture;
mod dto_capture_model;
mod dto_capture_request;
mod dto_clock;
mod dto_clock_model;
mod dto_common;
mod dto_contracts;
mod dto_crypt;
mod dto_crypt_model;
mod dto_document;
mod dto_dynamic_block;
mod dto_dynamic_block_model;
mod dto_index;
mod dto_memory;
mod dto_memory_model;
mod dto_model;
mod dto_projection;
mod dto_property_profile;
mod dto_property_profile_model;
mod dto_refile;
mod dto_refile_model;
mod dto_refile_request;
mod dto_runtime;
mod dto_runtime_model;
mod dto_sdd;
mod dto_sdd_model;
mod dto_shared_model;
mod dto_source_block_model;
mod dto_source_blocks;

pub use bindings::Org;
