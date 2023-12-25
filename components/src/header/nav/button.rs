use crate::header::HeaderStateSignal;
use crate::svg::SVGIcon;
use leptos::*;

/// Header menu link
///
/// Each link of the header menu
#[component]
pub fn HeaderMenuLink(
    /// Title of the link
    title: Signal<String>,
    /// URL of the link
    href: &'static str,
    /// SVG path of the icon
    svg_path: &'static str,
) -> impl IntoView {
    let header_state = expect_context::<HeaderStateSignal>().0;
    let title_fn = create_memo(move |_| title());

    view! {
        <li
            on:click=move |_| window().location().set_href(href).unwrap()
            class=move || match header_state().menu_open {
                true => "block",
                false => "hidden lg:block",
            }
        >

            <SVGIcon
                path=svg_path
                role="link"
                aria_label=title_fn
                view_box="0 0 24 24"
                width=""
                height=""
            />
        </li>
    }
}

/// Header menu button
///
/// Each button of the header menu that is not a link
#[component]
pub fn HeaderMenuButton(
    /// Additional classes to add to the button
    class: Signal<String>,
    /// Title of the button
    title: Signal<String>,
    /// SVG path of the icon
    svg_path: &'static str,
) -> impl IntoView {
    view! {
        <li title=title class=class tabindex=0>
            <SVGIcon role="button" path=svg_path width="36" height="36" view_box="0 0 24 24"/>
        </li>
    }
}
