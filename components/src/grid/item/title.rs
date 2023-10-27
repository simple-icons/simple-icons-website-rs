use crate::copy::copy_setting_copied_transition_in_element;
use i18n::move_tr;
use i18n::Language;
use leptos::{ev::MouseEvent, *};
use std::collections::HashMap;
use types::SimpleIcon;
use web_sys;

pub fn get_icon_localized_title(
    icon: &'static SimpleIcon,
    language: &Language,
) -> String {
    let current_code = language.id.to_string();
    let current_lang = language.id.language.to_string();
    if let Some(aliases) = icon.aliases {
        if let Some(loc) = aliases.loc {
            for (lang, loc_title) in loc {
                let loc_language = lang.to_string();
                if loc_language == current_code || loc_language == current_lang
                {
                    return loc_title.to_string();
                }
            }
        }
    }
    icon.title.to_string()
}

/// Icon grid item title
#[component]
pub fn IconGridItemTitle(
    /// Brand title
    brand_name: Memo<String>,
    /// Slug
    slug: &'static str,
) -> impl IntoView {
    let container_title = move_tr!("copy-icon-slug", &{
        let mut map = HashMap::new();
        map.insert("icon".to_string(), brand_name().into());
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
