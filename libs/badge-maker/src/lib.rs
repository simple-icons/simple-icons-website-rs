use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/lib.js")]
extern "C" {
    pub fn make_badge(
        label: &str,
        message: &str,
        color: &str,
        style: &str,
        logo_base_64: &str,
    ) -> String;
}
