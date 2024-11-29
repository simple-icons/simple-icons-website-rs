use leptos::*;
use leptos_fluent::tr;

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
        tr!("will-be-removed-at", {
            "icon" => title(),
            "version" => removal_at_version,
        })
    };
    view! {
        <a href=pull_request_url class="deprecated" title=title target="_blank">
            <span></span>
            <p>{move || tr!("deprecated")}</p>
        </a>
    }
}
