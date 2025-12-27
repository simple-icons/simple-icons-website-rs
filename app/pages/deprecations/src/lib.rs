pub(crate) mod app;

use crate::app::App;
use leptos::{mount::mount_to_body, prelude::document};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    // Remove #loader and body background
    if let Some(loader) = document().get_element_by_id("loader") {
        loader.remove();
    }
    if let Some(body) = document().body() {
        body.remove_attribute("style").ok();
    }

    mount_to_body(App);

    Ok(())
}
