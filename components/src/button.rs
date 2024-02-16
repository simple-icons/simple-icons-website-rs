use crate::svg::{IconOrSvg, SVGDef, SVGIcon};
use leptos::*;
use leptos_icons::Icon;

#[component]
pub fn Button(
    title: Signal<String>,
    #[prop(optional, into, default=IconOrSvg::SvgDef(&SVGDef::Null))]
    icon: IconOrSvg,
    #[prop(optional)] class: &'static str,
    #[prop(optional)] id: &'static str,
) -> impl IntoView {
    view! {
        <button title=title class=format!("button {}", class) id=id type="button" tabindex=0>
            {match icon {
                IconOrSvg::Icon(icon) => {
                    view! { <Icon icon width="24px" height="24px"/> }
                }
                ref value => {
                    if icon == IconOrSvg::SvgDef(&SVGDef::Null) {
                        return view! {}.into_view();
                    }
                    view! {
                        <SVGIcon
                            width="24"
                            height="24"
                            aria_hidden=true
                            path=match value {
                                IconOrSvg::SvgPath(svg_path) => svg_path,
                                IconOrSvg::SvgDef(svg_def) => svg_def.d(),
                                _ => unreachable!(),
                            }
                        />
                    }
                }
            }}

            {title}
        </button>
    }.into_view()
}
