use simple_icons_macros::js_library_version;
use web_sys;
use {leptos::document, wasm_bindgen::prelude::*};

#[wasm_bindgen(module = "/src/controls/download/pdf.js")]
extern "C" {
    pub fn download_pdf_(
        slug: &str,
        errorMessageSchema: String,
        pdfkitVersion: &str,
        blobStreamVersion: &str,
    );
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

    download_pdf_(
        slug,
        msg,
        js_library_version!("pdfkit"),
        js_library_version!("blob-stream"),
    );
}
