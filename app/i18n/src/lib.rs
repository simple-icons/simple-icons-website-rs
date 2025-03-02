use leptos::prelude::*;
use leptos_fluent::leptos_fluent;

fluent_templates::static_loader! {
    static TRANSLATIONS = {
        locales: "./locales",
        fallback_language: "en-US",
        customise: |bundle| bundle.set_use_isolating(false),
    };
}

#[component]
pub fn I18n(
    children: Children,
    url_param: &'static str,
    localstorage_key: &'static str,
) -> impl IntoView {
    leptos_fluent! {
        children: children(),
        locales: "./locales",
        translations: [TRANSLATIONS],
        check_translations: "../../{app,components}/**/*.rs",
        sync_html_tag_lang: true,
        sync_html_tag_dir: true,
        url_param,
        initial_language_from_url_param: true,
        initial_language_from_url_param_to_localstorage: true,
        localstorage_key,
        initial_language_from_localstorage: true,
        set_language_to_localstorage: true,
        initial_language_from_navigator: true,
        initial_language_from_navigator_to_localstorage: true,
    }
}
