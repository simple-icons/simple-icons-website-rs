pub mod image;
pub mod pdf;
pub mod svg;

use crate::controls::button::ControlButtonText;
use crate::storage::LocalStorage;
use crate::Url;
use i18n::{move_tr, tr};
pub use image::{copy_as_base64_jpg, download_jpg, download_png};
use leptos::{document, *};
pub use pdf::download_pdf;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
pub use svg::download_svg;
use wasm_bindgen::JsCast;
use web_sys;

#[derive(Default, Copy, Clone, PartialEq)]
pub enum DownloadType {
    #[default]
    SVG,
    PDF,
}

impl FromStr for DownloadType {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "svg" => Ok(Self::SVG),
            "pdf" => Ok(Self::PDF),
            _ => Err(()),
        }
    }
}

impl fmt::Display for DownloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SVG => write!(f, "svg"),
            Self::PDF => write!(f, "pdf"),
        }
    }
}

pub fn provide_download_type_context() {
    provide_context(DownloadTypeSignal(create_rw_signal(
        initial_download_type(),
    )));
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
        None => match get_download_type_from_localstorage() {
            Some(download_type) => download_type,
            None => DownloadType::default(),
        },
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
    let download_svg_title = move_tr!("download-filetype", &{
        let mut map = HashMap::new();
        map.insert("filetype".to_string(), tr!("svg").into());
        map
    });
    let download_pdf_title = move_tr!("download-filetype", &{
        let mut map = HashMap::new();
        map.insert("filetype".to_string(), tr!("pdf").into());
        map
    });

    view! {
        <div class="control">
            <label>{move_tr!("download")}</label>
            <div class="flex flex-row">
                <ControlButtonText
                    text=move_tr!("svg")
                    title=download_svg_title
                    active=Signal::derive(move || { download_type() == DownloadType::SVG })
                    on:click=move |_| {
                        download_type
                            .update(move |state| {
                                *state = DownloadType::SVG;
                                set_download_type_on_localstorage(state);
                            });
                    }
                />

                <ControlButtonText
                    text=move_tr!("pdf")
                    title=download_pdf_title
                    active=Signal::derive(move || { download_type() == DownloadType::PDF })
                    on:click=move |_| {
                        download_type
                            .update(|state| {
                                *state = DownloadType::PDF;
                                set_download_type_on_localstorage(state);
                            });
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
    let body = document().body().unwrap();
    body.append_child(&link).unwrap();
    link.click();
    body.remove_child(&link).unwrap();
}
