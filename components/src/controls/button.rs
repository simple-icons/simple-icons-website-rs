use crate::svg::SVGIcon;
use leptos::*;

pub trait ActiveFn = Fn() -> bool + 'static;
pub trait TextFn = Fn() -> String + 'static + Copy;

/// Abstract control button
#[component]
pub fn ControlButton<A, Ti>(
    /// Button title
    title: Ti,
    /// Button children
    children: Children,
    /// The control is active
    active: A,
    /// Optional classes
    #[prop(optional)]
    class: &'static str,
) -> impl IntoView
where
    Ti: TextFn,
    A: ActiveFn,
{
    view! {
        <button class:selected=active type="button" title=title tabindex=0 class=class>
            {children()}
        </button>
    }
}

/// Control button made from SVG path
#[component]
pub fn ControlButtonSVGPath<A, T>(
    /// Button title
    title: T,
    /// Button icon SVG path
    svg_path: &'static str,
    /// The control is active
    active: A,
    /// Optional classes
    #[prop(optional)]
    class: &'static str,
) -> impl IntoView
where
    A: ActiveFn,
    T: TextFn,
{
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
pub fn ControlButtonText<A, Ti, Tx>(
    /// Button title
    title: Ti,
    /// Button text
    text: Tx,
    /// The control is active
    active: A,
) -> impl IntoView
where
    Ti: TextFn,
    Tx: TextFn,
    A: ActiveFn,
{
    view! {
        <ControlButton title=title active=active>
            <span>{text}</span>
        </ControlButton>
    }
}
