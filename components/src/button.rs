use crate::svg::{SVGDef, SVGDefIcon};
use leptos::*;

#[component]
pub fn Button<'a, T>(
    title: T,
    #[prop(optional)] svg_path: &'static SVGDef,
    #[prop(optional)] class: &'a str,
    #[prop(optional)] id: &'static str,
) -> impl IntoView
where
    T: Fn() -> String + 'static + Copy,
{
    view! {
        <button title=title class=format!("button {}", class) id=id type="button" tabindex=0>
            {move || match svg_path {
                SVGDef::Null => view! { "" }.into_view(),
                _ => view! { <SVGDefIcon aria_hidden=true svg_def=svg_path/> },
            }}

            {title}
        </button>
    }
}
