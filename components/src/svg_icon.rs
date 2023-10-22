use crate::svg_def::SVGDef;
use leptos::*;

#[component]
pub fn SVGIcon(
    path: &'static str,
    #[prop(optional)] class: &'static str,
    #[prop(optional)] fill: &'static str,
    #[prop(optional, default = "24")] width: &'static str,
    #[prop(optional, default = "24")] height: &'static str,
    #[prop(optional, default = "img")] role: &'static str,
    #[prop(optional, default = true)] aria_hidden: bool,
    #[prop(optional)] aria_label: &'static str,
) -> impl IntoView {
    view! {
        <svg
            class=class
            role=role
            viewBox=format!("0 0 {} {}", width, height)
            width=width
            height=height
            aria-hidden=aria_hidden
            aria-label=aria_label
        >
            <path d=path fill=fill></path>
        </svg>
    }
}

#[component]
pub fn SVGDefIcon(
    svg_def: &'static SVGDef,
    #[prop(optional)] class: &'static str,
    #[prop(optional)] fill: &'static str,
    #[prop(optional, default = "24")] width: &'static str,
    #[prop(optional, default = "24")] height: &'static str,
    #[prop(optional, default = "img")] role: &'static str,
    #[prop(optional, default = true)] aria_hidden: bool,
    #[prop(optional)] aria_label: &'static str,
) -> impl IntoView {
    view! {
        <svg
            class=class
            fill=fill
            role=role
            viewBox=format!("0 0 {} {}", width, height)
            width=width
            height=height
            aria-hidden=aria_hidden
            aria-label=aria_label
        >
            <use_ href=format!("#{}", svg_def.id())></use_>
        </svg>
    }
}

/// Build a SVG string with the 24px24 viewBox and an optional `fill` attribute
pub fn svg_with_path_opt_fill(path: &str, fill: Option<&str>) -> String {
    format!(
        "<svg role=\"img\" viewBox=\"0 0 24 24\" xmlns=\"http://www.w3.org/2000/svg\"><path d=\"{}\"{}/></svg>",
        path,
        match fill {
            Some(fill) => format!(" fill=\"#{}\"", fill),
            None => "".to_string(),
        }
    )
}
