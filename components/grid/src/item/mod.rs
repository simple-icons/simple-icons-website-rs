mod deprecated;
pub mod details;
mod footer;
pub(crate) mod icon_preview;
mod links;
mod title;

use crate::item::title::get_icon_localized_title;
use deprecated::IconIsDeprecatedNotice;
use footer::IconGridItemFooter;
use icon_preview::IconGridItemPreview;
use leptos::prelude::*;
use leptos_fluent::I18n;
use links::IconGridItemLinks;
use simple_icons_website_types::SimpleIcon;
use title::IconGridItemTitle;

/// Icon grid item
///
/// Each icon displayed in the icons grid
#[component]
pub fn IconGridItem(icon: &'static SimpleIcon) -> impl IntoView {
    let icon_localized_title = Memo::new(move |_| {
        get_icon_localized_title(icon, (expect_context::<I18n>().language)())
    });

    view! {
        <li>
            <IconGridItemPreview slug=icon.slug title=icon_localized_title />
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
                            at_version=deprecation.at_version
                            renamed=deprecation.renamed
                        />
                    }
                })}

            <IconGridItemTitle brand_name=icon_localized_title slug=icon.slug />
            <IconGridItemFooter icon=icon icon_localized_title=icon_localized_title />
        </li>
    }
}
