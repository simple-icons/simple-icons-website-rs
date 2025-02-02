pub mod fuzzy;

use crate::controls::layout::LayoutSignal;
use crate::controls::order::{
    set_order_mode, OrderMode, OrderModeSignal, OrderModeVariant,
};
use crate::grid::{IconsGrid, IconsGridSignal, IconsIndexSignal, ICONS};
use crate::Ids;
use fuzzy::{build_searcher, search};
use leptos::{
    prelude::{document, *},
    task::spawn_local,
    wasm_bindgen::JsCast,
};
use leptos_fluent::tr;
use simple_icons_website_storage::LocalStorage;
use simple_icons_website_types::SimpleIcon;
use simple_icons_website_url as Url;
use web_sys::HtmlInputElement;
use web_sys_simple_events::dispatch_input_event_on_input;

#[derive(Copy, Clone)]
pub struct SearchValueSignal(pub RwSignal<String>);

pub fn get_search_input() -> HtmlInputElement {
    document()
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

pub fn provide_search_context(icons: Vec<&'static SimpleIcon>) -> String {
    let initial_search_value = initial_search_value(icons);
    provide_context(SearchValueSignal(RwSignal::new(
        initial_search_value.clone(),
    )));

    initial_search_value
}

fn initial_search_value(icons: Vec<&'static SimpleIcon>) -> String {
    let search_value = match Url::params::get(&Url::params::Names::Query) {
        Some(value) => {
            set_search_value_on_localstorage(value.as_str());
            value
        }
        None => match get_search_value_from_localstorage() {
            Some(value) => {
                Url::params::update(&Url::params::Names::Query, value.as_str());
                set_search_value_on_localstorage(value.as_str());
                value
            }
            None => String::new(),
        },
    };

    init_searcher(icons);
    search_value
}

pub fn get_search_value_from_localstorage() -> Option<String> {
    LocalStorage::get(LocalStorage::Keys::SearchValue)
        .as_ref()
        .and_then(|value| match value.is_empty() {
            true => None,
            false => Some(value.clone()),
        })
}

pub fn set_search_value_on_localstorage(search_value: &str) {
    LocalStorage::set(LocalStorage::Keys::SearchValue, search_value);
}

pub fn fire_on_search_event() {
    let input = get_search_input();
    dispatch_input_event_on_input(&input);
}

fn init_searcher(icons: Vec<&'static SimpleIcon>) {
    let icons_candidates_ids = icons
        .iter()
        .map(|icon| {
            let mut candidates: Vec<&str> = vec![icon.title, icon.slug];
            candidates.extend(icon.plain_aliases());
            (candidates, icon.order_alpha)
        })
        .collect::<Vec<(Vec<&str>, usize)>>();

    let icon_candidates_ids_js_array = js_sys::Array::new();
    for (icon_candidates, icon_order_alpha) in &icons_candidates_ids {
        let candidates_array = js_sys::Array::new();
        for icon_title in icon_candidates {
            candidates_array.push(&js_sys::JsString::from(*icon_title).into());
        }

        let icon_title_id_array = js_sys::Array::of2(
            &candidates_array,
            &js_sys::Number::from(*icon_order_alpha as u32).into(),
        );
        icon_candidates_ids_js_array.push(&icon_title_id_array);
    }

    build_searcher(&icon_candidates_ids_js_array);
}

#[inline(always)]
fn new_displayed_icons_from_search_result(
    search_result: &js_sys::Array,
    search_result_length: &u32,
    icons_per_page: usize,
) -> Vec<&'static SimpleIcon> {
    let mut new_displayed_icons: Vec<&'static SimpleIcon> = Vec::new();
    for i in 0..*search_result_length {
        let result_icon_array = js_sys::Array::from(&search_result.get(i));
        let icon_order_alpha = result_icon_array.get(1).as_f64().unwrap();
        new_displayed_icons.push(&ICONS[icon_order_alpha as usize]);
        if new_displayed_icons.len() >= icons_per_page {
            break;
        }
    }

    new_displayed_icons
}

#[inline(always)]
fn extend_new_icons_with_search_result(
    search_result: &js_sys::Array,
    search_result_length: &u32,
    new_icons: &mut Vec<&'static SimpleIcon>,
    icons_per_page: u32,
) {
    if *search_result_length > icons_per_page {
        for i in icons_per_page..*search_result_length {
            let result_icon_array = js_sys::Array::from(&search_result.get(i));
            let icon_order_alpha = result_icon_array.get(1).as_f64().unwrap();
            new_icons.push(&ICONS[icon_order_alpha as usize]);
        }
    }
}

pub fn search_icons_and_returns_first_page(
    search_value: &str,
    icons_per_page: usize,
) -> (Vec<&'static SimpleIcon>, Vec<&'static SimpleIcon>) {
    let search_result = js_sys::Array::from(&search(search_value));
    let search_result_length = search_result.length();

    let new_displayed_icons = new_displayed_icons_from_search_result(
        &search_result,
        &search_result_length,
        icons_per_page,
    );

    let mut new_icons = Vec::with_capacity(search_result_length as usize);
    new_icons.extend(new_displayed_icons.clone());
    extend_new_icons_with_search_result(
        &search_result,
        &search_result_length,
        &mut new_icons,
        icons_per_page as u32,
    );

    (new_icons, new_displayed_icons)
}

pub async fn search_icons(
    search_value: String,
    icons_grid_signal: RwSignal<IconsGrid>,
    icons_per_page: usize,
) {
    let search_result = js_sys::Array::from(&search(&search_value));
    let search_result_length = search_result.length();

    let new_displayed_icons = new_displayed_icons_from_search_result(
        &search_result,
        &search_result_length,
        icons_per_page,
    );
    let new_displayed_icons_for_signal = new_displayed_icons.clone();

    icons_grid_signal.update(move |grid| {
        grid.loaded_icons = new_displayed_icons_for_signal;
    });

    let mut new_icons = Vec::with_capacity(search_result_length as usize);
    new_icons.extend(new_displayed_icons);
    extend_new_icons_with_search_result(
        &search_result,
        &search_result_length,
        &mut new_icons,
        icons_per_page as u32,
    );

    icons_grid_signal.update(move |grid| grid.icons = new_icons);
}

async fn on_search(
    search_input_ref: HtmlInputElement,
    search_signal: RwSignal<String>,
    icons_grid_signal: RwSignal<IconsGrid>,
    order_mode_signal: RwSignal<OrderMode>,
    icons_per_page: usize,
    icons: Vec<&'static SimpleIcon>,
) {
    let value = search_input_ref.value();
    search_signal.update(move |state| {
        Url::params::update(&Url::params::Names::Query, &value);

        if value.is_empty() {
            // Reset grid
            icons_grid_signal.update(|grid| {
                grid.icons.clone_from(&icons);
                grid.loaded_icons =
                    grid.icons.iter().take(icons_per_page).copied().collect();
            });
            // Set new order mode
            set_order_mode(
                &order_mode_signal().favorite,
                &order_mode_signal,
                &icons_grid_signal,
                None,
                true,
                icons,
            );
            set_search_value_on_localstorage(&value);
            *state = value;
            return;
        }

        let search_value_copy = value.clone();
        spawn_local(search_icons(value, icons_grid_signal, icons_per_page));
        set_search_value_on_localstorage(&search_value_copy);
        set_order_mode(
            &OrderModeVariant::SearchMatch,
            &order_mode_signal,
            &icons_grid_signal,
            None,
            false,
            icons,
        );
        *state = search_value_copy;
    });
}

#[component]
pub fn SearchControl() -> impl IntoView {
    let icons_grid = expect_context::<IconsGridSignal>().0;
    let search = expect_context::<SearchValueSignal>().0;
    let order_mode = expect_context::<OrderModeSignal>().0;
    let layout = expect_context::<LayoutSignal>().0;
    let icons = expect_context::<IconsIndexSignal>().0;

    let search_input_ref = NodeRef::new();
    // Focus on load. Fallback for Safari, see:
    // https://caniuse.com/?search=autofocus
    search_input_ref.on_load(|input: HtmlInputElement| {
        _ = input.focus();
        _ = input.blur();
    });

    // The onfocus attribute puts the cursor at the end of the input
    view! {
        <div class="control">
            <label for=Ids::SearchInput.as_str()>{move || tr!("search")}</label>
            <div class="search">
                <input
                    node_ref=search_input_ref
                    id=Ids::SearchInput.as_str()
                    type="search"
                    autocomplete="off"
                    autofocus
                    placeholder=move || tr!("search-by-brand")
                    value=search
                    onfocus="var value = this.value; this.value = null; this.value = value;"
                    on:input=move |_| {
                        spawn_local(
                            on_search(
                                search_input_ref.get().unwrap(),
                                search,
                                icons_grid,
                                order_mode,
                                layout().icons_per_page() as usize,
                                icons.clone(),
                            ),
                        )
                    }
                />

                <Show when=move || !search().is_empty()>
                    <span
                        title=move || tr!("clear-search")
                        on:click=move |_| {
                            search_input_ref.get().unwrap().set_value("");
                            fire_on_search_event();
                        }
                    >

                        "×"
                    </span>
                </Show>
            </div>
        </div>
    }
}
