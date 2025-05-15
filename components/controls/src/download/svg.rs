use crate::download::download;

/// Download a SVG icon by its slug
pub fn download_svg(slug: &str) {
    download(&format!("{slug}.svg"), &format!("/icons/{slug}.svg"));
}
