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
    let title_fn = Memo::new(move |_| title());
    let is_xs_screen = use_media_query("(max-width: 475px)");

    // TODO: leptos-icons should be able to accept `'static str` and `String`
    //       as `width` and `height` properties using `Cow` or something similar
    let size =
        Memo::new(move |_| if is_xs_screen() { XS_ICON_SIZE } else { "24" });

    view! {
        <ControlButton active class attr:title=title>
            {match icon {
                IconOrSvg::Icon(icon) => {
                    view! {
                        <Icon
                            icon
                            width=Signal::derive(move || size().to_string())
                            height=Signal::derive(move || size().to_string())
                        />
                    }
                        .into_any()
                }
                value => {
                    view! {
                        <SVGIcon
                            role="img"
                            aria_hidden=true
                            aria_label=title_fn
                            view_box="0 0 24 24"
                            width=size
                            height=size
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
