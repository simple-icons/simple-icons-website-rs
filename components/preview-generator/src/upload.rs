use crate::{Brand, helpers::is_valid_hex_color};
use leptos::prelude::*;
use simple_icons_sdk as sdk;
use simple_icons_website_grid_constants::ICONS;

fn is_svg_file(file: &web_sys::File) -> bool {
    file.name().to_lowercase().ends_with(".svg")
        || file.type_() == "image/svg+xml"
}

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
            if file_content.contains("fill=\"") {
                let hex = sdk::normalize_color(
                    file_content
                        .split("fill=\"")
                        .nth(1)
                        .unwrap()
                        .split('"')
                        .next()
                        .unwrap(),
                );
                if is_valid_hex_color(&hex) {
                    set_color(hex.to_string());
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
