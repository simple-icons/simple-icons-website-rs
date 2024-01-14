use crate::copy::copy_setting_copied_transition_in_element;
use crate::fetch::fetch_text;
use i18n::move_tr;
use leptos::{ev::MouseEvent, wasm_bindgen::JsCast, *};
use std::collections::HashMap;
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
    let button_title = move_tr!("copy-icon-svg", &{
        let mut map = HashMap::new();
        map.insert("icon".to_string(), title().into());
        map
    });
    let img_alt = move_tr!("subject-icon", &{
        let mut map = HashMap::new();
        map.insert("icon".to_string(), title().into());
        map
    });
    view! {
        <button title=button_title on:click=on_click_copy_image_children_src_content>
            <img src=format!("/icons/{}.svg", slug) alt=img_alt width=56 height=56/>
        </button>
    }
}
