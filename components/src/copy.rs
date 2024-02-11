use crate::Ids;
use leptos::{ev::MouseEvent, *};
use leptos_use::{use_clipboard, UseClipboardReturn};
use log;
use wasm_bindgen::{closure::Closure, prelude::*, JsCast};
use web_sys;

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

/// Copy a value to the clipboard and sets a transition in copy button
/// to properly show the user that the value has been copied.
///
/// See the `.copy-button-*` classes components in stylesheet.
pub fn copy_and_set_copied_transition(value: String, el: web_sys::HtmlElement) {
    let UseClipboardReturn {
        is_supported, copy, ..
    } = use_clipboard();

    if !is_supported() {
        log::error!("Clipboard API not supported by the browser");
        return;
    }

    copy(&value);
    set_copied_class(el);
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
            match window().navigator().clipboard() {
                Some(_navigator_clipboard) => {
                    #[cfg(debug_assertions)]
                    log::debug!(
                    "Copying item to clipboard using Navigator.Clipboard API",
                );
                    copy_blob_as_image_with_navigator_clipboard(&blob);
                }
                None => ::log::error!(
                    "Navigator.Clipboard API required to copy PNG images"
                ),
            }
        });
    container.to_blob(closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();
}

/// Copy the inner text of the event target to the clipboard
pub(crate) fn copy_inner_text_on_click(ev: MouseEvent) {
    let target = event_target::<web_sys::HtmlElement>(&ev);
    let value = target.text_content().unwrap();
    copy_and_set_copied_transition(value, target);
}

/// Hidden input to copy values to the Clipboard
///
/// Used when the browser does not implement the Navigator.Clipboard API
#[component]
pub fn CopyInput() -> impl IntoView {
    view! { <input aria-hidden=true class="hidden" id=Ids::CopyInput.as_str() tabindex=-1/> }
}
