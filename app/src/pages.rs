//! Application pages
use components::controls::color_scheme::ColorSchemeControl;
use components::controls::download::provide_download_type_context;
use components::controls::layout::provide_layout_context;
use components::controls::order::provide_order_mode_context;
use components::controls::search::provide_search_context;
use components::controls::Controls;
use components::grid::{provide_icons_grid_contexts, Grid};
use components::preview::PreviewGenerator;
use leptos::*;

#[component]
pub fn Index() -> impl IntoView {
    let initial_search_value = provide_search_context();
    let initial_order_mode = provide_order_mode_context(&initial_search_value);
    provide_download_type_context();
    let initial_layout = provide_layout_context();
    provide_icons_grid_contexts(
        &initial_search_value,
        &initial_order_mode,
        &initial_layout,
    );
    view! {
        <Controls/>
        <Grid/>
    }
}

#[component]
pub fn Preview() -> impl IntoView {
    view! {
        <menu class="-mt-4 lg:bg-transparent">
            <ColorSchemeControl/>
        </menu>
        <PreviewGenerator/>
    }
}
