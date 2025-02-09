use leptos::prelude::*;
use leptos_fluent::tr;
use simple_icons_website_copy::copy_child_img_src_content_from_mouse_event;

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
        <button title=title on:click=copy_child_img_src_content_from_mouse_event>
            <img src=format!("/icons/{}.svg", slug) alt=alt width=56 height=56 />
        </button>
    }
}
