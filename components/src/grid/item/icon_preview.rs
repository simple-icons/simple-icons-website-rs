use crate::copy::copy_setting_copied_transition_in_element;
use crate::fetch::fetch_text_forcing_cache;
use i18n::move_gettext;
use leptos::{ev::MouseEvent, *};
use wasm_bindgen::JsCast;
use web_sys;

/// Copy image children source content to clipboard
pub(crate) fn on_click_copy_image_children_src_content(ev: MouseEvent) {
    let target = event_target::<web_sys::HtmlElement>(&ev);
    let src = target
        .children()
        .item(0)
        .unwrap()
        .dyn_into::<web_sys::HtmlImageElement>()
        .unwrap()
        .get_attribute("src")
        .unwrap();
    spawn_local(async move {
        if let Some(svg) = fetch_text_forcing_cache(&src).await {
            copy_setting_copied_transition_in_element(svg, target).await
        }
    });
}

/// Icon grid item preview
///
/// The icon preview in the grid.
/// Contains the lazy loaded logo of the brand.
#[component]
pub fn IconGridItemPreview(
    /// Icon slug
    slug: &'static str,
    /// Brand title
    title: &'static str,
) -> impl IntoView {
    view! {
        <button
            title=move_gettext!("Copy {} SVG", title)
            on:click=on_click_copy_image_children_src_content
        >
            <img src=format!("/icons/{}.svg", slug) alt=move_gettext!("{} icon", title)/>
        </button>
    }
}
