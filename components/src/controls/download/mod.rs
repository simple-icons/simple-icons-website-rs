pub mod pdf;
pub mod svg;

use crate::controls::button::ControlButtonText;
use crate::storage::LocalStorage;
use crate::Url;
use i18n::move_gettext;
use leptos::{document, window, *};
pub use pdf::download_pdf;
use std::fmt;
pub use svg::download_svg;
use wasm_bindgen::JsCast;
use web_sys;

#[derive(Default, Copy, Clone, PartialEq)]
pub enum DownloadType {
    #[default]
    SVG,
    PDF,
}

impl DownloadType {
    fn from_str(download_type: &str) -> Option<Self> {
        match download_type {
            "svg" => Some(Self::SVG),
            "pdf" => Some(Self::PDF),
            _ => None,
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
    match download_type_from_url() {
        Some(download_type) => {
            set_download_type_on_localstorage(&download_type);
            download_type
        }
        None => match download_type_from_localstorage() {
            Some(download_type) => download_type,
            None => DownloadType::default(),
        },
    }
}

fn download_type_from_url() -> Option<DownloadType> {
    match Url::params::get(&Url::params::Names::DownloadType) {
        Some(download_type) => DownloadType::from_str(download_type.as_str()),
        None => None,
    }
}

fn download_type_from_localstorage() -> Option<DownloadType> {
    match window()
        .local_storage()
        .unwrap()
        .unwrap()
        .get_item(LocalStorage::Keys::DownloadType.as_str())
    {
        Ok(Some(download_type)) => {
            DownloadType::from_str(download_type.as_str())
        }
        _ => None,
    }
}

fn set_download_type_on_localstorage(download_type: &DownloadType) {
    window()
        .local_storage()
        .unwrap()
        .unwrap()
        .set_item(
            LocalStorage::Keys::DownloadType.as_str(),
            &download_type.to_string(),
        )
        .unwrap();
}

#[component]
pub fn DownloadFileTypeControl() -> impl IntoView {
    let download_type = use_context::<DownloadTypeSignal>().unwrap().0;

    view! {
        <div class="control">
            <label>{move_gettext!("Download")}</label>
            <div class="flex flex-row">
                <ControlButtonText
                    text=move_gettext!("SVG")
                    title=move_gettext!("Download SVG")
                    active=move || { download_type() == DownloadType::SVG }
                    on:click=move |_| {
                        download_type
                            .update(move |state| {
                                *state = DownloadType::SVG;
                                set_download_type_on_localstorage(state);
                            });
                    }
                />
                <ControlButtonText
                    text=move_gettext!("PDF")
                    title=move_gettext!("Download PDF")
                    active=move || { download_type() == DownloadType::PDF }
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
