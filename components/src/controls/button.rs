use crate::svg::SVGIcon;
use leptos::*;

/// Abstract control button
#[component]
pub fn ControlButton(
    /// Button title
    title: Signal<String>,
    /// Button children
    children: Children,
    /// The control is active
    active: Signal<bool>,
    /// Optional classes
    #[prop(optional)]
    class: &'static str,
) -> impl IntoView {
    view! {
        <button class:selected=active type="button" title=title tabindex=0 class=class>
            {children()}
        </button>
    }
}

/// Control button made from SVG path
#[component]
pub fn ControlButtonSVGPath(
    /// Button title
    title: Signal<String>,
    /// Button icon SVG path
    svg_path: &'static str,
    /// The control is active
    active: Signal<bool>,
    /// Optional classes
    #[prop(optional)]
    class: &'static str,
) -> impl IntoView {
    let title_fn = create_memo(move |_| title());
    view! {
        <ControlButton title=title active=active class=class>
            <SVGIcon
                role="img"
                aria_hidden=true
                aria_label=title_fn
                view_box="0 0 24 24"
                path=svg_path
            />

        </ControlButton>
    }
}

/// Control button made from text
#[component]
pub fn ControlButtonText(
    /// Button title
    title: Signal<String>,
    /// Button text
    text: Signal<String>,
    /// The control is active
    active: Signal<bool>,
) -> impl IntoView {
    view! {
        <ControlButton title=title active=active>
            <span>{text}</span>
        </ControlButton>
    }
}
