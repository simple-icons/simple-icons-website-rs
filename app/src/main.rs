//! Simple Icons website
//!
//! Built with Rust using [Leptos](https://docs.rs/leptos)
//! and [Tailwind CSS](https://tailwindcss.com/).
//!
//! See the contributing guide for instructions on how to
//! develop and build for production.
//!
pub(crate) mod app;
pub(crate) mod head;
pub(crate) mod pages;

use crate::app::App;
use leptos::{document, mount_to_body, view};
use log::Level;

pub fn main() {
    _ = console_log::init_with_level(Level::Debug);
    console_error_panic_hook::set_once();

    // Remove #loader and body background
    document().get_element_by_id("loader").unwrap().remove();
    document().body().unwrap().remove_attribute("style").ok();

    mount_to_body(|| view! { <App /> });
}
