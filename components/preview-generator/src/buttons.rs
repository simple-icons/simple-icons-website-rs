use crate::{canvas::canvas as canvas_container, helpers::is_valid_hex_color};
use leptos::{prelude::*, task::spawn_local};
use leptos_fluent::{move_tr, tr};
use simple_icons_sdk as sdk;
use simple_icons_website_controls::download::download;
use simple_icons_website_grid_constants::ICONS;
use simple_icons_website_ids::Ids;
use simple_icons_website_svg_defs::SVGDef;
use simple_icons_website_svg_icon::{SVGIcon, svg_with_title_path_opt_fill};
use wasm_bindgen::JsCast;
use web_sys_simple_copy::copy_canvas_container_as_image;

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
                <PreviewUploadSVGButton set_brand=set_brand set_color=set_color set_path=set_path />
            </div>
            <div class="float-right">
                <PreviewCopyButton />
                <PreviewSaveButton brand=brand />
                <PreviewDownloadSVGButton brand=brand path=path />
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
    async fn on_upload_svg_file(
        file: web_sys::File,
        set_brand: WriteSignal<String>,
        set_color: WriteSignal<String>,
        set_path: WriteSignal<String>,
    ) {
        match wasm_bindgen_futures::JsFuture::from(file.text()).await {
            Ok(text) => {
                let file_content = text.as_string().unwrap();

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
                    let brand = file_content
                        .split("<title>")
                        .nth(1)
                        .unwrap()
                        .split("</title>")
                        .next()
                        .unwrap();
                    set_brand(brand.to_string());

                    if !file_content.contains("fill=\"") {
                        for icon in ICONS.iter() {
                            if icon.title == brand {
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

    // File input hiding needs `max-w-0` and/or `invisible` on Safari:

    view! {
        <form class="inline-block">
            <input
                type="file"
                name="upload-svg"
                accept=".svg"
                class="fixed right-full bottom-full max-w-0 max-h-0 w-0 h-0 overflow-hidden -z-10 invisible"
                id=Ids::PreviewUploadSvgButton
                on:change=move |ev| {
                    let input = event_target::<web_sys::HtmlInputElement>(&ev);
                    let file = input.files().unwrap().get(0).unwrap();
                    spawn_local(on_upload_svg_file(file, set_brand, set_color, set_path));
                    input.set_value("");
                }
            />

            <button
                title=move_tr!("upload-svg")
                class="button"
                type="button"
                tabindex=0
                on:click=move |ev| {
                    event_target::<web_sys::HtmlButtonElement>(&ev)
                        .previous_element_sibling()
                        .unwrap()
                        .dyn_ref::<web_sys::HtmlInputElement>()
                        .unwrap()
                        .click();
                }
            >
                <SVGIcon width="24" height="24" aria_hidden=true path=SVGDef::Upload.d() />
                {move_tr!("upload-svg")}
            </button>
        </form>
    }
}

#[component]
fn PreviewCopyButton() -> impl IntoView {
    let (copied, set_copied) = signal(false);

    view! {
        <button
            class="button"
            type="button"
            id=Ids::PreviewCopyButton
            on:click=move |_| {
                let canvas = canvas_container();
                spawn_local(copy_canvas_container_as_image(canvas));
                set_copied(true);
                set_timeout(
                    move || {
                        set_copied(false);
                    },
                    std::time::Duration::from_secs(1),
                );
            }
        >
            <svg viewBox="0 0 24 24" width="24" height="24">
                <Show when=copied fallback=move || view! { <path d=SVGDef::Copy.d()></path> }>
                    <path d="M0 0h24v24H0z" fill="none"></path>
                    <path d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z"></path>
                </Show>
            </svg>
            {move || tr!("copy-preview")}
        </button>
    }
}

#[component]
fn PreviewSaveButton(brand: ReadSignal<String>) -> impl IntoView {
    view! {
        <button
            title=move || tr!("save-preview")
            class="button"
            type="button"
            id=Ids::PreviewSaveButton
            tabindex=0
            on:click=move |_| {
                let canvas = canvas_container();
                let filename = format!("{}.png", &sdk::title_to_slug(&brand()));
                let url = canvas.to_data_url().unwrap();
                download(&filename, &url);
            }
        >
            <SVGIcon width="24" height="24" aria_hidden=true path=SVGDef::Save.d() />
            {move || tr!("save-preview")}
        </button>
    }
}

#[component]
fn PreviewDownloadSVGButton(
    brand: ReadSignal<String>,
    path: ReadSignal<String>,
) -> impl IntoView {
    view! {
        <button
            title=move_tr!("download-filetype", { "filetype" => tr!("svg") })
            class="button"
            id=Ids::PreviewDownloadSvgButton
            type="button"
            tabindex=0
            on:click=move |_| {
                let filename = format!("{}.svg", &sdk::title_to_slug(&brand()));
                let url = format!(
                    "data:image/svg+xml;utf8,{}",
                    js_sys::encode_uri_component(
                        &svg_with_title_path_opt_fill(&brand(), &path(), None),
                    ),
                );
                download(&filename, &url);
            }
        >
            <SVGIcon width="24" height="24" aria_hidden=true path=SVGDef::Download.d() />
            {move_tr!("download-filetype", { "filetype" => tr!("svg") })}
        </button>
    }
}
