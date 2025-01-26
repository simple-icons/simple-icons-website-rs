use crate::fetch::fetch_text;
use leptos::{ev::MouseEvent, prelude::*, task::spawn_local};
use leptos_use::{use_clipboard, UseClipboardReturn};
use wasm_bindgen::{closure::Closure, prelude::*, JsCast};

fn set_copied_class(el: web_sys::HtmlElement) {
    el.class_list().add_1("copied").unwrap();
    set_timeout(
        move || {
            el.class_list().remove_1("copied").unwrap();

            // Unset focus
            el.blur().unwrap();
        },
        std::time::Duration::from_millis(1000),
    );
}

/// Copy a value to the clipboard and sets temporally a class in the element
/// to properly show the user that the value has been copied.
///
/// See the `.copy-button-*` classes in stylesheets.
pub fn copy_and_set_copied_transition(value: &str, el: web_sys::HtmlElement) {
    let UseClipboardReturn {
        is_supported, copy, ..
    } = use_clipboard();

    if !is_supported() {
        leptos::logging::error!("Clipboard API not supported by the browser");
        return;
    }

    copy(value);
    set_copied_class(el);
}

/// Copy a text to the clipboard
pub fn copy_text(text: &str) {
    let UseClipboardReturn {
        copy, is_supported, ..
    } = use_clipboard();

    if !is_supported() {
        leptos::logging::error!("Clipboard API not supported by the browser");
        return;
    }

    copy(text);
}

#[wasm_bindgen(module = "/src/copy.js")]
extern "C" {
    pub fn copy_blob_as_image_with_navigator_clipboard(blob: &web_sys::Blob);
}

pub async fn copy_canvas_container_as_image(
    container: web_sys::HtmlCanvasElement,
) {
    let closure: Closure<dyn FnMut(web_sys::Blob)> =
        Closure::new(move |blob: web_sys::Blob| {
            copy_blob_as_image_with_navigator_clipboard(&blob);
        });
    container.to_blob(closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();
}

/// Copy the inner text of the event target to the clipboard
pub(crate) fn copy_inner_text_on_click(ev: MouseEvent) {
    let target = event_target::<web_sys::HtmlElement>(&ev);
    let value = target.text_content().unwrap();
    copy_and_set_copied_transition(&value, target);
}

/// Copy image children source content to clipboard
pub(crate) fn copy_child_img_src_content_from_mouse_event(ev: MouseEvent) {
    let target = event_target::<web_sys::HtmlElement>(&ev);
    let src = target
        .children()
        .item(0)
        .unwrap()
        .dyn_into::<web_sys::HtmlImageElement>()
        .unwrap()
        .get_attribute("src")
        .unwrap();
    spawn_local(async move {
        if let Ok(content) = fetch_text(&src).await {
            copy_and_set_copied_transition(&content, target)
        }
    });
}
