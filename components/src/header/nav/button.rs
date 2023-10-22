use crate::header::HeaderStateSignal;
use leptos::*;
use wasm_bindgen::JsCast;

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
    let header_state = use_context::<HeaderStateSignal>().unwrap().0;

    view! {
        <li
            class=move || {
                if header_state.get().menu_open {
                    "block".to_string()
                } else {
                    "hidden lg:block".to_string()
                }
            }

            on:click=move |ev| {
                event_target::<web_sys::HtmlElement>(&ev)
                    .first_element_child()
                    .unwrap()
                    .dyn_into::<web_sys::HtmlElement>()
                    .unwrap()
                    .click();
            }
        >

            <a title=title href=href>
                <svg role="link" aria-label=title viewBox="0 0 24 24">
                    <path d=svg_path></path>
                </svg>
            </a>
        </li>
    }
}

/// Header menu button
///
/// Each button of the header menu that is not a link
#[component]
pub fn HeaderMenuButton<C, T>(
    /// Additional classes to add to the button
    additional_classes: C,
    /// Title of the button
    title: T,
    /// SVG path of the icon
    svg_path: &'static str,
) -> impl IntoView
where
    C: Fn() -> String + 'static + Copy,
    T: TitleFn,
{
    view! {
        <li title=title class=additional_classes tabindex=0>
            <svg role="button" aria-label=title viewBox="0 0 24 24">
                <path d=svg_path></path>
            </svg>
        </li>
    }
}
