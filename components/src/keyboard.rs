use leptos::{document, ev::keydown, set_timeout, wasm_bindgen::JsCast};
use leptos_use::use_event_listener;

pub fn load_keyboard_shortcut_ctrl_and_key_on_click_id(
    button_id: &'static str,
    key: &'static str,
) {
    set_timeout(
        move || {
            _ = use_event_listener(
                document().body().unwrap(),
                keydown,
                move |ev: web_sys::KeyboardEvent| {
                    if ev.ctrl_key() && ev.key() == key {
                        document()
                            .get_element_by_id(button_id)
                            .unwrap()
                            .dyn_into::<web_sys::HtmlElement>()
                            .unwrap()
                            .click();
                        ev.prevent_default();
                    }
                },
            );
        },
        std::time::Duration::from_millis(200),
    );
}
