use crate::svg_def::SVGDef;
use crate::svg_icon::SVGDefIcon;
use leptos::*;

#[component]
pub fn Button<T>(
    #[prop(optional)] svg_path: &'static SVGDef,
    title: T,
    #[prop(optional)] class: &'static str,
) -> impl IntoView
where
    T: Fn() -> String + 'static + Copy,
{
    view! {
        <button title=title class=format!("button {}", class) type="button" tabindex=0>
            {move || match svg_path {
                SVGDef::Null => view! { "" }.into_view(),
                _ => view! { <SVGDefIcon aria_hidden=true svg_def=svg_path/> },
            }}

            {title}
        </button>
    }
}
