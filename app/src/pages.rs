//! Application pages
use components::controls::download::provide_download_type_context;
use components::controls::layout::provide_layout_context;
use components::controls::order::provide_order_mode_context;
use components::controls::search::provide_search_context;
use components::controls::Controls;
use components::copy::CopyInput;
use components::grid::{provide_icons_grid_contexts, Grid};
use components::svg_defs::SVGDefsDefinition;
use leptos::*;

#[component]
pub fn Index() -> impl IntoView {
    view! {
        <SVGDefsDefinition/>
        <CopyInput/>
        <IndexMain/>
    }
}

#[component]
fn IndexMain() -> impl IntoView {
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
        <main>
            <Controls/>
            <Grid/>
        </main>
    }
}
