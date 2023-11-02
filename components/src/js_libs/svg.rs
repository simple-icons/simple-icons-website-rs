use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/js_libs/svg_bridge.js")]
extern "C" {
    pub fn svg_path_bbox_(path: &str) -> JsValue;
}

pub fn svg_path_bbox(path: &str) -> ((f64, f64, f64, f64), Option<String>) {
    let ret = js_sys::Array::from(&svg_path_bbox_(path));
    let error_msg = ret.get(1);
    if error_msg.is_null() {
        let array = js_sys::Array::from(&ret.get(0));
        let x1 = array.get(0).as_f64().unwrap();
        let y1 = array.get(1).as_f64().unwrap();
        let x2 = array.get(2).as_f64().unwrap();
        let y2 = array.get(3).as_f64().unwrap();
        ((x1, y1, x2, y2), None)
    } else {
        ((0.0, 0.0, 0.0, 0.0), error_msg.as_string())
    }
}
