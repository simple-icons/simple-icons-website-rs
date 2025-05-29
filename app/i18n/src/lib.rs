use leptos::prelude::*;
use leptos_fluent::leptos_fluent;

fluent_templates::static_loader! {
    static TRANSLATIONS = {
        locales: "./locales",
        fallback_language: "en-US",
        // Whithout configuring no isolation, a lot of character marks are
        // added to the text and makes the website impossible to test.
        //
        // According to fluent-templates documentation, these marks are used
        // to display the text correctly in right-to-left languages. We don't
        // currently have any right-to-left languages in the project, so it's
        // disabled to make testing easier. We can enable it later if this
        // affects the rendering of right-to-left languages.
        // See https://github.com/XAMPPRocky/fluent-templates?tab=readme-ov-file#why-is-there-extra-characters-around-the-values-of-arguments
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
        default_language: "en-US",
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
