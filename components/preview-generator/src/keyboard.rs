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
                click_button(Ids::PreviewUploadSVGButton.as_str());
                event.prevent_default();
            } else if code == "arrowdown"
                && ["controlleft", "controlright"]
                    .contains(&previous_code.as_str())
            {
                click_button(Ids::PreviewDownloadSVGButton.as_str());
                event.prevent_default();
            } else if code == "keyc"
                && ["controlleft", "controlright"]
                    .contains(&previous_code.as_str())
            {
                // Ctrl + C
                if let Ok(Some(_)) = window().get_selection() {
                    // don't copy the view because there is some text selected
                } else {
                    click_button(Ids::PreviewCopyButton.as_str());
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
