use crate::HeaderStateSignal;
use leptos::prelude::*;
use leptos_icons::Icon;
use simple_icons_website_svg_icon::{IconOrSvg, SVGIcon};

/// Header menu link
///
/// Each link of the header menu
#[component]
pub fn HeaderMenuLink(
    /// Title of the link
    #[prop(into)]
    title: Signal<String>,
    /// URL of the link
    href: &'static str,
    /// Class of the link
    #[prop(default = "")]
    class: &'static str,
    /// SVG path of the icon
    #[prop(into)]
    icon: IconOrSvg,
    /// Width for the icon
    #[prop(default = 25)]
    width: usize,
    /// Height for the icon
    #[prop(default = 25)]
    height: usize,
    /// Optional text for the button
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let header_state = expect_context::<HeaderStateSignal>().0;

    view! {
        <a
            href=href
            title=title
            target="_blank"
            class=move || match header_state().menu_open {
                true => format!("rounded-md flex flex-row {class}"),
                false => format!("rounded-md hidden lg:flex flex-row {class}"),
            }
        >
            {match icon {
                IconOrSvg::Icon(icon) => {
                    view! {
                        <Icon
                            icon
                            width=format!("{width}")
                            height=format!("{height}")
                            attr:class="inline-flex"
                        />
                    }
                        .into_any()
                }
                value => {
                    view! {
                        <SVGIcon
                            width=format!("{width}")
                            height=format!("{height}")
                            path=match value {
                                IconOrSvg::SvgPath(svg_path) => svg_path,
                                IconOrSvg::SvgDef(svg_def) => svg_def.d(),
                                _ => unreachable!(),
                            }
                            class="inline-flex"
                        />
                    }
                        .into_any()
                }
            }}
            {match children {
                Some(children) => {
                    view! { <span class="relative top-[0.5px]">{children()}</span> }.into_any()
                }
                #[allow(clippy::unit_arg, clippy::unused_unit)]
                None => view! {  }.into_any(),
            }}
        </a>
    }
}

/// Header menu button
///
/// Each button of the header menu that is not a link
#[component]
pub fn HeaderMenuButton(
    icon: icondata::Icon,
    /// Optional text for the button
    #[prop(optional)]
    children: Option<Children>,
    /// Width for the icon
    #[prop(default = 25)]
    width: usize,
    /// Height for the icon
    #[prop(default = 25)]
    height: usize,
) -> impl IntoView {
    view! {
        <li tabindex=0 class="inline-flex justify-center rounded-lg">
            <Icon
                icon
                width=format!("{width}")
                height=format!("{height}")
                attr:class="inline-block"
            />
            {match children {
                Some(children) => {
                    view! { <span class="relative top-[1.5px]">{children()}</span> }.into_any()
                }
                #[allow(clippy::unit_arg, clippy::unused_unit)]
                None => view! {  }.into_any(),
            }}
        </li>
    }
}
