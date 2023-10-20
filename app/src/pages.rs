//! Application pages
use components::button::Button;
use components::controls::color_scheme::ColorSchemeControl;
use components::controls::download::provide_download_type_context;
use components::controls::layout::provide_layout_context;
use components::controls::order::provide_order_mode_context;
use components::controls::search::provide_search_context;
use components::controls::Controls;
use components::grid::{provide_icons_grid_contexts, Grid};
use components::preview_generator::PreviewGenerator;
use components::svg_def::SVGDef;
use i18n::move_tr;
use leptos::*;
use leptos_router::{use_navigate, use_query_map};

#[component]
pub fn Index() -> impl IntoView {
    // Trick to redirect to other pages for servers that don't support SPAs
    if use_query_map()().get("p").is_some() {
        use_navigate()("/preview", Default::default());
    }

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
        <menu class="-mt-4 lg:bg-transparent flex flex-row lg:flex-col">
            <ColorSchemeControl/>
            <div class=concat!(
                "flex items-center relative left-3 lg:-left-0.5",
                " max-w-auto lg:max-w-[114px]",
            )>
                <Button
                    class="mx-auto mt-[29px] lg:mt-5 max-h-[40px]"
                    title=move_tr!("icons")
                    on:click=move |_| use_navigate()("/", Default::default())
                    svg_path=&SVGDef::Grid
                />

            </div>
        </menu>
        <PreviewGenerator/>
    }
}
