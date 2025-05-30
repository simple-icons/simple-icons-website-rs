use crate::{
    CurrentIconViewSignal, item::details::fill_icon_details_modal_with_icon,
};
use leptos::ev::MouseEvent;
use leptos::prelude::*;
use leptos_fluent::{expect_i18n, move_tr, tr};
use simple_icons_website_controls::download::{
    DownloadType, DownloadTypeSignal, download_png, download_svg,
};
use simple_icons_website_controls_search::focus_search_bar;
use simple_icons_website_copy::copy_and_set_copied_transition;
use simple_icons_website_modal::ModalOpenSignal;
use simple_icons_website_svg_defs::SVGDef;
use simple_icons_website_svg_icon::SVGIcon;
use simple_icons_website_types::SimpleIcon;

/// Icon grid item footer
///
/// Contains the buttons to copy color, view the expanded icon card and download the icon
#[component]
pub fn IconGridItemFooter(
    /// The icon
    icon: &'static SimpleIcon,
    /// Localized brand name
    icon_localized_title: Memo<&'static str>,
) -> impl IntoView {
    // Hex color formatted for CSS
    let css_hex = &format!("#{}", icon.hex);

    // Controls context
    let download_type = expect_context::<DownloadTypeSignal>().0;

    // Context to handle the opening state of detail modals
    let current_icon_view = expect_context::<CurrentIconViewSignal>().0;

    // Modal open context
    let modal_open = expect_context::<ModalOpenSignal>();

    let view_icon_button_title =
        move || tr!("view-icon", {"icon" => icon_localized_title()});

    let i18n = expect_i18n();

    view! {
        <div>
            <button
                title=move_tr!("copy-hex-color")
                class:dark=icon.hex_is_relatively_light
                style=format!("background: {}", css_hex)
                on:click=move |ev: MouseEvent| {
                    let target = event_target::<web_sys::HtmlElement>(&ev);
                    let value = target.text_content().unwrap();
                    copy_and_set_copied_transition(&value, target);
                }
            >

                {css_hex.clone()}
            </button>
            <button
                title=view_icon_button_title
                on:click=move |_| {
                    fill_icon_details_modal_with_icon(i18n, icon);
                    current_icon_view.update(|state| *state = Some(icon));
                    modal_open.set_icon();
                }
            >

                <SVGIcon path=&SVGDef::View />
            </button>
            <button
                title=move_tr!("download")
                data-error-generating-pdf-msg=move_tr!("error-generating-pdf")
                on:click=move |_| {
                    if download_type() == DownloadType::SVG {
                        download_svg(icon.slug);
                    } else {
                        download_png(icon.slug);
                    }
                    focus_search_bar();
                }
            >

                <SVGIcon path=&SVGDef::DownloadThin />
            </button>
        </div>
    }
}
