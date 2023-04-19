mod deprecated;
pub mod details;
mod footer;
mod icon_preview;
mod links;
mod title;

use deprecated::*;
use footer::*;
use icon_preview::*;
use links::*;
use simple_icons::StaticSimpleIcon;
use title::*;

use leptos::*;

/// Icon grid item
///
/// Each icon displayed in the icons grid
#[component]
pub fn IconGridItem(
    cx: Scope,
    /// Icon
    icon: &'static StaticSimpleIcon,
) -> impl IntoView {
    view! { cx,
        <li>
            <IconGridItemPreview slug=icon.slug title=icon.title/>
            <IconGridItemLinks
                guidelines_url=icon.guidelines_url
                license_url=icon.license_url
                license_type=icon.license_type
            />
            {if icon.is_deprecated {
                Some(
                    view! { cx,
                        <IconIsDeprecatedNotice
                            title=icon.title
                            pull_request_url=icon.deprecation_pull_request_url.unwrap()
                            removal_at_version=icon.removal_at_version.unwrap()
                        />
                    },
                )
            } else {
                None
            }}
            <IconGridItemTitle title=icon.title slug=icon.slug/>
            <IconGridItemFooter icon=icon/>
        </li>
    }
}
