use i18n::move_gettext;
use leptos::*;

#[component]
pub fn IconIsDeprecatedNotice(
    /// Icon brand title
    title: &'static str,
    /// Link to the pull request that is removing the icon
    pull_request_url: String,
    /// Removal version
    removal_at_version: &'static str,
) -> impl IntoView {
    view! {
        <a
            href=pull_request_url
            class="deprecated"
            title=move_gettext!("{} will be removed at v{}", title, removal_at_version)
        >
            <span></span>
            <p>{move_gettext!("Deprecated")}</p>
        </a>
    }
}
