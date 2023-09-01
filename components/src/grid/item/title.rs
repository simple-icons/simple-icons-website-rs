use crate::copy::copy_setting_copied_transition_in_element;
use i18n::move_tr;
use leptos::{ev::MouseEvent, *};
use std::collections::HashMap;
use web_sys;

/// Icon grid item title
#[component]
pub fn IconGridItemTitle(
    /// Brand title
    brand_name: &'static str,
    /// Slug
    slug: &'static str,
) -> impl IntoView {
    let container_title = move_tr!("copy-icon-slug", &{
        let mut map = HashMap::new();
        map.insert("icon".to_string(), brand_name.into());
        map.insert("slug".to_string(), slug.into());
        map
    });
    view! {
        <h2
            title=container_title
            tabindex=0
            on:click=move |ev: MouseEvent| {
                let target = event_target::<web_sys::HtmlElement>(&ev);
                spawn_local(copy_setting_copied_transition_in_element(slug.to_string(), target));
            }
        >

            {brand_name}
        </h2>
    }
}
