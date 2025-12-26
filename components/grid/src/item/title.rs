use leptos::{ev::MouseEvent, prelude::*};
use leptos_fluent::{Language, move_tr};
use simple_icons_website_copy::copy_and_set_copied_transition;
use simple_icons_website_types::SimpleIcon;
use unic_langid::{LanguageIdentifier, LanguageIdentifierError};

pub fn get_icon_localized_title(
    icon: &'static SimpleIcon,
    language: &Language,
) -> &'static str {
    if let Some(aliases) = icon.aliases
        && let Some(loc) = aliases.loc
    {
        let current_lang_region = language.id;
        let maybe_current_lang: Result<
            LanguageIdentifier,
            LanguageIdentifierError,
        > = language.id.parse();
        if let Err(e) = &maybe_current_lang {
            leptos::logging::warn!(
                "Failed to parse language identifier: {}",
                e
            );
        }

        for (lang, loc_title) in loc {
            if *lang == current_lang_region {
                return loc_title;
            }
        }

        let current_lang = maybe_current_lang.unwrap().language.to_string();

        for (lang, loc_title) in loc {
            let mut loc_language = lang.to_string();
            if loc_language.contains('-') {
                loc_language =
                    loc_language.split('-').next().unwrap().to_string();
            }
            if loc_language == current_lang {
                return loc_title;
            }
        }
    }
    icon.title
}

/// Icon grid item title
#[component]
pub fn IconGridItemTitle(
    /// Brand title
    brand_name: Memo<&'static str>,
    /// Slug
    slug: &'static str,
) -> impl IntoView {
    view! {
        <h2
            title=move_tr!("copy-icon-slug", {"icon" => brand_name(), "slug" => slug})
            tabindex=0
            on:click=move |ev: MouseEvent| {
                let target = event_target::<web_sys::HtmlElement>(&ev);
                copy_and_set_copied_transition(slug, target);
            }
        >
            {brand_name}
        </h2>
    }
}
