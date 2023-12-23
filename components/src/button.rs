use crate::svg::{SVGDef, SVGIcon};
use leptos::*;

#[component]
pub fn Button<T>(
    title: T,
    #[prop(optional)] svg_path: &'static SVGDef,
    #[prop(optional)] class: &'static str,
    #[prop(optional)] id: &'static str,
) -> impl IntoView
where
    T: Fn() -> String + 'static + Copy,
{
    view! {
        <button title=title class=format!("button {}", class) id=id type="button" tabindex=0>
            <Show when=move || svg_path != &SVGDef::Null>
                <SVGIcon aria_hidden=true path=svg_path/>
            </Show>
            {title}
        </button>
    }
}
