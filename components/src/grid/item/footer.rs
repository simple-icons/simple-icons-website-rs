use crate::controls::{
    download::{download_pdf, download_svg, DownloadType, DownloadTypeSignal},
    search::focus_search_bar,
};
use crate::copy::copy_setting_copied_transition_in_element;
use crate::grid::item::details::fill_icon_details_modal_with_icon;
use crate::grid::CurrentIconViewSignal;
use crate::modal::ModalOpenSignal;
use crate::svg::{SVGDef, SVGIcon};
use leptos::ev::MouseEvent;
use leptos::*;
use leptos_fluent::i18n;
use std::collections::HashMap;
use types::SimpleIcon;
use web_sys;

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

    let view_icon_button_title = move || {
        i18n().trs("view-icon", &{
            let mut map = HashMap::new();
            map.insert("icon".to_string(), icon_localized_title().into());
            map
        })
    };

    view! {
        <div>
            <button
                title=Signal::derive(move || i18n().tr("copy-hex-color"))
                class:dark=icon.hex_is_relatively_light
                style=format!("background: {}", css_hex)
                on:click=move |ev: MouseEvent| {
                    let target = event_target::<web_sys::HtmlElement>(&ev);
                    let value = target.text_content().unwrap();
                    spawn_local(copy_setting_copied_transition_in_element(value, target));
                }
            >

                {css_hex}
            </button>
            <button
                title=view_icon_button_title
                on:click=move |_| {
                    fill_icon_details_modal_with_icon(icon);
                    current_icon_view.update(|state| *state = Some(icon));
                    modal_open.set_icon();
                }
            >

                <SVGIcon path=&SVGDef::View/>
            </button>
            <button
                title=Signal::derive(move || i18n().tr("download"))
                data-error-generating-pdf-msg=Signal::derive(move || {
                    i18n().tr("error-generating-pdf")
                })

                on:click=move |_| {
                    if download_type() == DownloadType::SVG {
                        download_svg(icon.slug);
                    } else {
                        download_pdf(icon.slug);
                    }
                    focus_search_bar();
                }
            >

                <SVGIcon path=&SVGDef::DownloadThin/>
            </button>
        </div>
    }
}
