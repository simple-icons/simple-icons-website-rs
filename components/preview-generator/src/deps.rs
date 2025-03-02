use simple_icons_macros::js_library_version;
use wasm_bindgen::prelude::*;

static BADGE_MAKER_VERSION: &str = js_library_version!("badge-maker");
static SVG_PATH_BBOX_VERSION: &str = js_library_version!("svg-path-bbox");

#[wasm_bindgen(module = "/src/deps.js")]
extern "C" {
    pub fn add_scripts_(badgeMakerVersion: &str, svgPathBboxVersion: &str);
    pub fn is_badge_maker_loaded() -> bool;
}

pub fn add_preview_generator_scripts() {
    add_scripts_(BADGE_MAKER_VERSION, SVG_PATH_BBOX_VERSION);
}
