use crate::{
    Brand,
    helpers::{is_svg_file, is_valid_hex_color},
};
use leptos::prelude::*;
use simple_icons_sdk as sdk;
use simple_icons_website_grid_constants::ICONS;

pub(crate) async fn upload_svg_file(
    file: web_sys::File,
    set_color: WriteSignal<String>,
    set_path: WriteSignal<String>,
    brand: RwSignal<Brand>,
) {
    if !is_svg_file(&file) {
        ::leptos::logging::error!(
            "Uploaded file is not an SVG: {}",
            file.name()
        );
        return;
    }

    match wasm_bindgen_futures::JsFuture::from(file.text()).await {
        Ok(text) => {
            let Some(file_content) = text.as_string() else {
                ::leptos::logging::error!(
                    "Uploaded SVG file could not be read as text"
                );
                return;
            };

            // Set color
            if let Some(after) = file_content.split("fill=\"").nth(1)
                && let Some(hex) = after.split('"').next()
            {
                let normalized = sdk::normalize_color(hex);
                if is_valid_hex_color(&normalized) {
                    set_color(normalized.to_string());
                }
            }

            // Set brand
            if file_content.contains("<title>")
                && file_content.contains("</title>")
            {
                let brand_title = file_content
                    .split("<title>")
                    .nth(1)
                    .unwrap()
                    .split("</title>")
                    .next()
                    .unwrap();
                brand.update(|b| b.0 = brand_title.to_string());

                if !file_content.contains("fill=\"") {
                    for icon in ICONS.iter() {
                        if icon.title == brand_title {
                            set_color(icon.hex.to_string());
                            break;
                        }
                    }
                }
            }

            // Set path
            if file_content.contains(" d=\"") {
                let path = file_content
                    .split(" d=\"")
                    .nth(1)
                    .unwrap()
                    .split('"')
                    .next()
                    .unwrap();
                set_path(path.to_string());
            }
        }
        Err(err) => {
            ::leptos::logging::error!(
                "Error reading uploaded SVG file: {:?}",
                err
            )
        }
    }
}
