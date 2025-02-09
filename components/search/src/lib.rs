use simple_icons_website_ids::Ids;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

pub fn get_search_input() -> HtmlInputElement {
    leptos::prelude::document()
        .get_element_by_id(Ids::SearchInput.as_str())
        .unwrap()
        .unchecked_into::<HtmlInputElement>()
}

/// Set the focus on the search bar
pub fn focus_search_bar() {
    let input = get_search_input();
    _ = input.blur();
    _ = input.focus();
}
