use leptos::prelude::{document, window, GetUntracked, RwSignal, Update};
use simple_icons_website_ids::Ids;
use std::boxed::Box;
use wasm_bindgen::JsCast;

pub(crate) fn listen_keyboard_shortcuts() {
    let keys_pressed = RwSignal::new(vec![]);

    let keydown_listener = wasm_bindgen::closure::Closure::wrap(Box::new(
        move |event: web_sys::KeyboardEvent| {
            keys_pressed.update(|keys| {
                keys.push(event.code().to_lowercase());
            });

            let pressed_codes = keys_pressed.get_untracked();
            let code = event.code().to_lowercase();

            if pressed_codes.len() != 2 {
                return;
            }
            let previous_code = pressed_codes[0].clone();

            let click_button = |button_id: &str| {
                document()
                    .get_element_by_id(button_id)
                    .unwrap()
                    .unchecked_into::<web_sys::HtmlInputElement>()
                    .click();
                keys_pressed.update(|keys| {
                    *keys = vec![];
                });
            };

            // Ctrl + Up
            if code == "arrowup"
                && ["controlleft", "controlright"]
                    .contains(&previous_code.as_str())
            {
                click_button(Ids::PreviewUploadSvgButton.as_str());
                event.prevent_default();
            } else if code == "arrowdown"
                && ["controlleft", "controlright"]
                    .contains(&previous_code.as_str())
            {
                click_button(Ids::PreviewDownloadSvgButton.as_str());
                event.prevent_default();
            } else if code == "keyc"
                && ["controlleft", "controlright"]
                    .contains(&previous_code.as_str())
            {
                // Ctrl + C
                let click_preview_copy_button =
                    || click_button(Ids::PreviewCopyButton.as_str());
                if let Ok(Some(selection)) = window().get_selection() {
                    // don't copy the view when there is text selected
                    if selection.type_() != "Range" {
                        click_preview_copy_button();
                        event.prevent_default();
                    }
                } else {
                    click_preview_copy_button();
                    event.prevent_default();
                }
            } else if code == "keys"
                && ["controlleft", "controlright"]
                    .contains(&previous_code.as_str())
            {
                click_button(Ids::PreviewSaveButton.as_str());
                event.prevent_default();
            }
        },
    )
        as Box<dyn Fn(_)>);

    let keyup_listener = wasm_bindgen::closure::Closure::wrap(Box::new(
        move |event: web_sys::KeyboardEvent| {
            let code = event.code().to_lowercase();
            keys_pressed.update(|keys| {
                keys.retain(|key| key != &code);
            });
        },
    )
        as Box<dyn Fn(_)>);

    document()
        .add_event_listener_with_callback(
            "keydown",
            keydown_listener.as_ref().unchecked_ref(),
        )
        .expect("Failed to add keydown event listener");
    document()
        .add_event_listener_with_callback(
            "keyup",
            keyup_listener.as_ref().unchecked_ref(),
        )
        .expect("Failed to add keyup event listener");

    keydown_listener.forget();
    keyup_listener.forget();
}
