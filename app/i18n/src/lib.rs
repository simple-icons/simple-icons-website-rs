use leptos::prelude::*;
use leptos_fluent::leptos_fluent;

#[component]
pub fn I18n(
    children: Children,
    url_param: &'static str,
    local_storage_key: &'static str,
) -> impl IntoView {
    leptos_fluent! {
        children: children(),
        locales: "./locales",
        default_language: "en-US",
        check_translations: true,
        sync_html_tag_lang: true,
        sync_html_tag_dir: true,
        url_param,
        initial_language_from_url_param: true,
        initial_language_from_url_param_to_local_storage: true,
        local_storage_key,
        initial_language_from_local_storage: true,
        set_language_to_local_storage: true,
        initial_language_from_navigator: true,
        initial_language_from_navigator_to_local_storage: true,
        customise: |bundle| bundle.set_use_isolating(false),
    }
}
