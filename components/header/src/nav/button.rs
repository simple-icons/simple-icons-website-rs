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
    /// SVG path of the icon
    #[prop(into)]
    icon: IconOrSvg,
) -> impl IntoView {
    let header_state = expect_context::<HeaderStateSignal>().0;
    let title_fn = Memo::new(move |_| title());

    view! {
        <li
            on:click=move |_| window().location().set_href(href).unwrap()
            title=title_fn
            class=move || match header_state().menu_open {
                true => "block",
                false => "hidden lg:block",
            }
        >

            {match icon {
                IconOrSvg::Icon(icon) => {
                    view! { <Icon icon width="36px" height="36px" /> }.into_any()
                }
                value => {
                    view! {
                        <SVGIcon
                            width="36"
                            height="36"
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

        </li>
    }
}

/// Header menu button
///
/// Each button of the header menu that is not a link
#[component]
pub fn HeaderMenuButton(icon: icondata::Icon) -> impl IntoView {
    view! {
        <li tabindex=0>
            <Icon icon width="36px" height="36px" />
        </li>
    }
}
