use crate::grid::{icons_loader::IconsLoaderSignal, IconsGridSignal};
use crate::svg::{SVGDef, SVGIcon};
use i18n::move_tr;
use leptos::*;
use leptos_use::use_window_scroll;

#[component]
pub fn ScrollButton(
    /// The title of the button
    title: Signal<String>,
    /// The SVG path of the icon
    svg_path: &'static SVGDef,
    /// Additional classes to be added to the button
    class: &'static str,
) -> impl IntoView {
    view! {
        <button class=format!("scroll-button {}", class) title=title>
            <SVGIcon path=svg_path/>
        </button>
    }
}

#[component]
pub fn ScrollToHeaderButton() -> impl IntoView {
    let (_, window_scroll_y) = use_window_scroll();

    view! {
        <Show when=move || { window_scroll_y() >= 200.0 }>
            <ScrollButton
                class="scroll-to-header-button"
                title=move_tr!("go-to-header")
                on:click=move |_| {
                    document().query_selector("header").unwrap().unwrap().scroll_into_view()
                }

                svg_path=&SVGDef::CircleArrowUp
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
            icons_loader().load && icons_grid().loaded_icons.len() < icons_grid().icons.len()
        }>
            <ScrollButton
                class="scroll-to-footer-button"
                title=move_tr!("go-to-footer")
                on:click=move |_| {
                    icons_loader.update(|state| state.load = false);
                    let footer = document().query_selector("footer").unwrap().unwrap();
                    footer.scroll_into_view();
                }

                svg_path=&SVGDef::CircleArrowDown
            />
        </Show>
    }
}

#[component]
pub fn ScrollButtons() -> impl IntoView {
    view! {
        <ScrollToHeaderButton/>
        <ScrollToFooterButton/>
    }
}
