//! Org-Interactive WebAssembly facade.

use crate::{Org, dto_projection};
use wasm_bindgen::prelude::{JsValue, wasm_bindgen};

#[wasm_bindgen]
impl Org {
    #[wasm_bindgen(js_name = orgInteractiveJson)]
    pub fn org_interactive_json(&self) -> Result<String, JsValue> {
        let document = self.document();
        dto_projection::org_interactive_json(&document).map_err(|error| JsValue::from_str(&error))
    }
}
