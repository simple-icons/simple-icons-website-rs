pub mod color;
mod deprecated;
pub mod lint;

pub use deprecated::{IconDeprecation, fetch_deprecated_simple_icons};
use simple_icons_sdk::{
    SimpleIconDataAliases, SimpleIconDataLicense, get_simple_icons_data,
    title_to_slug,
};
use std::fs;
use std::path::Path;

/// Struct for a Simple Icon
#[derive(Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct SimpleIcon {
    pub slug: String,
    pub title: String,
    pub hex: String,
    pub source: String,
    pub guidelines: Option<String>,
    pub license: Option<SimpleIconDataLicense>,
    pub aliases: Option<SimpleIconDataAliases>,
}

/// Get simple icons
pub fn get_simple_icons() -> Vec<SimpleIcon> {
    let simple_icons_data = get_simple_icons_data();
    let mut simple_icons: Vec<SimpleIcon> =
        Vec::with_capacity(simple_icons_data.len());

    for icon_data in simple_icons_data {
        let icon = SimpleIcon {
            slug: match icon_data.slug {
                Some(slug) => slug,
                None => title_to_slug(&icon_data.title),
            },
            title: icon_data.title,
            hex: icon_data.hex,
            source: icon_data.source,
            guidelines: icon_data.guidelines,
            license: icon_data.license,
            aliases: icon_data.aliases,
        };
        simple_icons.push(icon);
    }

    simple_icons
}

/// Get the SVG path for a simple icon by its slug
pub fn get_simple_icon_svg_path(slug: &str) -> String {
    let icon_file_path = Path::new("node_modules")
        .join("simple-icons")
        .join("icons")
        .join(format!("{slug}.svg"));
    let icon_file_content = fs::read_to_string(&icon_file_path)
        .unwrap_or_else(|err| panic!("Error reading icon file: {err}"));
    let icon_path = icon_file_content
        .split_once("d=\"")
        .unwrap()
        .1
        .split_once('"')
        .unwrap()
        .0;
    icon_path.to_string()
}
