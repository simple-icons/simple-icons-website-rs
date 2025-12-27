use icondata::{BsGrid3x2GapFill, IoWarningSharp, VsPreview};
use leptos::prelude::*;
use leptos_fluent::move_tr;
use leptos_icons::Icon;
use simple_icons_website_controls::color_scheme::ColorSchemeControl;
use simple_icons_website_page_layout::SimpleIconsApp;

pub const TITLE: &str = "Simple Icons | 404 Not Found";

#[component]
pub fn App() -> impl IntoView {
    view! {
        <SimpleIconsApp title=TITLE>
            <Error404 />
        </SimpleIconsApp>
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
                    <a class="button mx-auto" href="/">
                        <Icon icon=BsGrid3x2GapFill width="24px" height="24px" />
                        {move_tr!("icons")}
                    </a>
                </li>
                <li class="flex p-1">
                    <a class="button mx-auto" href="/deprecations/">
                        <Icon icon=IoWarningSharp width="24px" height="24px" />
                        {move_tr!("deprecations")}
                    </a>
                </li>
                <li class="flex p-1">
                    <a class="button mx-auto" href="/preview/">
                        <Icon icon=VsPreview width="24px" height="24px" />
                        {move_tr!("preview-generator")}
                    </a>
                </li>
            </ul>
        </div>
    }
}
