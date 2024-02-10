use crate::copy::copy_setting_copied_transition_in_element;
use crate::fetch::fetch_text;
use leptos::{ev::MouseEvent, wasm_bindgen::JsCast, *};
use leptos_fluent::tr;
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
        if let Some(svg) = fetch_text(&src).await {
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
    title: Memo<&'static str>,
) -> impl IntoView {
    let title = move || tr!("copy-icon-svg", {"icon" => title()});
    let alt = move || tr!("subject-icon", {"icon" => title()});
    view! {
        <button title=title on:click=on_click_copy_image_children_src_content>
            <img src=format!("/icons/{}.svg", slug) alt=alt width=56 height=56/>
        </button>
    }
}
