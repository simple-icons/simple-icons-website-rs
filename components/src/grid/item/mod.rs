mod deprecated;
pub mod details;
mod footer;
pub(crate) mod icon_preview;
mod links;
mod title;

use crate::grid::item::title::get_icon_localized_title;
use deprecated::IconIsDeprecatedNotice;
use footer::IconGridItemFooter;
use icon_preview::IconGridItemPreview;
use leptos::*;
use leptos_fluent::i18n;
use links::IconGridItemLinks;
use title::IconGridItemTitle;
use types::SimpleIcon;

/// Icon grid item
///
/// Each icon displayed in the icons grid
#[component]
pub fn IconGridItem(icon: &'static SimpleIcon) -> impl IntoView {
    let i18n = i18n();
    let icon_localized_title = create_memo(move |_| {
        get_icon_localized_title(icon, i18n.language.get())
    });

    view! {
        <li>
            <IconGridItemPreview slug=icon.slug title=icon_localized_title/>
            <IconGridItemLinks
                guidelines_url=icon.guidelines
                license_url=icon.license_url
                license_type=icon.license_type
            />
            {icon
                .deprecation
                .as_ref()
                .map(|deprecation| {
                    view! {
                        <IconIsDeprecatedNotice
                            title=icon_localized_title
                            pull_request_url=deprecation.get_pull_request_url()
                            removal_at_version=deprecation.removal_at_version
                        />
                    }
                })}

            <IconGridItemTitle brand_name=icon_localized_title slug=icon.slug/>
            <IconGridItemFooter icon=icon icon_localized_title=icon_localized_title/>
        </li>
    }
}
