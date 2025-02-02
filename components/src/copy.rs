use leptos::{ev::MouseEvent, prelude::*, task::spawn_local};
use leptos_use::{use_clipboard, UseClipboardReturn};
use wasm_bindgen::JsCast;
use web_sys_simple_fetch::fetch_text;

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

/// Copy image children source content to clipboard
pub(crate) fn copy_child_img_src_content_from_mouse_event(ev: MouseEvent) {
    let target = event_target::<web_sys::HtmlElement>(&ev);
    if let Some(src) = target
        .children()
        .item(0)
        .unwrap()
        .unchecked_into::<web_sys::HtmlImageElement>()
        .get_attribute("src")
    {
        spawn_local(async move {
            if let Ok(content) = fetch_text(&src).await {
                copy_and_set_copied_transition(&content, target)
            }
        });
    }
}
