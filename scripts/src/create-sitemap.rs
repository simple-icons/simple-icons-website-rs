use config::CONFIG;
use std::fs;
use std::io::Write;
use std::path;

fn main() {
    let sitemap_asset_path = path::Path::new("app/assets/sitemap.xml");

    if sitemap_asset_path.exists() {
        fs::remove_file(sitemap_asset_path).unwrap();
    }
    let mut sitemap_file = fs::File::create(sitemap_asset_path).unwrap();

    sitemap_file
        .write_all(
            format!(
                concat!(
                    "<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\"",
                    " xmlns:xhtml=\"http://www.w3.org/1999/xhtml\">\n",
                    "  <url>\n",
                    "    <loc>http{}://{}/</loc>\n",
                    "    <lastmod>{:?}</lastmod>\n",
                    "    <changefreq>weekly</changefreq>\n",
                    "  </url>\n",
                    "</urlset>",
                ),
                if CONFIG.domain.contains(':') {  // localhost
                    ""
                } else {
                    "s"
                },
                CONFIG.domain,
                chrono::offset::Utc::now(),
            )
            .as_bytes(),
        )
        .expect("Unable to write sitemap.xml asset file")
}
