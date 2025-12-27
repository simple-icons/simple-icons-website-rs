use leptos::prelude::*;
use simple_icons_website_controls::color_scheme::ColorSchemeControl;
use leptos_fluent::move_tr;
use leptos_icons::Icon;
use leptos_router::components::A;
use simple_icons_website_grid_constants::ICONS;
use simple_icons_website_preview_generator::{PreviewGenerator, add_preview_generator_scripts};
use icondata::{BsGrid3x2GapFill, IoWarningSharp};
use simple_icons_website_controls::search::init_searcher;
use simple_icons_website_page_base::SimpleIconsApp;

pub const TITLE: &str = "Simple Icons | Preview Generator";

#[component]
pub fn App() -> impl IntoView {
    view! {
        <SimpleIconsApp title=TITLE>
            <Preview />
        </SimpleIconsApp>
    }
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
