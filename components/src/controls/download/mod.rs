pub mod image;
pub mod pdf;
pub mod svg;

use crate::controls::button::ControlButtonText;
use crate::storage::LocalStorage;
use crate::Url;
pub use image::{
    copy_as_base64_jpg, copy_as_base64_png, copy_as_image_jpg,
    copy_as_image_png, download_jpg, download_png,
};
use leptos::prelude::{document, *};
use leptos_fluent::{move_tr, tr};
pub use pdf::download_pdf;
use std::fmt;
use std::str::FromStr;
pub use svg::download_svg;
use wasm_bindgen::JsCast;
use web_sys;

#[derive(Default, Copy, Clone, PartialEq)]
pub enum DownloadType {
    #[default]
    SVG,
    PNG,
}

impl FromStr for DownloadType {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "png" => Ok(Self::PNG),
            _ => Ok(Self::SVG),
        }
    }
}

impl fmt::Display for DownloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SVG => write!(f, "svg"),
            Self::PNG => write!(f, "png"),
        }
    }
}

pub fn provide_download_type_context() {
    provide_context(DownloadTypeSignal(RwSignal::new(initial_download_type())));
}

#[derive(Copy, Clone)]
pub struct DownloadTypeSignal(pub RwSignal<DownloadType>);

fn initial_download_type() -> DownloadType {
    match Url::params::get(&Url::params::Names::DownloadType)
        .and_then(|value| value.parse().ok())
    {
        Some(download_type) => {
            set_download_type_on_localstorage(&download_type);
            download_type
        }
        None => get_download_type_from_localstorage().unwrap_or_default(),
    }
}

fn get_download_type_from_localstorage() -> Option<DownloadType> {
    LocalStorage::get(LocalStorage::Keys::DownloadType)
        .as_ref()
        .and_then(|value| DownloadType::from_str(value).ok())
}

fn set_download_type_on_localstorage(download_type: &DownloadType) {
    LocalStorage::set(
        LocalStorage::Keys::DownloadType,
        &download_type.to_string(),
    );
}

#[component]
pub fn DownloadFileTypeControl() -> impl IntoView {
    let download_type = expect_context::<DownloadTypeSignal>().0;

    view! {
        <div class="control">
            <label>{move || tr!("download")}</label>
            <div class="flex flex-row">
                <ControlButtonText
                    text=move_tr!("svg")
                    title=move_tr!("download-filetype", {"filetype" => tr!("svg") })
                    active=Signal::derive(move || { download_type() == DownloadType::SVG })
                    on:click=move |_| {
                        download_type.set(DownloadType::SVG);
                        set_download_type_on_localstorage(&DownloadType::SVG);
                    }
                />

                <ControlButtonText
                    text=move_tr!("png")
                    title=move_tr!("download-filetype", { "filetype" => tr!("png") })
                    active=Signal::derive(move || { download_type() == DownloadType::PNG })
                    on:click=move |_| {
                        download_type.set(DownloadType::PNG);
                        set_download_type_on_localstorage(&DownloadType::PNG);
                    }
                />

            </div>
        </div>
    }
}

/// Download a SVG icon by its slug
pub fn download(filename: &str, href: &str) {
    let link = document()
        .create_element("a")
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();
    link.set_attribute("class", "hidden").unwrap();
    link.set_attribute("download", filename).unwrap();
    link.set_attribute("href", href).unwrap();
    link.click();
}
