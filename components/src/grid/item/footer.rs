use crate::controls::download::{
    download_pdf, download_svg, DownloadType, DownloadTypeSignal,
};
use crate::copy::copy_setting_copied_transition_in_element;
use crate::grid::item::details::fill_icon_details_modal_with_icon;
use crate::grid::CurrentIconViewSignal;
use crate::modal::ModalOpenSignal;
use crate::svg_defs::SVGDefs;
use i18n::{move_gettext, LocaleSignal};
use leptos::ev::MouseEvent;
use leptos::*;
use types::SimpleIcon;
use web_sys;

/// Icon grid item footer
///
/// Contains the buttons to copy color, view the expanded icon card and download the icon
#[component]
pub fn IconGridItemFooter(
    /// The icon
    icon: &'static SimpleIcon,
) -> impl IntoView {
    // Hex color formatted for CSS
    let css_hex = &format!("#{}", icon.hex);

    // Controls context
    let download_type = use_context::<DownloadTypeSignal>().unwrap().0;

    // Context to handle the opening state of detail modals
    let current_icon_view = use_context::<CurrentIconViewSignal>().unwrap().0;

    // Locale context
    let locale_state = use_context::<LocaleSignal>().unwrap().0;

    // Modal open context
    let modal_open = use_context::<ModalOpenSignal>().unwrap();

    view! {
        <div>
            <button
                title=move_gettext!("Copy hex color")
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
                title=move_gettext!("View {}", icon.title)
                on:click=move |_| {
                    fill_icon_details_modal_with_icon(icon, &locale_state());
                    current_icon_view.update(|state| *state = Some(icon));
                    modal_open.set_icon();
                }
            >
                <svg viewBox="0 0 24 24">
                    <use_ href=format!("#{}", SVGDefs::ViewPath.id())></use_>
                </svg>
            </button>
            <button
                title=move_gettext!("Download")
                data-error-generating-pdf-msg-schema=move_gettext!("Error generating PDF with PDFKit library: {}")
                on:click=move |_| {
                    if download_type() == DownloadType::SVG {
                        download_svg(icon.slug);
                    } else {
                        download_pdf(icon.slug);
                    }
                }
            >
                <svg viewBox="0 0 24 24">
                    <use_ href=format!("#{}", SVGDefs::DownloadPath.id())></use_>
                </svg>
            </button>
        </div>
    }
}
