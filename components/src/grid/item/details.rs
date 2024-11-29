use crate::controls::download::{
    copy_as_base64_jpg, copy_as_base64_png, copy_as_image_jpg,
    copy_as_image_png, download, download_jpg, download_pdf, download_png,
    download_svg,
};
use crate::copy::{copy_and_set_copied_transition, copy_inner_text_on_click};
use crate::fetch::fetch_text;
use crate::grid::item::icon_preview::on_click_copy_image_children_src_content;
use crate::grid::item::title::get_icon_localized_title;
use crate::grid::CurrentIconViewSignal;
use crate::menu::{Menu, MenuItem};
use crate::modal::{Modal, ModalOpenSignal};
use crate::Ids;
use icondata::{
    BiCheckRegular, BiMenuAltRightRegular, BiMenuRegular, BsCode,
    BsWindowFullscreen, IoColorWand, TbJpg, TbPdf, TbPng, TbSvg,
    VsSymbolNamespace,
};
use leptos::{html::Ul, wasm_bindgen::JsCast, *};
use leptos_fluent::{expect_i18n, move_tr, tr};
use leptos_icons::Icon;
use leptos_use::on_click_outside;
use simple_icons_website_types::SimpleIcon;
use web_sys;

fn get_brand_name_from_modal_container() -> String {
    document()
        .get_element_by_id(Ids::IconDetailsModal.as_str())
        .unwrap()
        .parent_element()
        .unwrap()
        .parent_element()
        .unwrap()
        .get_elements_by_tag_name("h2")
        .item(0)
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap()
        .inner_text()
}

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

fn get_hex_from_modal_container() -> String {
    document()
        .get_element_by_id(Ids::IconDetailsModal.as_str())
        .unwrap()
        .get_elements_by_tag_name("button")
        .item(1)
        .unwrap()
        .dyn_into::<web_sys::HtmlButtonElement>()
        .unwrap()
        .inner_text()
}

pub fn fill_icon_details_modal_with_icon(icon: &'static SimpleIcon) {
    let language = expect_i18n().language.get();
    let icon_localized_title = get_icon_localized_title(icon, language);

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
    modal_header.set_inner_text(icon_localized_title);

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
            &tr!("copy-icon-slug", {
                "icon" => icon_localized_title,
                "slug" => icon.slug,
            }),
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
        .set_attribute(
            "title",
            &tr!("copy-icon-svg", {
                "icon" => icon_localized_title,
            }),
        )
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
        modal_deprecation_paragraph.set_inner_html(&tr!(
            "will-be-removed-at-extended",
            {
                "icon" => icon_localized_title,
                "version" => format!(
                    "<a href=\"{}\" target=\"_blank\">v{}</a>",
                    deprecation.get_milestone_url(),
                    deprecation.removal_at_version,
                ),
                "date" => js_sys::Date::new(&wasm_bindgen::JsValue::from(
                        deprecation.milestone_due_on,
                    ))
                    .to_locale_date_string(
                        &language.id.to_string(),
                        &wasm_bindgen::JsValue::from(js_sys::Object::new()),
                    )
                    .as_string()
                    .unwrap(),
                "pr" => format!(
                    "<a href=\"{}\" target=\"_blank\">#{}</a>",
                    deprecation.get_pull_request_url(),
                    deprecation.pull_request_number,
                ),
            },
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
}

/// Details modal icon preview
#[component]
fn IconDetailsModalPreview() -> impl IntoView {
    view! {
        <button on:click=on_click_copy_image_children_src_content>
            <img />
        </button>
    }
}

/// Details modal icon information
#[component]
fn IconDetailsModalInformation() -> impl IntoView {
    view! {
        <div>
            <h3 on:click=copy_inner_text_on_click></h3>
            <button on:click=copy_inner_text_on_click title=move || tr!("copy-hex-color")></button>
            <a target="_blank">{move || tr!("brand-guidelines")}</a>
            <a target="_blank" title=move || tr!("license")></a>
            <p></p>
        </div>
    }
}

/// Detail modal view for icons
#[component]
pub fn IconDetailsModal() -> impl IntoView {
    let current_icon_view = expect_context::<CurrentIconViewSignal>().0;
    let modal_open = expect_context::<ModalOpenSignal>();

    let (controls_open, set_controls_open) = create_signal(false);
    let menu_ref = create_node_ref::<Ul>();
    _ = on_click_outside(menu_ref, move |_| {
        if controls_open.get_untracked() {
            set_controls_open(false);
        }
    });

    let modal_is_open = Signal::derive(move || current_icon_view().is_some());

    let (copying_as_base64_svg, set_copying_as_base64_svg) =
        create_signal(false);
    let copy_as_base64_svg_icon = create_memo(move |_| {
        if copying_as_base64_svg() {
            BiCheckRegular
        } else {
            BsCode
        }
    });

    let copy_as_base64_svg_text = create_memo(move |_| {
        if copying_as_base64_svg() {
            tr!("copied")
        } else {
            tr!("copy-as-base64-svg")
        }
    });

    let (copying_as_base64_jpg, set_copying_as_base64_jpg) =
        create_signal(false);
    let copy_as_base64_jpg_icon = create_memo(move |_| {
        if copying_as_base64_jpg() {
            BiCheckRegular
        } else {
            BsCode
        }
    });

    let copy_as_base64_jpg_text = create_memo(move |_| {
        if copying_as_base64_jpg() {
            tr!("copied")
        } else {
            tr!("copy-as-base64-jpg")
        }
    });

    let (copying_as_base64_png, set_copying_as_base64_png) =
        create_signal(false);
    let copy_as_base64_png_icon = create_memo(move |_| {
        if copying_as_base64_png() {
            BiCheckRegular
        } else {
            BsCode
        }
    });

    let copy_as_base64_png_text = create_memo(move |_| {
        if copying_as_base64_png() {
            tr!("copied")
        } else {
            tr!("copy-as-base64-png")
        }
    });

    let (copying_hex, set_copying_hex) = create_signal(false);
    let copy_hex_msg = create_memo(move |_| {
        if copying_hex() {
            tr!("copied")
        } else {
            tr!("copy-hex-color")
        }
    });

    let copy_hex_icon = create_memo(move |_| {
        if copying_hex() {
            BiCheckRegular
        } else {
            IoColorWand
        }
    });

    let controls_menu_item_class = move || {
        concat!(
            "my-auto dark:bg-gray-700 bg-slate-300 text-sm",
            " hover:bg-slate-200 dark:hover:bg-slate-600 z-50"
        )
        .to_string()
    };

    let download_svg_msg =
        move_tr!("download-filetype", {"filetype" => tr!("svg")});
    let download_colored_svg_msg =
        move_tr!("download-filetype", {"filetype" => tr!("colored-svg")});
    let download_pdf_msg =
        move_tr!("download-filetype", {"filetype" => tr!("pdf")});
    let download_jpg_msg =
        move_tr!("download-filetype", {"filetype" => tr!("jpg")});
    let download_png_msg =
        move_tr!("download-filetype", {"filetype" => tr!("png")});

    let (copying_svg, set_copying_svg) = create_signal(false);
    let copy_svg_msg = create_memo(move |_| match copying_svg() {
        true => tr!("copied"),
        false => tr!("copy-filetype", {"filetype" => tr!("svg")}),
    });

    let copy_svg_icon = create_memo(move |_| match copying_svg() {
        true => BiCheckRegular,
        false => TbSvg,
    });

    let (copying_png, set_copying_png) = create_signal(false);
    let copy_png_msg = create_memo(move |_| match copying_png() {
        true => tr!("copied"),
        false => tr!("copy-filetype", {"filetype" => tr!("png")}),
    });

    let copy_png_icon = create_memo(move |_| match copying_png() {
        true => BiCheckRegular,
        false => TbPng,
    });

    let (copying_jpg, set_copying_jpg) = create_signal(false);
    let copy_jpg_msg = create_memo(move |_| match copying_jpg() {
        true => tr!("copied"),
        false => tr!("copy-filetype", {"filetype" => tr!("jpg")}),
    });

    let copy_jpg_icon = create_memo(move |_| match copying_jpg() {
        true => BiCheckRegular,
        false => TbJpg,
    });

    let (copying_brand_name, set_copying_brand_name) = create_signal(false);
    let copy_brand_name_msg =
        create_memo(move |_| match copying_brand_name() {
            true => tr!("copied"),
            false => tr!("copy-brand-name"),
        });

    let copy_brand_name_icon =
        create_memo(move |_| match copying_brand_name() {
            true => BiCheckRegular,
            false => VsSymbolNamespace,
        });

    let (copying_icon_modal_url, set_copying_icon_modal_url) =
        create_signal(false);
    let copy_icon_modal_url_msg = create_memo(move |_| {
        if copying_icon_modal_url() {
            tr!("copied")
        } else {
            tr!("copy-icon-modal-url")
        }
    });

    let copy_icon_modal_url_icon = create_memo(move |_| {
        if copying_icon_modal_url() {
            BiCheckRegular
        } else {
            BsWindowFullscreen
        }
    });

    view! {
        <Modal
            title_is_copyable=true
            is_open=modal_is_open
            on_close_focus_search_bar=true
            on_close=Signal::derive(move || {
                current_icon_view.update(|state| *state = None);
                modal_open.set_none();
            })
        >

            <div class="icon-details-modal" id=Ids::IconDetailsModal.as_str()>
                <div>
                    <IconDetailsModalPreview />
                    <IconDetailsModalInformation />
                </div>
                <div class="cursor-pointer absolute right-[47px] top-[14px] z-50">
                    <span on:click=move |_| set_controls_open(!controls_open.get_untracked())>
                        <Icon
                            width="27"
                            height="27"
                            icon=Signal::derive(move || match controls_open() {
                                true => BiMenuRegular,
                                false => BiMenuAltRightRegular,
                            })
                        />

                    </span>
                    <Show when=controls_open>
                        <Menu
                            ref_=menu_ref
                            class=concat!(
                                "absolute top-8 right-1 text-sm",
                                " border-custom-divider-color bg-slate-300 dark:bg-gray-700",
                                " max-h-[330px] scroll-bar overflow-y-auto",
                            )
                        >

                            <MenuItem
                                class=controls_menu_item_class()
                                text=download_svg_msg
                                icon=Signal::derive(move || TbSvg)
                                on:click=move |_| {
                                    let slug = get_slug_from_modal_container();
                                    download_svg(&slug);
                                }
                            />

                            <MenuItem
                                class=controls_menu_item_class()
                                text=download_pdf_msg
                                icon=Signal::derive(move || TbPdf)
                                on:click=move |_| {
                                    let slug = get_slug_from_modal_container();
                                    download_pdf(&slug);
                                }
                            />

                            <MenuItem
                                class=controls_menu_item_class()
                                text=download_png_msg
                                icon=Signal::derive(move || TbPng)
                                on:click=move |_| {
                                    let slug = get_slug_from_modal_container();
                                    download_png(&slug);
                                }
                            />

                            <MenuItem
                                class=controls_menu_item_class()
                                text=download_jpg_msg
                                icon=Signal::derive(move || TbJpg)
                                on:click=move |_| {
                                    let slug = get_slug_from_modal_container();
                                    download_jpg(&slug);
                                }
                            />

                            <MenuItem
                                class=controls_menu_item_class()
                                text=download_colored_svg_msg
                                icon=Signal::derive(move || TbSvg)
                                on:click=move |_| {
                                    let slug = get_slug_from_modal_container();
                                    let hex = get_hex_from_modal_container();
                                    spawn_local(async move {
                                        let svg = fetch_text(&format!("/icons/{}.svg", &slug))
                                            .await
                                            .unwrap_or_else(|| {
                                                panic!("Error fetching SVG /icons/{}.svg", &slug)
                                            });
                                        let colored_icon_svg = svg
                                            .replacen("<svg", &format!("<svg fill=\"{}\"", hex), 1);
                                        download(
                                            &format!("{}-color.svg", slug),
                                            &format!(
                                                "data:image/svg+xml;utf8,{}",
                                                js_sys::encode_uri_component(&colored_icon_svg),
                                            ),
                                        );
                                    });
                                }
                            />

                            <MenuItem
                                class=controls_menu_item_class()
                                text=copy_svg_msg
                                icon=copy_svg_icon
                                on:click=move |_| {
                                    let slug = get_slug_from_modal_container();
                                    set_copying_svg(true);
                                    spawn_local(async move {
                                        if let Some(svg) = fetch_text(
                                                &format!("/icons/{}.svg", slug),
                                            )
                                            .await
                                        {
                                            copy_and_set_copied_transition(
                                                svg,
                                                document()
                                                    .get_element_by_id(Ids::IconDetailsModal.as_str())
                                                    .unwrap()
                                                    .get_elements_by_tag_name("button")
                                                    .item(0)
                                                    .unwrap()
                                                    .dyn_into::<web_sys::HtmlElement>()
                                                    .unwrap(),
                                            );
                                        }
                                    });
                                    set_timeout(
                                        move || set_copying_svg(false),
                                        std::time::Duration::from_secs(1),
                                    );
                                }
                            />

                            <MenuItem
                                class=controls_menu_item_class()
                                text=copy_png_msg
                                icon=copy_png_icon
                                on:click=move |_| {
                                    let slug = get_slug_from_modal_container();
                                    set_copying_png(true);
                                    copy_as_image_png(&slug);
                                    set_timeout(
                                        move || set_copying_png(false),
                                        std::time::Duration::from_secs(1),
                                    );
                                }
                            />

                            <MenuItem
                                class=controls_menu_item_class()
                                text=copy_jpg_msg
                                icon=copy_jpg_icon
                                on:click=move |_| {
                                    let slug = get_slug_from_modal_container();
                                    set_copying_jpg(true);
                                    copy_as_image_jpg(&slug);
                                    set_timeout(
                                        move || set_copying_jpg(false),
                                        std::time::Duration::from_secs(1),
                                    );
                                }
                            />

                            <MenuItem
                                class=controls_menu_item_class()
                                text=copy_hex_msg
                                icon=copy_hex_icon
                                on:click=move |ev| {
                                    let hex = get_hex_from_modal_container();
                                    set_copying_hex(true);
                                    copy_and_set_copied_transition(
                                        hex,
                                        ev
                                            .target()
                                            .unwrap()
                                            .dyn_into::<web_sys::HtmlElement>()
                                            .unwrap(),
                                    );
                                    set_timeout(
                                        move || set_copying_hex(false),
                                        std::time::Duration::from_secs(1),
                                    );
                                }
                            />

                            <MenuItem
                                class=controls_menu_item_class()
                                text=copy_as_base64_svg_text
                                icon=copy_as_base64_svg_icon
                                on:click=move |ev| {
                                    if copying_as_base64_svg.get_untracked() {
                                        return;
                                    }
                                    set_copying_as_base64_svg(true);
                                    set_timeout(
                                        move || set_copying_as_base64_svg(false),
                                        std::time::Duration::from_secs(1),
                                    );
                                    let slug = get_slug_from_modal_container();
                                    spawn_local(async move {
                                        if let Some(svg) = fetch_text(
                                                &format!("/icons/{}.svg", slug),
                                            )
                                            .await
                                        {
                                            let base64 = window().btoa(&svg).unwrap();
                                            let base64_svg = format!(
                                                "data:image/svg+xml;base64,{}",
                                                base64,
                                            );
                                            copy_and_set_copied_transition(
                                                base64_svg,
                                                ev
                                                    .target()
                                                    .unwrap()
                                                    .dyn_into::<web_sys::HtmlElement>()
                                                    .unwrap(),
                                            );
                                        }
                                    });
                                }
                            />

                            <MenuItem
                                class=controls_menu_item_class()
                                text=copy_as_base64_jpg_text
                                icon=copy_as_base64_jpg_icon
                                on:click=move |_| {
                                    if copying_as_base64_jpg.get_untracked() {
                                        return;
                                    }
                                    set_copying_as_base64_jpg(true);
                                    set_timeout(
                                        move || set_copying_as_base64_jpg(false),
                                        std::time::Duration::from_secs(1),
                                    );
                                    let slug = get_slug_from_modal_container();
                                    copy_as_base64_jpg(&slug);
                                }
                            />

                            <MenuItem
                                class=controls_menu_item_class()
                                text=copy_as_base64_png_text
                                icon=copy_as_base64_png_icon
                                on:click=move |_| {
                                    if copying_as_base64_png.get_untracked() {
                                        return;
                                    }
                                    set_copying_as_base64_png(true);
                                    set_timeout(
                                        move || set_copying_as_base64_png(false),
                                        std::time::Duration::from_secs(1),
                                    );
                                    let slug = get_slug_from_modal_container();
                                    copy_as_base64_png(&slug);
                                }
                            />

                            <MenuItem
                                class=controls_menu_item_class()
                                text=copy_brand_name_msg
                                icon=copy_brand_name_icon
                                on:click=move |ev| {
                                    let brand_name = get_brand_name_from_modal_container();
                                    set_copying_brand_name(true);
                                    copy_and_set_copied_transition(
                                        brand_name,
                                        ev
                                            .target()
                                            .unwrap()
                                            .dyn_into::<web_sys::HtmlElement>()
                                            .unwrap(),
                                    );
                                    set_timeout(
                                        move || set_copying_brand_name(false),
                                        std::time::Duration::from_secs(1),
                                    );
                                }
                            />

                            <MenuItem
                                class=controls_menu_item_class()
                                text=copy_icon_modal_url_msg
                                icon=copy_icon_modal_url_icon
                                on:click=move |ev| {
                                    if copying_icon_modal_url.get_untracked() {
                                        return;
                                    }
                                    set_copying_icon_modal_url(true);
                                    let current_url = window().location().href().unwrap();
                                    let current_url_split = current_url
                                        .split("/")
                                        .collect::<Vec<&str>>();
                                    let url = format!(
                                        "{}//{}/?modal=icon&q={}",
                                        current_url_split[0],
                                        current_url_split[2],
                                        get_slug_from_modal_container(),
                                    );
                                    copy_and_set_copied_transition(
                                        url,
                                        ev
                                            .target()
                                            .unwrap()
                                            .dyn_into::<web_sys::HtmlElement>()
                                            .unwrap(),
                                    );
                                    set_timeout(
                                        move || set_copying_icon_modal_url(false),
                                        std::time::Duration::from_secs(1),
                                    );
                                }
                            />

                        </Menu>
                    </Show>
                </div>
            </div>
        </Modal>
    }
}
