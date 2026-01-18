use leptos::prelude::*;
use simple_icons_website_controls::color_scheme::ColorSchemeControl;
use simple_icons_website_controls::search::init_searcher;
use simple_icons_website_grid_constants::ICONS;
use simple_icons_website_page_layout::SimpleIconsApp;
use simple_icons_website_preview_generator::{
    PreviewGenerator, add_preview_generator_scripts,
};

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
        </menu>
        <div class="page-padding-x flex justify-center">
            <PreviewGenerator />
        </div>
    }
}
