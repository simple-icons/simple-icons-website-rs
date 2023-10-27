use leptos::{document, ev::keydown, set_timeout};
use leptos_use::use_event_listener;
use wasm_bindgen::JsCast;
use web_sys::{HtmlButtonElement, KeyboardEvent};

pub fn load_keyboard_shortcut_ctrl_and_key_on_click_id(
    button_id: &'static str,
    key: &'static str,
) {
    set_timeout(
        move || {
            let body = document().body().unwrap();
            _ = use_event_listener(body, keydown, move |ev: KeyboardEvent| {
                if ev.ctrl_key() && ev.key() == key {
                    document()
                        .get_element_by_id(button_id)
                        .unwrap()
                        .dyn_into::<HtmlButtonElement>()
                        .unwrap()
                        .click();
                    ev.prevent_default();
                }
            });
        },
        std::time::Duration::from_millis(200),
    );
}
