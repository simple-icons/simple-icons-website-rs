pub mod fuzzy;

use crate::controls::layout::LayoutSignal;
use crate::controls::order::{
    set_order_mode, OrderMode, OrderModeSignal, OrderModeVariant,
};
use crate::event::dispatch_input_event_on_input;
use crate::grid::{IconsGrid, IconsGridSignal, ICONS};
use crate::storage::LocalStorage;
use crate::Ids;
use crate::Url;
use fuzzy::{build_searcher, search};
use i18n::move_tr;
use js_sys::JsString;
use leptos::{document, html::Input, window, *};
use types::SimpleIcon;
use wasm_bindgen::JsCast;
use web_sys;

#[derive(Copy, Clone)]
pub struct SearchValueSignal(pub RwSignal<String>);

pub fn provide_search_context() -> String {
    let initial_search_value = initial_search_value();
    provide_context(SearchValueSignal(create_rw_signal(
        initial_search_value.clone(),
    )));

    initial_search_value
}

fn initial_search_value() -> String {
    let search_value = match Url::params::get(&Url::params::Names::Query) {
        Some(value) => {
            set_search_value_on_localstorage(value.as_str());
            value
        }
        None => match initial_search_value_from_localstorage() {
            Some(value) => {
                Url::params::update(&Url::params::Names::Query, value.as_str());
                set_search_value_on_localstorage(value.as_str());
                value
            }
            None => String::new(),
        },
    };

    init_searcher();
    search_value
}

fn initial_search_value_from_localstorage() -> Option<String> {
    let local_storage = window().local_storage().unwrap().unwrap();

    match local_storage.get_item(LocalStorage::Keys::SearchValue.as_str()) {
        Ok(Some(search_value)) => match search_value.is_empty() {
            true => None,
            false => Some(search_value),
        },
        _ => None,
    }
}

pub fn set_search_value_on_localstorage(search_value: &str) {
    let local_storage = window().local_storage().unwrap().unwrap();
    local_storage
        .set_item(LocalStorage::Keys::SearchValue.as_str(), search_value)
        .unwrap();
}

pub fn fire_on_search_event() {
    let input = document()
        .get_element_by_id(Ids::SearchInput.as_str())
        .unwrap()
        .dyn_into::<web_sys::HtmlInputElement>()
        .unwrap();
    dispatch_input_event_on_input(&input);
}

fn init_searcher() {
    let icons_candidates_ids = ICONS
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
            candidates_array.push(&JsString::from(*icon_title).into());
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
    search_input_ref: NodeRef<Input>,
    search_signal: RwSignal<String>,
    icons_grid_signal: RwSignal<IconsGrid>,
    order_mode_signal: RwSignal<OrderMode>,
    icons_per_page: usize,
) {
    let value = search_input_ref.get().unwrap().value();
    search_signal.update(move |state| {
        Url::params::update(&Url::params::Names::Query, &value);

        if value.is_empty() {
            // Reset grid
            icons_grid_signal.update(|grid| {
                grid.icons = ICONS.iter().collect();
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
        );
        *state = search_value_copy;
    });
}

#[component]
pub fn SearchControl() -> impl IntoView {
    let icons_grid = use_context::<IconsGridSignal>().unwrap().0;
    let search = use_context::<SearchValueSignal>().unwrap().0;
    let order_mode = use_context::<OrderModeSignal>().unwrap().0;
    let layout = use_context::<LayoutSignal>().unwrap().0;

    let search_input_ref = create_node_ref::<Input>();
    // Focus on load. Fallback for Safari, see:
    // https://caniuse.com/?search=autofocus
    search_input_ref.on_load(|input| {
        _ = input.focus();
    });

    /* The onfocus attribute puts the cursor at the end of the input */
    view! {
        <div class="control">
            <label for=Ids::SearchInput.as_str()>{move_tr!("search")}</label>
            <div class="search">
                <input
                    node_ref=search_input_ref
                    id=Ids::SearchInput.as_str()
                    type="search"
                    autocomplete="off"
                    autofocus
                    placeholder=move_tr!("search-by-brand")
                    value=search
                    onfocus="var value = this.value; this.value = null; this.value = value;"
                    on:input=move |_| {
                        spawn_local(
                            on_search(
                                search_input_ref,
                                search,
                                icons_grid,
                                order_mode,
                                layout().icons_per_page() as usize,
                            ),
                        )
                    }
                />

                <Show when=move || !search().is_empty()>
                    <span
                        title=move_tr!("clear-search")
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
