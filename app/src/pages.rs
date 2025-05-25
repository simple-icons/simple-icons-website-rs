//! Application pages
use icondata::{BsGrid3x2GapFill, IoWarningSharp, VsPreview};
use leptos::prelude::*;
use leptos_fluent::move_tr;
use leptos_icons::Icon;
use leptos_router::{
    components::A,
    hooks::{use_navigate, use_query_map},
    NavigateOptions,
};
use simple_icons_website_controls::{
    color_scheme::ColorSchemeControl,
    download::provide_download_type_context,
    layout::provide_layout_context,
    order::provide_order_mode_context,
    search::{init_searcher, provide_search_context},
    Controls,
};
use simple_icons_website_grid::{provide_icons_grid_contexts, Grid};
use simple_icons_website_grid_constants::{DEPRECATED_ICONS, ICONS};
use simple_icons_website_grid_types::IconsIndexSignal;
use simple_icons_website_preview_generator::{
    add_preview_generator_scripts, PreviewGenerator,
};

fn index_redirections() -> bool {
    let query_map = use_query_map().get_untracked();

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
        #[allow(leptos_print_stdout)]
        {
            ::leptos::logging::log!("Redirecting to {}", url);
        }
        use_navigate()(&url, navigate_opts);

        return true;
    }

    false
}

#[component]
pub fn Index() -> AnyView {
    let redirected = index_redirections();

    if redirected {
        #[allow(clippy::unit_arg, clippy::unused_unit)]
        return view!().into_any();
    }

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
        <Controls />
        <Grid />
    }
    .into_any()
}

#[component]
pub fn AllIconsIndex() -> impl IntoView {
    provide_context::<IconsIndexSignal>(IconsIndexSignal(
        ICONS.iter().collect(),
    ));
    view! { <Index /> }
}

#[component]
pub fn DeprecationsIndex() -> impl IntoView {
    provide_context::<IconsIndexSignal>(IconsIndexSignal(
        DEPRECATED_ICONS.iter().collect(),
    ));
    view! { <Index /> }
}

#[component]
pub fn Preview() -> impl IntoView {
    init_searcher(ICONS.iter().collect());
    add_preview_generator_scripts();

    view! {
        <menu class="page-padding-x -mt-4 lg:bg-transparent flex flex-row lg:flex-col">
            <ColorSchemeControl />
            <div class=concat!(
                "flex lg:flex-col items-center lg:space-y-3",
                " relative left-4 lg:left-0 mt-2 sm:mt-7",
                " lg:max-w-[114px]",
            )>
                <A attr:class="button mx-auto max-h-[40px] ml-0 lg:ml-1" href="/">
                    <Icon icon=BsGrid3x2GapFill width="24px" height="24px" />
                    {move_tr!("icons")}
                </A>
                <A attr:class="button mx-auto max-h-[40px] ml-2 lg:-ml-1.5" href="/deprecations">
                    <Icon icon=IoWarningSharp width="24px" height="24px" />
                    {move_tr!("deprecations")}
                </A>
            </div>
        </menu>
        <div class="page-padding-x flex justify-center">
            <PreviewGenerator />
        </div>
    }
}

#[component]
pub fn Error404() -> impl IntoView {
    view! {
        <menu class="page-padding-x -mt-4 bg-transparent">
            <ColorSchemeControl />
        </menu>
        <div class="page-padding-x -mt-2 sm:-mt-[52px] flex flex-col items-center justify-center h-full">
            <h1 class="text-8xl font-bold">{"404"}</h1>
            <p class="text-2xl font-bold">{move_tr!("page-not-found")}</p>
            <hr class="w-1/2 my-4 border-t-[var(--divider-color)]" />
            <p class="text-lg font-bold font-sans pt-2">{move_tr!("maybe-youre-looking-for")}</p>
            <ul class="flex flex-col sm:flex-row py-5">
                <li class="flex p-1">
                    <A attr:class="button mx-auto" href="/">
                        <Icon icon=BsGrid3x2GapFill width="24px" height="24px" />
                        {move_tr!("icons")}
                    </A>
                </li>
                <li class="flex p-1">
                    <A attr:class="button mx-auto" href="/deprecations">
                        <Icon icon=IoWarningSharp width="24px" height="24px" />
                        {move_tr!("deprecations")}
                    </A>
                </li>
                <li class="flex p-1">
                    <A attr:class="button mx-auto" href="/preview">
                        <Icon icon=VsPreview width="24px" height="24px" />
                        {move_tr!("preview-generator")}
                    </A>
                </li>
            </ul>
        </div>
    }
}
