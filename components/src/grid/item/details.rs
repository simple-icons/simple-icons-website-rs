use crate::controls::download::{download_pdf, download_svg};
use crate::copy::copy_inner_text_on_click;
use crate::fetch::fetch_text_forcing_cache;
use crate::grid::item::icon_preview::on_click_copy_image_children_src_content;
use crate::grid::CurrentIconViewSignal;
use crate::modal::{Modal, ModalOpenSignal};
use crate::Ids;
use i18n::{gettext, move_gettext, Language};
use leptos::{document, *};
use types::SimpleIcon;
use wasm_bindgen::JsCast;
use web_sys;

fn get_slug_from_modal_container() -> String {
    document()
        .get_element_by_id(Ids::IconDetailsModal.as_str())
        .unwrap()
        .get_elements_by_tag_name("h3")
        .item(0)
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap()
        .inner_text()
}

pub fn fill_icon_details_modal_with_icon(
    icon: &'static SimpleIcon,
    locale: &Language,
) {
    let modal_body = document()
        .get_element_by_id(Ids::IconDetailsModal.as_str())
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();

    // Set the modal title
    let modal_header = modal_body
        .parent_element()
        .unwrap()
        .previous_element_sibling()
        .unwrap()
        .first_element_child()
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();
    modal_header.set_inner_text(icon.title);

    // Set the slug
    let modal_slug = modal_body
        .get_elements_by_tag_name("h3")
        .item(0)
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();
    modal_slug.set_inner_text(icon.slug);
    modal_slug
        .set_attribute(
            "title",
            &gettext!("Copy {} slug ({})", icon.title, icon.slug),
        )
        .unwrap();

    // Set the copy hex color button
    let modal_hex_color_button = modal_body
        .query_selector(":first-child > :last-child > button")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::HtmlButtonElement>()
        .unwrap();
    modal_hex_color_button.set_inner_text(&format!("#{}", icon.hex));
    modal_hex_color_button
        .set_attribute(
            "style",
            &format!(
                "background-color:#{};color:var(--{}-contrast-color);",
                icon.hex,
                match icon.hex_is_relatively_light {
                    true => "dark",
                    false => "light",
                }
            ),
        )
        .unwrap();
    modal_hex_color_button
        .class_list()
        .add_1(match icon.hex_is_relatively_light {
            true => "copy-button-black",
            false => "copy-button-white",
        })
        .unwrap();

    // Set preview image container src and button title
    let modal_preview_button = modal_body
        .query_selector(":first-child > :first-child > button")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::HtmlButtonElement>()
        .unwrap();
    modal_preview_button
        .set_attribute("title", &gettext!("Copy {} SVG", icon.title))
        .unwrap();
    modal_preview_button
        .children()
        .item(0)
        .unwrap()
        .dyn_into::<web_sys::HtmlImageElement>()
        .unwrap()
        .set_attribute("src", &format!("/icons/{}.svg", icon.slug))
        .unwrap();

    // Set the brand guidelines link
    let modal_brand_guidelines_link = modal_body
        .get_elements_by_tag_name("a")
        .item(0)
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();
    if let Some(guidelines) = icon.guidelines {
        modal_brand_guidelines_link
            .set_attribute("href", guidelines)
            .unwrap();
        modal_brand_guidelines_link
            .class_list()
            .remove_1("hidden")
            .unwrap();
    } else {
        modal_brand_guidelines_link
            .class_list()
            .add_1("hidden")
            .unwrap();
    }

    // Set the license
    let modal_license_link = modal_body
        .get_elements_by_tag_name("a")
        .item(1)
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();
    if icon.license_url.is_some() || icon.license_type.is_some() {
        modal_license_link.class_list().remove_1("hidden").unwrap();
    } else {
        modal_license_link.class_list().add_1("hidden").unwrap();
    }
    if let Some(license_url) = icon.license_url {
        modal_license_link
            .set_attribute("href", license_url)
            .unwrap();
    }
    if let Some(license_type) = icon.license_type {
        modal_license_link.set_inner_text(license_type);
        modal_license_link
            .set_attribute(
                "href",
                &format!("https://spdx.org/licenses/{}", license_type),
            )
            .unwrap();
    } else {
        let title = modal_license_link.get_attribute("title").unwrap();
        modal_license_link.set_inner_text(&title);
    }

    // Set the deprecation information
    let modal_deprecation_paragraph = modal_body
        .get_elements_by_tag_name("p")
        .item(0)
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();

    if let Some(deprecation) = icon.deprecation {
        modal_deprecation_paragraph.set_inner_html(&gettext!(
            "{} will be removed at {} about {} (see {})",
            icon.title,
            &format!(
                "<a href=\"{}\">v{}</a>",
                deprecation.get_milestone_url(),
                deprecation.removal_at_version,
            ),
            &js_sys::Date::new(&wasm_bindgen::JsValue::from(
                deprecation.milestone_due_on,
            ))
            .to_locale_date_string(
                locale.code,
                &wasm_bindgen::JsValue::from(js_sys::Object::new())
            )
            .as_string()
            .unwrap(),
            &format!(
                "<a href=\"{}\">#{}</a>",
                deprecation.get_pull_request_url(),
                deprecation.pull_request_number,
            )
        ));
        modal_deprecation_paragraph
            .class_list()
            .remove_1("hidden")
            .unwrap();
    } else {
        modal_deprecation_paragraph
            .class_list()
            .add_1("hidden")
            .unwrap();
    }

    // Set download buttons
    let modal_footer = modal_body
        .first_element_child()
        .unwrap()
        .next_element_sibling()
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();

    let download_colored_icon_container = modal_footer
        .get_elements_by_tag_name("a")
        .item(0)
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();

    spawn_local(async move {
        if let Some(svg) =
            fetch_text_forcing_cache(&format!("/icons/{}.svg", icon.slug)).await
        {
            let colored_icon_svg =
                svg.replace("<svg", &format!("<svg fill=\"%23{}\"", icon.hex));
            download_colored_icon_container
                .set_attribute(
                    "href",
                    &format!("data:image/svg+xml,{}", &colored_icon_svg),
                )
                .unwrap();
            download_colored_icon_container
                .set_attribute("download", &format!("{}-color.svg", icon.slug))
                .unwrap();
        }
    });
}

/// Details modal icon preview
#[component]
fn IconDetailsModalPreview() -> impl IntoView {
    view! {
        <button on:click=on_click_copy_image_children_src_content>
            <img/>
        </button>
    }
}

/// Details modal icon information
#[component]
fn IconDetailsModalInformation() -> impl IntoView {
    view! {
        <div>
            <h3 on:click=copy_inner_text_on_click></h3>
            <button
                on:click=copy_inner_text_on_click
                title=move_gettext!("Copy hex color")
            ></button>
            <a target="_blank">{move_gettext!("Brand guidelines")}</a>
            <a target="_blank" title=move_gettext!("License")></a>
            <p></p>
        </div>
    }
}

#[component]
fn IconDetailsModalFooter() -> impl IntoView {
    view! {
        <div>
            <button
                on:click=move |_| download_svg(&get_slug_from_modal_container())
                aria-label=move_gettext!("Download SVG")
            >
                {move_gettext!("Download SVG")}
            </button>
            <a aria-label=move_gettext!("Download colored SVG")>
                {move_gettext!("Download colored SVG")}
            </a>
            <button
                on:click=move |_| download_pdf(&get_slug_from_modal_container())
                aria-label=move_gettext!("Download PDF")
            >
                {move_gettext!("Download PDF")}
            </button>
        </div>
    }
}

/// Detail modal view for icons
#[component]
pub fn IconDetailsModal() -> impl IntoView {
    let current_icon_view = use_context::<CurrentIconViewSignal>().unwrap().0;
    let modal_open = use_context::<ModalOpenSignal>().unwrap();

    view! {
        <Modal
            title=move || "".to_string()
            title_is_copyable=true
            is_open=move || current_icon_view().is_some()
            on_close=move |_| {
                current_icon_view.update(|state| *state = None);
                modal_open.set_none();
            }
        >
            <div class="icon-details-modal" id=Ids::IconDetailsModal.as_str()>
                <div>
                    <IconDetailsModalPreview/>
                    <IconDetailsModalInformation/>
                </div>
                <IconDetailsModalFooter/>
            </div>
        </Modal>
    }
}
