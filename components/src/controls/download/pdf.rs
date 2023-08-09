use leptos::document;
use wasm_bindgen::prelude::*;
use web_sys;

#[wasm_bindgen(module = "/src/controls/download/pdf.js")]
extern "C" {
    pub fn download_pdf_(slug: &str, errorMessageSchema: String);
}

pub fn download_pdf(slug: &str) {
    let msg = document()
        .query_selector("[data-error-generating-pdf-msg]")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::HtmlButtonElement>()
        .unwrap()
        .get_attribute("data-error-generating-pdf-msg")
        .unwrap();

    download_pdf_(slug, msg);
}
