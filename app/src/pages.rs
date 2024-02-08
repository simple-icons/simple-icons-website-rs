//! Application pages
use components::button::Button;
use components::controls::color_scheme::ColorSchemeControl;
use components::controls::download::provide_download_type_context;
use components::controls::layout::provide_layout_context;
use components::controls::order::provide_order_mode_context;
use components::controls::search::provide_search_context;
use components::controls::Controls;
use components::grid::{
    provide_icons_grid_contexts, Grid, IconsIndexSignal, DEPRECATED_ICONS,
    ICONS,
};
use components::preview_generator::PreviewGenerator;
use components::svg::SVGDef;
use leptos::*;
use leptos_fluent::i18n;
use leptos_router::{use_navigate, use_query_map, NavigateOptions};

fn index_redirections() {
    let query_map = use_query_map()();

    // Trick to redirect to other pages for servers that don't support SPAs
    if let Some(redirection) = query_map.get("r") {
        let navigate_opts = NavigateOptions {
            replace: true,
            ..Default::default()
        };

        let mut new_parms = query_map.clone();
        new_parms.remove("r");

        let url = format!("{}{}", redirection, new_parms.to_query_string());
        #[cfg(debug_assertions)]
        ::log::debug!("Redirecting to {}", url);
        use_navigate()(&url, navigate_opts);
    }
}

#[component]
pub fn Index() -> impl IntoView {
    index_redirections();

    let icons = expect_context::<IconsIndexSignal>().0;
    let initial_search_value = provide_search_context(icons.clone());
    let initial_order_mode = provide_order_mode_context(&initial_search_value);
    provide_download_type_context();
    let initial_layout = provide_layout_context();

    provide_icons_grid_contexts(
        &initial_search_value,
        &initial_order_mode,
        &initial_layout,
        icons,
    );

    view! {
        <Controls/>
        <Grid/>
    }
}

#[component]
pub fn AllIconsIndex() -> impl IntoView {
    provide_context::<IconsIndexSignal>(IconsIndexSignal(
        ICONS.iter().collect(),
    ));

    view! { <Index/> }
}

#[component]
pub fn DeprecationsIndex() -> impl IntoView {
    provide_context::<IconsIndexSignal>(IconsIndexSignal(
        DEPRECATED_ICONS.iter().collect(),
    ));
    view! { <Index/> }
}

#[component]
pub fn Preview() -> impl IntoView {
    let i18n = i18n();
    view! {
        <menu class="page-padding-x -mt-4 lg:bg-transparent flex flex-row lg:flex-col">
            <ColorSchemeControl/>
            <div class=concat!(
                "flex items-center relative left-3 lg:-left-0.5",
                " max-w-auto lg:max-w-[114px]",
                " mt-0 md:mt-[29px] lg:mt-5",
            )>
                <Button
                    class="mx-auto max-h-[40px]"
                    title=Signal::derive(move || i18n.tr("icons"))
                    on:click=move |_| use_navigate()("/", Default::default())
                    svg_path=&SVGDef::Grid
                />

            </div>
        </menu>
        <div class="page-padding-x flex justify-center">
            <PreviewGenerator/>
        </div>
    }
}

#[component]
pub fn Error404() -> impl IntoView {
    view! {
        <menu class="page-padding-x -mt-4 bg-transparent">
            <ColorSchemeControl/>
        </menu>
        <div class="page-padding-x -mt-2 sm:-mt-[52px] flex flex-col items-center justify-center h-full">
            <h1 class="text-8xl font-bold">{"404"}</h1>
            <p class="text-2xl font-bold">{move || i18n().tr("page-not-found")}</p>
            <hr class="w-1/2 my-4 border-t-[var(--divider-color)]"/>
            <p class="text-lg font-bold font-sans pt-2">
                {move || i18n().tr("maybe-youre-looking-for")}
            </p>
            <ul class="flex flex-col sm:flex-row py-5">

                <li class="flex p-1">
                    <Button
                        class="mx-auto"
                        title=Signal::derive(move || i18n().tr("icons"))
                        on:click=move |_| use_navigate()("/", Default::default())
                        svg_path=&SVGDef::Grid
                    />
                </li>

                <li class="flex p-1">
                    <Button
                        class="mx-auto"
                        title=Signal::derive(move || i18n().tr("deprecations"))
                        on:click=move |_| use_navigate()("/deprecations", Default::default())
                        svg_path=&SVGDef::Warning
                    />
                </li>
                <li class="flex p-1">
                    <Button
                        class="mx-auto"
                        title=Signal::derive(move || i18n().tr("preview-generator"))
                        on:click=move |_| use_navigate()("/preview", Default::default())
                        svg_path=&SVGDef::EyeBox
                    />
                </li>
            </ul>
        </div>
    }
}
