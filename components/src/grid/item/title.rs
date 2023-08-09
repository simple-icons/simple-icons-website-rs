use crate::copy::copy_setting_copied_transition_in_element;
use i18n::move_gettext;
use leptos::{ev::MouseEvent, *};
use web_sys;

/// Icon grid item title
#[component]
pub fn IconGridItemTitle(
    /// Brand title
    title: &'static str,
    /// Slug
    slug: &'static str,
) -> impl IntoView {
    view! {
        <h2
            title=move_gettext!("Copy {} slug ({})", title, slug)
            tabindex=0
            on:click=move |ev: MouseEvent| {
                let target = event_target::<web_sys::HtmlElement>(&ev);
                spawn_local(copy_setting_copied_transition_in_element(slug.to_string(), target));
            }
        >
            {title}
        </h2>
    }
}
