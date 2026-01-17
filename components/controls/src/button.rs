use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_use::use_media_query;
use simple_icons_website_svg_icon::{IconOrSvg, SVGIcon};

pub(crate) static XS_ICON_SIZE: &str = "19";

/// Abstract control button
#[component]
pub fn ControlButton(
    children: Children,
    #[prop(into)] active: Signal<bool>,
    #[prop(optional)] class: &'static str,
) -> impl IntoView {
    view! {
        <button class:selected=active type="button" tabindex=0 class=class>
            {children()}
        </button>
    }
}

/// Control button made from SVG path
#[component]
pub fn ControlButtonIcon(
    /// Button title
    title: Signal<String>,
    /// Button icon SVG path
    #[prop(into)]
    icon: IconOrSvg,
    /// The control is active
    #[prop(into)]
    active: Signal<bool>,
    /// Optional classes
    #[prop(optional)]
    class: &'static str,
) -> impl IntoView {
    let is_xs_screen = use_media_query("(max-width: 475px)");

    // TODO: remove `to_string()` and deduplicate implementations when
    // https://github.com/carloskiki/leptos-icons/pull/63 is merged
    let size = Signal::derive(move || {
        if is_xs_screen() {
            XS_ICON_SIZE.to_string()
        } else {
            "24".to_string()
        }
    });
    let size_for_svgicon = Signal::derive(move || {
        if is_xs_screen() {
            XS_ICON_SIZE.to_string()
        } else {
            "24".to_string()
        }
    });

    view! {
        <ControlButton active class attr:title=title>
            {match icon {
                IconOrSvg::Icon(icon) => view! { <Icon icon width=size height=size /> }.into_any(),
                value => {
                    view! {
                        <SVGIcon
                            role="img"
                            aria_hidden=true
                            aria_label=title
                            view_box="0 0 24 24"
                            width=size_for_svgicon
                            height=size_for_svgicon
                            path=match value {
                                IconOrSvg::SvgPath(svg_path) => svg_path,
                                IconOrSvg::SvgDef(svg_def) => svg_def.d(),
                                _ => unreachable!(),
                            }
                        />
                    }
                        .into_any()
                }
            }}

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
        <ControlButton active attr:title=title>
            <span>{text}</span>
        </ControlButton>
    }
}
