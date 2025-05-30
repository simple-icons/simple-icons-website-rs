use wasm_bindgen::{JsCast, closure::Closure};

/// Copy a `<canvas>` HTML container as an image to the clipboard.
///
/// Use it with `spawn_local(copy_canvas_container_as_image(container))`.
pub async fn copy_canvas_container_as_image(
    container: web_sys::HtmlCanvasElement,
) {
    let closure: Closure<dyn FnMut(web_sys::Blob)> = Closure::new(
        move |blob: web_sys::Blob| {
            let clipboard_item_object = js_sys::Object::new();
            _ = js_sys::Reflect::set(
                &clipboard_item_object,
                &blob.type_().into(),
                &blob.into(),
            );
            let clipboard_item = web_sys::ClipboardItem::new_with_record_from_str_to_blob_promise(
                &clipboard_item_object,
            ).unwrap();
            let to_copy_array = js_sys::Array::new();
            to_copy_array.push(&clipboard_item);
            let clipboard = web_sys::window().unwrap().navigator().clipboard();
            let _ = clipboard.write(&to_copy_array);
        },
    );
    container.to_blob(closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();
}
