use leptos::{document, set_timeout_with_handle};
use std::time::Duration;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{HtmlElement, KeyboardEvent};

pub fn load_keyboard_shortcut_ctrl_and_key_on_click_id(
    button_id: &'static str,
    key: &'static str,
) {
    let closure: Closure<dyn FnMut(KeyboardEvent)> =
        Closure::new(move |ev: KeyboardEvent| {
            let button = document()
                .get_element_by_id(button_id)
                .unwrap()
                .dyn_into::<HtmlElement>()
                .unwrap();
            if ev.ctrl_key() && ev.key() == key {
                button.click();
                ev.prevent_default();
            }
        });

    _ = set_timeout_with_handle(
        move || {
            let body = document().body().unwrap();
            body.add_event_listener_with_callback(
                "keydown",
                closure.as_ref().unchecked_ref(),
            )
            .unwrap();
            closure.forget();
        },
        Duration::from_millis(1000),
    );
}
