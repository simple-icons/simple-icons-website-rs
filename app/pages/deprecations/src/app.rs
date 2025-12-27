use leptos::prelude::*;
use simple_icons_website_grid_constants::DEPRECATED_ICONS;
use simple_icons_website_grid_types::IconsIndexSignal;
use simple_icons_website_page_layout::{Index, SimpleIconsApp};

pub static TITLE: &str = "Simple Icons | Deprecations";

#[component]
pub fn App() -> impl IntoView {
    view! {
        <SimpleIconsApp title=TITLE>
            <DeprecationsIndex />
        </SimpleIconsApp>
    }
}

#[component]
pub fn DeprecationsIndex() -> impl IntoView {
    provide_context::<IconsIndexSignal>(IconsIndexSignal(
        DEPRECATED_ICONS.iter().collect(),
    ));
    view! { <Index /> }
}
