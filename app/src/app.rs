use leptos::prelude::*;
use simple_icons_website_grid_constants::ICONS;
use simple_icons_website_grid_types::IconsIndexSignal;
use simple_icons_website_page_layout::{Index, SimpleIconsApp};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <SimpleIconsApp>
            <AllIconsIndex />
        </SimpleIconsApp>
    }
}

#[component]
pub fn AllIconsIndex() -> impl IntoView {
    provide_context::<IconsIndexSignal>(IconsIndexSignal(
        ICONS.iter().collect(),
    ));
    view! { <Index /> }
}
