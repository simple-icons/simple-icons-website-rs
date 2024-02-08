use leptos::*;
use leptos_fluent::i18n;
use std::collections::HashMap;

#[component]
pub fn IconIsDeprecatedNotice(
    /// Icon brand title
    title: Memo<&'static str>,
    /// Link to the pull request that is removing the icon
    pull_request_url: String,
    /// Removal version
    removal_at_version: &'static str,
) -> impl IntoView {
    let title = move || {
        i18n().trs("will-be-removed-at", &{
            let mut map = HashMap::new();
            map.insert("icon".to_string(), title().into());
            map.insert("version".to_string(), removal_at_version.into());
            map
        })
    };
    view! {
        <a href=pull_request_url class="deprecated" title=title>
            <span></span>
            <p>{move || i18n().tr("deprecated")}</p>
        </a>
    }
}
