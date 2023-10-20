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

fn index_redirections() {
    let query_map = use_query_map()();

    if query_map.get("p").is_some() {
        use_navigate()("/preview", Default::default());
    }

    // Trick to redirect to other pages for servers that don't support SPAs
    if let Some(redirection) = query_map.get("r") {
        use_navigate()(redirection, Default::default());
    }
}

#[component]
pub fn Index() -> impl IntoView {
    index_redirections();

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

#[component]
pub fn Error404() -> impl IntoView {
    view! {
        <menu class="-mt-4 bg-transparent">
            <ColorSchemeControl/>
        </menu>
        <div class="-mt-2 sm:-mt-[52px] flex flex-col items-center justify-center h-full">
            <h1 class="text-8xl font-bold">{"404"}</h1>
            <p class="text-2xl font-bold">{move_tr!("page-not-found")}</p>
            <hr class="w-1/2 my-4 border-t-[var(--divider-color)]"/>
            <p class="text-lg font-bold font-sans pt-2">{move_tr!("maybe-youre-looking-for")}</p>
            <ul class="flex flex-col sm:flex-row py-5">
                <li class="flex p-1">
                    <Button
                        class="mx-auto"
                        title=move_tr!("icons")
                        on:click=move |_| use_navigate()("/", Default::default())
                        svg_path=&SVGDef::Grid
                    />
                </li>
                <li class="flex p-1">
                    <Button
                        class="mx-auto"
                        title=move_tr!("preview-generator")
                        on:click=move |_| use_navigate()("/preview", Default::default())
                        svg_path=&SVGDef::EyeBox
                    />
                </li>
            </ul>
        </div>
    }
}
