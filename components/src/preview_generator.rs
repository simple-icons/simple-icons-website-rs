#![allow(clippy::redundant_closure)]

use crate::button::Button;
use crate::controls::download::download;
use crate::controls::search::fuzzy::search;
use crate::fetch::fetch_text;
use crate::grid::ICONS;
use crate::svg_def::SVGDef;
use i18n::{move_tr, tr};
use leptos::*;
use simple_icons::{color, sdk};
use simple_icons_macros::{get_number_of_icons, simple_icon_svg_path};
use std::collections::HashMap;
use types::SimpleIcon;
use wasm_bindgen::{closure::Closure, JsCast};
use wasm_bindgen_futures;

/// Initial brand when the preview is loaded
fn initial_brand_value() -> String {
    "Simple Icons".to_string()
}

/// Initial color when the preview is loaded
fn initial_color() -> String {
    "111111".to_string()
}

/// Initial SVG path when the preview is loaded
fn initial_path() -> String {
    simple_icon_svg_path!("simpleicons").to_string()
}

/// Check if a string is a valid hex color
fn is_valid_hex_color(value: &str) -> bool {
    if value.len() != 6 && value.len() != 3 {
        return false;
    }

    for c in value.chars() {
        if !c.is_ascii_hexdigit() {
            return false;
        }
    }
    true
}

/// Get the URL of a badge
fn badge_url(color: &str, svg: &str, style: &str) -> String {
    format!(
        concat!(
            "https://img.shields.io/badge/{}-preview-{}.svg",
            "?style={}&logo=data:image/svg%2bxml;base64,{}",
        ),
        match style {
            "social" => "",
            _ => "simple%20icons",
        },
        color,
        style,
        window().btoa(svg).unwrap(),
    )
}

/// Get the contrast color for a given hex color
fn contrast_color_for(hex: &str) -> String {
    if !is_valid_hex_color(hex) {
        return "black".to_string();
    }
    let is_light_hex =
        color::is_relatively_light_icon_hex(&sdk::normalize_color(hex));
    if is_light_hex { "black" } else { "white" }.to_string()
}

/// Build a SVG string with the 24px24 viewBox and an optional `fill` attribute
fn build_svg(path: &str, fill: Option<&str>) -> String {
    format!(
        "<svg role=\"img\" viewBox=\"0 0 24 24\" xmlns=\"http://www.w3.org/2000/svg\"><path d=\"{}\"{}/></svg>",
        path,
        match fill {
            Some(fill) => format!(" fill=\"#{}\"", fill),
            None => "".to_string(),
        }
    )
}

fn get_canvas_container() -> web_sys::HtmlCanvasElement {
    document()
        .get_elements_by_class_name("preview-figure")
        .item(0)
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap()
        .get_elements_by_tag_name("canvas")
        .item(0)
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap()
}

fn get_preview_canvas_context() -> web_sys::CanvasRenderingContext2d {
    let canvas = get_canvas_container();
    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    ctx.set_font("1rem sans");
    ctx
}

fn create_badge_image_for_canvas(
    badge_index: usize,
    badge_url: &str,
    x: f64,
    y: f64,
) {
    let badge_img_for_canvas = document()
        .create_element("img")
        .unwrap()
        .dyn_into::<web_sys::HtmlImageElement>()
        .unwrap();
    badge_img_for_canvas
        .set_attribute("style", "display: none")
        .unwrap();
    badge_img_for_canvas
        .set_attribute(
            "id",
            &format!("preview-badge-image-for-canvas-{}", &badge_index),
        )
        .unwrap();
    badge_img_for_canvas.set_cross_origin(Some("anonymous"));

    document()
        .body()
        .unwrap()
        .append_child(&badge_img_for_canvas)
        .unwrap();

    let closure: Closure<dyn FnMut()> = Closure::new(move || {
        let img = document()
            .get_element_by_id(&format!(
                "preview-badge-image-for-canvas-{}",
                &badge_index
            ))
            .unwrap()
            .dyn_into::<web_sys::HtmlImageElement>()
            .unwrap();

        let ctx = get_preview_canvas_context();
        ctx.draw_image_with_html_image_element(&img, x, 420.0 + y)
            .unwrap();
        document().body().unwrap().remove_child(&img).unwrap();
    });
    badge_img_for_canvas.set_onload(Some(closure.as_ref().unchecked_ref()));
    closure.forget();

    badge_img_for_canvas
        .set_attribute("src", badge_url)
        .unwrap();
}

macro_rules! draw_badge_impl {
    ($badge_index:literal, $x:literal, $y:literal) => {{
        let badges_containers = document()
            .get_elements_by_class_name("preview-badges")
            .item(0)
            .unwrap()
            .children();
        let badge_img = badges_containers
            .item($badge_index)
            .unwrap()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap()
            .first_element_child()
            .unwrap()
            .dyn_into::<web_sys::HtmlImageElement>()
            .unwrap();

        create_badge_image_for_canvas(
            $badge_index,
            badge_img.src().as_str(),
            $x as f64,
            $y as f64,
        )
    }};
}

/// Draw the current badges in the canvas
fn update_badges_in_canvas() {
    draw_badge_impl!(0, 15, 15);
    draw_badge_impl!(1, 203, 16);
    draw_badge_impl!(2, 385, 6);
    draw_badge_impl!(3, 630, 14);

    draw_badge_impl!(4, 15, 41);
    draw_badge_impl!(5, 203, 41);
    draw_badge_impl!(6, 385, 39);
    draw_badge_impl!(7, 630, 40);
}

/// Function triggered to update the canvas with the current SVG
fn update_preview_canvas() {
    let container = document()
        .get_elements_by_class_name("preview-figure")
        .item(0);
    if container.is_none() {
        return;
    }

    let figure = document()
        .get_elements_by_class_name("preview-figure")
        .item(0)
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();
    let canvas = figure
        .get_elements_by_tag_name("canvas")
        .item(0)
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();
    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    ctx.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());

    // Draw the SVG of the preview card in the canvas
    let preview_card_svg =
        figure.get_elements_by_tag_name("svg").item(0).unwrap();
    let preview_card_img = document()
        .create_element("img")
        .unwrap()
        .dyn_into::<web_sys::HtmlImageElement>()
        .unwrap();
    preview_card_img
        .set_attribute("style", "display: none")
        .unwrap();
    preview_card_img
        .set_attribute("id", "preview-card-image-for-canvas")
        .unwrap();
    preview_card_img.set_cross_origin(Some("anonymous"));
    document()
        .body()
        .unwrap()
        .append_child(&preview_card_img)
        .unwrap();

    // Set the onload attribute and draw the image
    let closure: Closure<dyn FnMut()> = Closure::new(move || {
        let preview_card_img = document()
            .get_element_by_id("preview-card-image-for-canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlImageElement>()
            .unwrap();
        ctx.draw_image_with_html_image_element(&preview_card_img, 0.0, 0.0)
            .unwrap();
        document()
            .body()
            .unwrap()
            .remove_child(&preview_card_img)
            .unwrap();

        update_badges_in_canvas();
    });
    preview_card_img.set_onload(Some(closure.as_ref().unchecked_ref()));
    closure.forget();

    let preview_card_url = format!(
        "data:image/svg+xml;utf8,{}",
        js_sys::encode_uri_component(&preview_card_svg.outer_html())
    );
    preview_card_img
        .set_attribute("src", preview_card_url.as_str())
        .unwrap();
}

fn search_brand_suggestions(
    value: &str,
) -> (Vec<&'static SimpleIcon>, Vec<&'static SimpleIcon>) {
    let mut icons: Vec<&'static SimpleIcon> = Vec::with_capacity(6);
    let mut more_icons: Vec<&'static SimpleIcon> = Vec::new();
    let search_result = js_sys::Array::from(&search(value));
    let search_result_length = search_result.length();
    for i in 0..search_result_length {
        let result_icon_array = js_sys::Array::from(&search_result.get(i));
        let icon_order_alpha = result_icon_array.get(1).as_f64().unwrap();
        if i > 5 {
            more_icons.push(&ICONS[icon_order_alpha as usize]);
        } else {
            icons.push(&ICONS[icon_order_alpha as usize]);
        }
    }
    (icons, more_icons)
}

/// Preview generator
#[component]
pub fn PreviewGenerator() -> impl IntoView {
    let (brand, set_brand) = create_signal(initial_brand_value());
    let (color, set_color) = create_signal(initial_color());
    let (path, set_path) = create_signal(initial_path());

    view! {
        <div class="preview">
            <div>
                <BrandInput brand=brand set_brand=set_brand set_color=set_color set_path=set_path/>
                <ColorInput color=color set_color=set_color/>
            </div>
            <PathInput path=path set_path=set_path/>

            <PreviewFigure brand=brand color=color path=path/>
            <PreviewBadges color=color path=path/>
            <PreviewButtons
                brand=brand
                path=path
                set_brand=set_brand
                set_color=set_color
                set_path=set_path
            />
        </div>
    }
}

#[component]
fn PathInput(
    path: ReadSignal<String>,
    set_path: WriteSignal<String>,
) -> impl IntoView
where {
    view! {
        <div class="preview-input-group">
            <label for="preview-path">{move_tr!("path")}</label>
            <input
                type="text"
                style="width:682px"
                name="preview-path"
                value=path
                prop:value=path
                on:input=move |ev| {
                    let target = ev
                        .target()
                        .unwrap()
                        .dyn_into::<web_sys::HtmlInputElement>()
                        .unwrap();
                    set_path(target.value());
                    update_preview_canvas();
                }
            />

        </div>
    }
}

#[component]
fn ColorInput(
    color: ReadSignal<String>,
    set_color: WriteSignal<String>,
) -> impl IntoView {
    view! {
        <div class="preview-input-group">
            <label for="preview-color">{move_tr!("color")}</label>
            <input
                type="text"
                style="width:68px"
                name="preview-color"
                value=color
                prop:value=color
                on:input=move |ev| {
                    let input = ev
                        .target()
                        .unwrap()
                        .dyn_into::<web_sys::HtmlInputElement>()
                        .unwrap();
                    let selection_start = input.selection_start().unwrap();
                    let selection_end = input.selection_end().unwrap();
                    let normalized_value = input.value().to_uppercase().replace('#', "");
                    input.set_value(&normalized_value);
                    input.set_selection_start(selection_start).unwrap();
                    input.set_selection_end(selection_end).unwrap();
                    set_color(normalized_value);
                    update_preview_canvas();
                }

                class:invalid=move || !is_valid_hex_color(&color())
                maxlength=6
            />
        </div>
    }
}

#[component]
fn BrandInput(
    brand: ReadSignal<String>,
    set_brand: WriteSignal<String>,
    set_color: WriteSignal<String>,
    set_path: WriteSignal<String>,
) -> impl IntoView {
    let (brand_suggestions, set_brand_suggestions) =
        create_signal(Vec::<&SimpleIcon>::with_capacity(6));
    let (more_brand_suggestions, set_more_brand_suggestions) =
        create_signal(Vec::<&SimpleIcon>::new());
    let (show_brand_suggestions, set_show_brand_suggestions) =
        create_signal(false);
    let (show_more_brand_suggestions, set_show_more_brand_suggestions) =
        create_signal(false);

    view! {
        <div class="preview-input-group">
            <label for="preview-brand">{move_tr!("brand")}</label>
            <input
                type="text"
                class="mr-7"
                style="width:524px"
                name="preview-brand"
                value=brand
                prop:value=brand
                on:input=move |ev| {
                    let input = ev
                        .target()
                        .unwrap()
                        .dyn_into::<web_sys::HtmlInputElement>()
                        .unwrap();
                    let value = input.value();
                    let (bs, more_bs) = search_brand_suggestions(&value);
                    let more_bs_length = more_bs.len();
                    set_brand(value.clone());
                    update_preview_canvas();
                    set_brand_suggestions(bs);
                    set_more_brand_suggestions(more_bs);
                    set_show_brand_suggestions(true);
                    if value.len() < 4 || more_bs_length == 0 {
                        set_show_more_brand_suggestions(false);
                    }
                }

                on:focus=move |ev| {
                    let input = ev
                        .target()
                        .unwrap()
                        .dyn_into::<web_sys::HtmlInputElement>()
                        .unwrap();
                    let value = input.value();
                    let (bs, more_bs) = search_brand_suggestions(&value);
                    set_brand_suggestions(bs);
                    set_more_brand_suggestions(more_bs);
                    set_show_brand_suggestions(true);
                }
            />

            <BrandSuggestions
                show_brand_suggestions=show_brand_suggestions
                show_more_brand_suggestions=show_more_brand_suggestions
                brand_suggestions=brand_suggestions
                more_brand_suggestions=more_brand_suggestions
                set_brand=set_brand
                set_color=set_color
                set_path=set_path
                set_show_brand_suggestions=set_show_brand_suggestions
                set_show_more_brand_suggestions=set_show_more_brand_suggestions
            />
        </div>
    }
}

#[component]
fn BrandSuggestions(
    show_brand_suggestions: ReadSignal<bool>,
    show_more_brand_suggestions: ReadSignal<bool>,
    brand_suggestions: ReadSignal<Vec<&'static SimpleIcon>>,
    more_brand_suggestions: ReadSignal<Vec<&'static SimpleIcon>>,
    set_brand: WriteSignal<String>,
    set_color: WriteSignal<String>,
    set_path: WriteSignal<String>,
    set_show_brand_suggestions: WriteSignal<bool>,
    set_show_more_brand_suggestions: WriteSignal<bool>,
) -> impl IntoView {
    let body = document().body().unwrap();
    let closure: Closure<dyn FnMut(web_sys::MouseEvent)> =
        Closure::new(move |ev: web_sys::MouseEvent| {
            let target = ev
                .target()
                .unwrap()
                .dyn_into::<web_sys::HtmlElement>()
                .unwrap();
            // Hide the brand suggestions when the user clicks outside the input
            if target.get_attribute("name").unwrap_or("".to_string())
                == "preview-brand"
            {
                return;
            }
            if target.get_attribute("class").unwrap_or("".to_string())
                == "more-suggestions"
            {
                return;
            }
            set_show_brand_suggestions(false);
            set_show_more_brand_suggestions(false);
        });
    body.set_onclick(Some(closure.as_ref().unchecked_ref()));
    closure.forget();

    view! {
        <ul
            class=move || {
                let mut cls = "preview-brand-suggestions".to_string();
                if show_more_brand_suggestions() {
                    cls.push_str(" overflow-y-scroll");
                }
                cls
            }

            class:hidden=move || { !show_brand_suggestions() || brand_suggestions().is_empty() }
        >

            {move || {
                if !show_brand_suggestions() {
                    return vec![];
                }
                let mut suggestions_containers = vec![];
                let bs = brand_suggestions();
                for icon in bs {
                    suggestions_containers
                        .push(
                            view! {
                                <BrandSuggestion
                                    icon=icon
                                    set_brand=set_brand
                                    set_color=set_color
                                    set_path=set_path
                                />
                            },
                        );
                }
                if !show_more_brand_suggestions() {
                    if !more_brand_suggestions().is_empty() {
                        suggestions_containers
                            .push(
                                view! {
                                    <>
                                        <li
                                            class="more-suggestions"
                                            role="button"
                                            title=move_tr!("load-more-icons")
                                            on:click=move |_| {
                                                set_show_more_brand_suggestions(true);
                                            }
                                        >

                                            <span>+</span>
                                        </li>
                                    </>
                                }
                                    .into(),
                            );
                    }
                } else {
                    let more_bs = more_brand_suggestions();
                    for icon in more_bs {
                        suggestions_containers
                            .push(
                                view! {
                                    <BrandSuggestion
                                        icon=icon
                                        set_brand=set_brand
                                        set_color=set_color
                                        set_path=set_path
                                    />
                                },
                            );
                    }
                }
                suggestions_containers
            }}

        </ul>
    }
}

#[component]
fn BrandSuggestion(
    icon: &'static SimpleIcon,
    set_brand: WriteSignal<String>,
    set_color: WriteSignal<String>,
    set_path: WriteSignal<String>,
) -> impl IntoView {
    view! {
        <li on:click=move |_| {
            set_brand(icon.title.to_string());
            set_color(icon.hex.to_string());
            spawn_local(async move {
                if let Some(svg) = fetch_text(&format!("/icons/{}.svg", icon.slug)).await {
                    set_path(sdk::svg_to_path(&svg));
                }
                update_preview_canvas();
            });
        }>
            <a>
                <img src=format!("./icons/{}.svg", icon.slug) width="24px" height="24px"/>
                <span>{icon.title}</span>
            </a>
        </li>
    }
}

#[component]
fn PreviewFigure(
    brand: ReadSignal<String>,
    color: ReadSignal<String>,
    path: ReadSignal<String>,
) -> impl IntoView
where {
    let fill_color = create_memo(move |_| contrast_color_for(&color()));

    view! {
        <figure class="preview-figure">
            <svg
                width="740"
                height="420"
                viewBox="0 0 740 420"
                xmlns="http://www.w3.org/2000/svg"
                class="pt-3"
            >
                <rect
                    fill=move || format!("#{}", color())
                    height="420"
                    rx="10"
                    ry="10"
                    width="740"
                    x="0"
                    y="0"
                ></rect>
                <svg viewBox="0 0 24 24" width="24" height="24" x="18" y="20">
                    <path d=move || path() fill=fill_color></path>
                </svg>
                <svg viewBox="0 0 24 24" width="80" height="80" x="70" y="20">
                    <path d=move || path() fill=fill_color></path>
                </svg>
                <svg viewBox="0 0 24 24" width="138" height="138" x="174" y="20">
                    <path d=move || path() fill=fill_color></path>
                </svg>
                <svg viewBox="0 0 24 24" width="375" height="375" x="350" y="20">
                    <path d=move || path() fill=fill_color></path>
                </svg>

                <g transform="translate(21,235)" style="font-family: Helvetica">
                    <text fill=fill_color font-size="25">
                        {move || format!("{} Preview", brand())}
                    </text>
                    <text fill=fill_color font-size="17" y="25">
                        {move || format!("{}.svg", sdk::title_to_slug(&brand()))}
                    </text>
                    <text fill=fill_color font-size="16" y="61">
                        {move || format!("Brand: {}", brand())}
                    </text>
                    <text fill=fill_color font-size="16" y="84">
                        {move || format!("Color: #{}", color())}
                    </text>

                    <g transform="translate(3, 142)" style="font-family: Helvetica">
                        <svg viewBox="0 0 24 24" width="24" height="24">
                            <path d=simple_icon_svg_path!("simpleicons") fill=fill_color></path>
                        </svg>
                        <text fill=fill_color x="30" y="7" font-size="12">
                            {format!("{} Free SVG brand icons", get_number_of_icons!())}
                        </text>
                        <text fill=fill_color x="30" y="25" font-size="12">
                            available at simpleicons.org
                        </text>
                    </g>
                </g>
            </svg>
            <canvas height="490" width="740"></canvas>
        </figure>
    }
}

#[component]
fn PreviewBadges(
    color: ReadSignal<String>,
    path: ReadSignal<String>,
) -> impl IntoView
where {
    let white_svg = create_memo(move |_| build_svg(&path(), Some("FFF")));
    let color_svg = create_memo(move |_| build_svg(&path(), Some(&color())));

    view! {
        <div class="preview-badges">
            <PreviewBadge color=color svg=white_svg style="flat"/>
            <PreviewBadge color=color svg=white_svg style="plastic"/>
            <PreviewBadge color=color svg=white_svg style="for-the-badge"/>
            <PreviewBadge color=color svg=color_svg style="social"/>
            <PreviewBadge color=color svg=color_svg style="flat"/>
            <PreviewBadge color=color svg=color_svg style="plastic"/>
            <PreviewBadge color=color svg=color_svg style="for-the-badge"/>
            <PreviewBadge color=color svg=color_svg style="social" text_color="4183c4"/>
        </div>
    }
}

#[component]
fn PreviewBadge(
    color: ReadSignal<String>,
    svg: Memo<String>,
    style: &'static str,
    #[prop(optional)] text_color: Option<&'static str>,
) -> impl IntoView {
    let on_load = move |ev: web_sys::Event| {
        let target = ev
            .target()
            .unwrap()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap();

        if target.get_attribute("reloaded") == Some("true".to_string()) {
            target.set_attribute("reloaded", "false").unwrap();
            return;
        }
        if text_color.is_some() {
            spawn_local(async move {
                let url = badge_url(&color(), &svg(), style);
                let badge_svg = fetch_text(&url).await.unwrap();
                let styled_badge_svg = badge_svg.replace(
                    "id=\"rlink\"",
                    &format!("id=\"rlink\" fill=\"#{}\"", &text_color.unwrap()),
                );
                let encoded_svg =
                    js_sys::encode_uri_component(&styled_badge_svg);
                if encoded_svg == target.get_attribute("src").unwrap() {
                    target.set_attribute("reloaded", "false").unwrap();
                    return;
                }

                target.set_attribute("reloaded", "true").unwrap();
                target
                    .set_attribute(
                        "src",
                        &format!("data:image/svg+xml;utf8,{}", encoded_svg),
                    )
                    .unwrap();

                update_preview_canvas();
            });
        } else {
            update_preview_canvas();
        }
    };

    view! {
        <div>
            <img src=move || badge_url(&color(), &svg(), style) on:load=on_load/>
        </div>
    }
}

#[component]
fn PreviewButtons(
    brand: ReadSignal<String>,
    path: ReadSignal<String>,
    set_brand: WriteSignal<String>,
    set_color: WriteSignal<String>,
    set_path: WriteSignal<String>,
) -> impl IntoView {
    async fn on_upload_svg_file(
        file: web_sys::File,
        set_brand: WriteSignal<String>,
        set_color: WriteSignal<String>,
        set_path: WriteSignal<String>,
    ) {
        match wasm_bindgen_futures::JsFuture::from(file.text()).await {
            Ok(text) => {
                let value = text.as_string().unwrap();

                // Set color
                if value.contains("fill=\"") {
                    let hex = sdk::normalize_color(
                        value
                            .split("fill=\"")
                            .nth(1)
                            .unwrap()
                            .split('"')
                            .next()
                            .unwrap(),
                    );
                    if is_valid_hex_color(&hex) {
                        set_color(hex.to_string());
                    }
                }

                // Set brand
                if value.contains("<title>") && value.contains("</title>") {
                    let brand = value
                        .split("<title>")
                        .nth(1)
                        .unwrap()
                        .split("</title>")
                        .next()
                        .unwrap();
                    set_brand(brand.to_string());
                }

                // Set path
                if value.contains(" d=\"") {
                    let path = value
                        .split(" d=\"")
                        .nth(1)
                        .unwrap()
                        .split('"')
                        .next()
                        .unwrap();
                    set_path(path.to_string());
                }
            }
            Err(err) => {
                ::log::error!("Error reading uploaded SVG file: {:?}", err)
            }
        }
    }

    view! {
        <div class="preview-buttons">
            <form class="inline-block">
                <input
                    type="file"
                    name="upload-svg"
                    accept=".svg"
                    class="absolute w-0 h-0 -z-index-1"
                    on:change=move |ev| {
                        let input = ev
                            .target()
                            .unwrap()
                            .dyn_into::<web_sys::HtmlInputElement>()
                            .unwrap();
                        let file = input.files().unwrap().get(0).unwrap();
                        spawn_local(on_upload_svg_file(file, set_brand, set_color, set_path));
                    }
                />

                <Button
                    svg_path=&SVGDef::Upload
                    title=move_tr!("upload-svg")
                    on:click=move |ev| {
                        let input = document()
                            .query_selector("input[name='upload-svg']")
                            .unwrap()
                            .unwrap()
                            .dyn_into::<web_sys::HtmlElement>()
                            .unwrap();
                        input.click();
                        ev.target()
                            .unwrap()
                            .dyn_into::<web_sys::HtmlElement>()
                            .unwrap()
                            .blur()
                            .unwrap();
                    }
                />

            </form>
            <Button
                svg_path=&SVGDef::Save
                title=move_tr!("save-preview")
                class="float-right ml-4"
                on:click=move |ev: web_sys::MouseEvent| {
                    let canvas = get_canvas_container();
                    let filename = format!("{}.png", &sdk::title_to_slug(&brand()));
                    let url = canvas.to_data_url().unwrap();
                    download(&filename, &url);
                    ev.target()
                        .unwrap()
                        .dyn_into::<web_sys::HtmlElement>()
                        .unwrap()
                        .blur()
                        .unwrap();
                }
            />

            <Button
                svg_path=&SVGDef::Download
                title=move_tr!(
                    "download-filetype", & { let mut map = HashMap::new(); map.insert("filetype"
                    .to_string(), tr!("svg") .into()); map }
                )

                class="float-right"
                on:click=move |ev: web_sys::MouseEvent| {
                    let filename = format!("{}.svg", &sdk::title_to_slug(&brand()));
                    let url = format!(
                        "data:image/svg+xml;utf8,{}",
                        js_sys::encode_uri_component(&build_svg(&path(), None)),
                    );
                    download(&filename, &url);
                    ev.target()
                        .unwrap()
                        .dyn_into::<web_sys::HtmlElement>()
                        .unwrap()
                        .blur()
                        .unwrap();
                }
            />

        </div>
    }
}
