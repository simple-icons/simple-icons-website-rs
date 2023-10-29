use crate::header::HeaderStateSignal;
use crate::svg::SVGIcon;
use leptos::*;

pub trait TitleFn = Fn() -> String + 'static + Copy;

/// Header menu link
///
/// Each link of the header menu
#[component]
pub fn HeaderMenuLink<T>(
    /// Title of the link
    title: T,
    /// URL of the link
    href: &'static str,
    /// SVG path of the icon
    svg_path: &'static str,
) -> impl IntoView
where
    T: TitleFn,
{
    let header_state = expect_context::<HeaderStateSignal>().0;
    let title_fn = create_memo(move |_| title());

    view! {
        <li
            on:click=move |_| window().location().set_href(href).unwrap()
            class=move || match header_state.get().menu_open {
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
pub fn HeaderMenuButton<C, T>(
    /// Additional classes to add to the button
    class: C,
    /// Title of the button
    title: T,
    /// SVG path of the icon
    svg_path: &'static str,
) -> impl IntoView
where
    C: Fn() -> &'static str + 'static,
    T: TitleFn,
{
    view! {
        <li title=title class=class tabindex=0>
            <SVGIcon role="button" path=svg_path width="36" height="36" view_box="0 0 24 24"/>
        </li>
    }
}
