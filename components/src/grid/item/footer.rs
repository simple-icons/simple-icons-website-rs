use crate::controls::download::{
    download_pdf, download_svg, DownloadType, DownloadTypeSignal,
};
use crate::copy::copy_setting_copied_transition_in_element;
use crate::grid::item::details::fill_icon_details_modal_with_icon;
use crate::grid::CurrentIconViewSignal;
use crate::modal::ModalOpenSignal;
use crate::svg::{SVGDef, SVGDefIcon};
use i18n::{move_tr, LocaleSignal};
use leptos::ev::MouseEvent;
use leptos::*;
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

    // Locale context
    let locale_state = expect_context::<LocaleSignal>().0;

    // Modal open context
    let modal_open = expect_context::<ModalOpenSignal>();

    let view_icon_button_title = move_tr!("view-icon", &{
        let mut map = HashMap::new();
        map.insert("icon".to_string(), icon_localized_title().into());
        map
    });

    view! {
        <div>
            <button
                title=move_tr!("copy-hex-color")
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
                    fill_icon_details_modal_with_icon(icon, &locale_state());
                    current_icon_view.update(|state| *state = Some(icon));
                    modal_open.set_icon();
                }
            >

                <SVGDefIcon svg_def=&SVGDef::View/>
            </button>
            <button
                title=move_tr!("download")
                data-error-generating-pdf-msg=move_tr!("error-generating-pdf")
                on:click=move |_| {
                    if download_type() == DownloadType::SVG {
                        download_svg(icon.slug);
                    } else {
                        download_pdf(icon.slug);
                    }
                }
            >

                <SVGDefIcon svg_def=&SVGDef::DownloadThin/>
            </button>
        </div>
    }
}
