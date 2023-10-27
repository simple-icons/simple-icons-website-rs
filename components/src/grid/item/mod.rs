mod deprecated;
pub mod details;
mod footer;
pub(crate) mod icon_preview;
mod links;
mod title;

use crate::grid::item::title::get_icon_localized_title;
use deprecated::IconIsDeprecatedNotice;
use footer::IconGridItemFooter;
use i18n::LocaleSignal;
use icon_preview::IconGridItemPreview;
use links::IconGridItemLinks;
use title::IconGridItemTitle;
use types::SimpleIcon;

use leptos::*;

/// Icon grid item
///
/// Each icon displayed in the icons grid
#[component]
pub fn IconGridItem(
    /// Icon
    icon: &'static SimpleIcon,
) -> impl IntoView {
    let locale_signal = expect_context::<LocaleSignal>().0;
    let icon_localized_title =
        create_memo(move |_| get_icon_localized_title(icon, &locale_signal()));

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
