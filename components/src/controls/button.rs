use crate::svg::SVGIcon;
use leptos::*;
use leptos_use::use_media_query;

pub(crate) static XS_ICON_SIZE: &str = "19";

/// Abstract control button
#[component]
pub fn ControlButton(
    /// Button title
    title: Signal<String>,
    /// Button children
    children: Children,
    /// The control is active
    #[prop(into)]
    active: MaybeSignal<bool>,
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
    #[prop(into)]
    active: MaybeSignal<bool>,
    /// Optional classes
    #[prop(optional)]
    class: &'static str,
) -> impl IntoView {
    let title_fn = create_memo(move |_| title());
    let is_xs_screen = use_media_query("(max-width: 475px)");
    let size =
        create_memo(move |_| if is_xs_screen() { XS_ICON_SIZE } else { "24" });

    view! {
        <ControlButton title active class>
            <SVGIcon
                role="img"
                aria_hidden=true
                aria_label=title_fn
                view_box="0 0 24 24"
                width=size
                height=size
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
        <ControlButton title active=active>
            <span>{text}</span>
        </ControlButton>
    }
}
