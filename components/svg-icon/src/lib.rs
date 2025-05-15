use leptos::prelude::*;
use simple_icons_website_svg_defs::SVGDef;

#[component]
pub fn SVGIcon(
    path: impl Into<&'static str>,
    #[prop(optional, into)] aria_label: Option<Signal<String>>,
    #[prop(optional)] class: &'static str,
    #[prop(optional)] fill: &'static str,
    #[prop(optional, into, default = "24".into())] width: MaybeProp<
        &'static str,
    >,
    #[prop(optional, into, default = "24".into())] height: MaybeProp<
        &'static str,
    >,
    #[prop(optional, into, default = "0 0 24 24".into())] view_box: MaybeProp<
        &'static str,
    >,
    #[prop(optional, default = "img")] role: &'static str,
    #[prop(optional, default = true)] aria_hidden: bool,
) -> impl IntoView {
    view! {
        <svg
            class=class
            role=role
            aria-hidden=if aria_hidden { "true" } else { "false" }
            width=width
            height=height
            aria-label=move || match &aria_label {
                Some(aria_label) => aria_label(),
                None => "".to_string(),
            }

            viewBox=view_box
        >

            <path d=path.into() fill=fill></path>
        </svg>
    }
}

/// Icon or SVG path
#[derive(PartialEq)]
pub enum IconOrSvg {
    Icon(icondata::Icon),
    SvgPath(&'static str),
    SvgDef(&'static SVGDef),
}

impl From<icondata::Icon> for IconOrSvg {
    fn from(icon: icondata::Icon) -> Self {
        Self::Icon(icon)
    }
}

impl From<&'static str> for IconOrSvg {
    fn from(svg_path: &'static str) -> Self {
        Self::SvgPath(svg_path)
    }
}

impl From<&'static SVGDef> for IconOrSvg {
    fn from(svg_def: &'static SVGDef) -> Self {
        Self::SvgDef(svg_def)
    }
}

/// Build a SVG string with the 24px24 viewBox from a path and
/// an optional `fill` attribute
pub fn svg_with_path_opt_fill(path: &str, fill: Option<&str>) -> String {
    format!(
        concat!(
            "<svg role=\"img\" viewBox=\"0 0 24 24\"",
            " xmlns=\"http://www.w3.org/2000/svg\"><path{} d=\"{}\"/></svg>",
        ),
        match fill {
            Some(fill) => format!(" fill=\"#{fill}\""),
            None => "".to_string(),
        },
        path,
    )
}

/// Build a SVG string with the 24px24 viewBox from a path, a title and
/// an optional `fill` attribute
pub fn svg_with_title_path_opt_fill(
    title: &str,
    path: &str,
    fill: Option<&str>,
) -> String {
    format!(
        concat!(
            "<svg role=\"img\" viewBox=\"0 0 24 24\"",
            " xmlns=\"http://www.w3.org/2000/svg\">",
            "<title>{}</title><path{} d=\"{}\"/></svg>",
        ),
        // TODO: Escape unicode characters like simple-icons does
        title
            .replace('&', "&amp;")
            .replace('\'', "&apos;")
            .replace('\"', "&quot;")
            .replace('<', "&lt;")
            .replace('>', "&gt;"),
        match fill {
            Some(fill) => format!(" fill=\"#{fill}\""),
            None => "".to_string(),
        },
        path,
    )
}
