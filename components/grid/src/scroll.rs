use icondata::{RiArrowDownCircleArrowsLine, RiArrowUpCircleArrowsLine};
use leptos::prelude::*;
use leptos_fluent::move_tr;
use leptos_icons::Icon;
use leptos_use::use_window_scroll;
use simple_icons_website_grid_icons_loader::IconsLoaderSignal;
use simple_icons_website_grid_types::IconsGridSignal;

#[component]
pub fn ScrollButton(icon: icondata::Icon) -> impl IntoView {
    view! {
        <button>
            <Icon icon width="24px" height="24px" />
        </button>
    }
}

#[component]
pub fn ScrollToHeaderButton() -> impl IntoView {
    let (_, window_scroll_y) = use_window_scroll();

    view! {
        <Show when=move || { window_scroll_y() >= 200.0 }>
            <ScrollButton
                icon=RiArrowUpCircleArrowsLine
                attr:class="scroll-button scroll-to-header-button"
                attr:title=move_tr!("go-to-header")
                on:click=move |_| {
                    document().query_selector("header").unwrap().unwrap().scroll_into_view()
                }
            />

        </Show>
    }
}

#[component]
pub fn ScrollToFooterButton() -> impl IntoView {
    let icons_loader = expect_context::<IconsLoaderSignal>().0;
    let icons_grid = expect_context::<IconsGridSignal>().0;

    view! {
        <Show when=move || {
            let grid = icons_grid();
            icons_loader().load && grid.loaded_icons.len() < grid.icons.len()
        }>
            <ScrollButton
                icon=RiArrowDownCircleArrowsLine
                attr:class="scroll-button scroll-to-footer-button"
                attr:title=move_tr!("go-to-footer")
                on:click=move |_| {
                    icons_loader.update(|state| state.load = false);
                    let footer = document().query_selector("footer").unwrap().unwrap();
                    footer.scroll_into_view();
                }
            />

        </Show>
    }
}

#[component]
pub fn ScrollButtons() -> impl IntoView {
    view! {
        <ScrollToHeaderButton />
        <ScrollToFooterButton />
    }
}
