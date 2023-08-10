use crate::Ids;
use leptos::{document, ev::MouseEvent, set_timeout_with_handle, window, *};
use log;
use std::time::Duration;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures;
use web_sys;

fn on_copied(button: web_sys::HtmlElement) {
    button.class_list().add_1("copied").unwrap();
    _ = set_timeout_with_handle(
        move || {
            button.class_list().remove_1("copied").unwrap();

            // Unset focus
            button.blur().unwrap();
        },
        Duration::from_millis(1000),
    );
}

fn on_error(value: &str, err: &JsValue) {
    log::error!("Error copying value '{}' to clipboard: {:?}", value, err);
}

/// Copy a value to the clipboard and sets a transition in copy button
/// to properly show the user that the value has been copied.
///
/// See the `.copy-button-*` classes components in stylesheet.
pub async fn copy_setting_copied_transition_in_element(
    value: String,
    button: web_sys::HtmlElement,
) {
    match window().navigator().clipboard() {
        Some(navigator_clipboard) => {
            #[cfg(debug_assertions)]
            log::debug!(
                "Copying value '{}' to clipboard using Navigator.Clipboard API",
                &value
            );

            match wasm_bindgen_futures::JsFuture::from(
                navigator_clipboard.write_text(&value),
            )
            .await
            {
                Ok(_) => on_copied(button),
                Err(err) => on_error(&value, &err),
            }
        }
        None => {
            #[cfg(debug_assertions)]
            log::debug!(
                "Copying value '{}' to clipboard using Document.execCommand",
                &value
            );

            let document = document();
            let copy_input = document
                .get_element_by_id(Ids::CopyInput.as_str())
                .unwrap()
                .dyn_into::<web_sys::HtmlInputElement>()
                .unwrap();
            copy_input.set_value(&value);
            copy_input.select();
            match document
                .dyn_into::<web_sys::HtmlDocument>()
                .unwrap()
                .exec_command("copy")
            {
                Ok(_) => on_copied(button),
                Err(err) => on_error(&value, &err),
            }
        }
    }
}

/// Copy the inner text of the event target to the clipboard
pub(crate) fn copy_inner_text_on_click(ev: MouseEvent) {
    let target = event_target::<web_sys::HtmlElement>(&ev);
    let value = target.text_content().unwrap();
    spawn_local(copy_setting_copied_transition_in_element(value, target));
}

#[component]
pub fn CopyInput() -> impl IntoView {
    view! { <input aria-hidden=true class="hidden" id=Ids::CopyInput.as_str() tabindex=-1/> }
}
