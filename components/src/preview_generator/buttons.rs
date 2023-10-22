use crate::button::Button;
use crate::controls::download::download;
use crate::copy::copy_canvas_container_as_image;
use crate::keyboard::load_keyboard_shortcut_ctrl_and_key_on_click_id;
use crate::preview_generator::{
    canvas::get_canvas_container, helpers::is_valid_hex_color,
};
use crate::svg_def::SVGDef;
use crate::svg_icon::svg_with_path_opt_fill;
use crate::Ids;
use i18n::{move_tr, tr};
use leptos::*;
use simple_icons::sdk;
use std::collections::HashMap;
use std::time::Duration;
use wasm_bindgen::JsCast;

#[component]
pub fn PreviewButtons(
    brand: ReadSignal<String>,
    path: ReadSignal<String>,
    set_brand: WriteSignal<String>,
    set_color: WriteSignal<String>,
    set_path: WriteSignal<String>,
) -> impl IntoView {
    view! {
        <div class="preview-buttons">
            <div>
                <PreviewUploadSVGButton set_brand=set_brand set_color=set_color set_path=set_path/>
            </div>
            <div class="float-right">
                <PreviewCopyButton/>
                <PreviewSaveButton brand=brand/>
                <PreviewDownloadSVGButton brand=brand path=path/>
            </div>
        </div>
    }
}

#[component]
fn PreviewUploadSVGButton(
    set_brand: WriteSignal<String>,
    set_color: WriteSignal<String>,
    set_path: WriteSignal<String>,
) -> impl IntoView {
    let input_id = Ids::PreviewUploadSVGButton.as_str();
    load_keyboard_shortcut_ctrl_and_key_on_click_id(input_id, "ArrowUp");

    async fn on_upload_svg_file(
        file: web_sys::File,
        set_brand: WriteSignal<String>,
        set_color: WriteSignal<String>,
        set_path: WriteSignal<String>,
    ) {
        match wasm_bindgen_futures::JsFuture::from(file.text()).await {
            Ok(text) => {
                let value = text.as_string().unwrap();

                // Set color
                if value.contains("fill=\"") {
                    let hex = sdk::normalize_color(
                        value
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
                if value.contains("<title>") && value.contains("</title>") {
                    let brand = value
                        .split("<title>")
                        .nth(1)
                        .unwrap()
                        .split("</title>")
                        .next()
                        .unwrap();
                    set_brand(brand.to_string());
                }

                // Set path
                if value.contains(" d=\"") {
                    let path = value
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
                ::log::error!("Error reading uploaded SVG file: {:?}", err)
            }
        }
    }

    view! {
        <form class="inline-block">
            <input
                type="file"
                name="upload-svg"
                accept=".svg"
                class="absolute w-0 h-0 -z-index-1"
                id=input_id
                on:change=move |ev| {
                    let input = ev
                        .target()
                        .unwrap()
                        .dyn_into::<web_sys::HtmlInputElement>()
                        .unwrap();
                    let file = input.files().unwrap().get(0).unwrap();
                    spawn_local(on_upload_svg_file(file, set_brand, set_color, set_path));
                }
            />

            <Button
                svg_path=&SVGDef::Upload
                title=move_tr!("upload-svg")
                on:click=move |_| {
                    let input = document()
                        .get_element_by_id(input_id)
                        .unwrap()
                        .dyn_into::<web_sys::HtmlElement>()
                        .unwrap();
                    input.click();
                }
            />

        </form>
    }
}

#[component]
fn PreviewCopyButton() -> impl IntoView {
    let (copied, set_copied) = create_signal(false);
    let class = format!(
        "button {}",
        if window().navigator().clipboard().is_none() {
            "hidden"
        } else {
            ""
        }
    );

    let button_id = Ids::PreviewCopyButton.as_str();
    load_keyboard_shortcut_ctrl_and_key_on_click_id(button_id, "c");

    view! {
        <button
            class=class
            id=button_id
            on:click=move |_| {
                let canvas = get_canvas_container();
                spawn_local(copy_canvas_container_as_image(canvas));
                set_copied(true);
                _ = set_timeout_with_handle(
                    move || {
                        set_copied(false);
                    },
                    Duration::from_millis(1000),
                );
            }
        >

            <svg viewBox="0 0 24 24" width="24" height="24">
                {move || match copied() {
                    true => {
                        view! {
                            <>
                                <path d="M0 0h24v24H0z" fill="none"></path>
                                <path d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z"></path>
                            </>
                        }
                            .into_view()
                    }
                    false => view! { <path d=SVGDef::Copy.d()></path> }.into_view(),
                }}

            </svg>
            {move_tr!("copy-preview")}
        </button>
    }
}

#[component]
fn PreviewSaveButton(brand: ReadSignal<String>) -> impl IntoView {
    let button_id = Ids::PreviewSaveButton.as_str();
    load_keyboard_shortcut_ctrl_and_key_on_click_id(button_id, "s");

    view! {
        <Button
            svg_path=&SVGDef::Save
            title=move_tr!("save-preview")
            id=button_id
            on:click=move |_| {
                let canvas = get_canvas_container();
                let filename = format!("{}.png", &sdk::title_to_slug(&brand()));
                let url = canvas.to_data_url().unwrap();
                download(&filename, &url);
            }
        />
    }
}

#[component]
fn PreviewDownloadSVGButton(
    brand: ReadSignal<String>,
    path: ReadSignal<String>,
) -> impl IntoView {
    let title = move_tr!("download-filetype", &{
        let mut map = HashMap::new();
        map.insert("filetype".to_string(), tr!("svg").into());
        map
    });

    let button_id = Ids::PreviewDownloadSVGButton.as_str();
    load_keyboard_shortcut_ctrl_and_key_on_click_id(button_id, "ArrowDown");

    view! {
        <Button
            svg_path=&SVGDef::Download
            title=title
            id=button_id
            on:click=move |_| {
                let filename = format!("{}.svg", &sdk::title_to_slug(&brand()));
                let url = format!(
                    "data:image/svg+xml;utf8,{}",
                    js_sys::encode_uri_component(&svg_with_path_opt_fill(&path(), None)),
                );
                download(&filename, &url);
            }
        />
    }
}
