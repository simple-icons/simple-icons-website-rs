mod buttons;
mod canvas;
pub(crate) mod helpers;
mod inputs;

use crate::controls::search::fuzzy::search;
use crate::fetch::fetch_text;
use crate::grid::ICONS;
use crate::svg::svg_with_path_opt_fill;
use crate::Url;
use buttons::PreviewButtons;
use canvas::update_preview_canvas;
use helpers::contrast_color_for;
use icondata::SiSimpleicons;
use inputs::{BrandInput, ColorInput, PathInput};
use lazy_static::lazy_static;
use leptos::*;
use leptos_use::use_device_pixel_ratio;
use simple_icons::sdk;
use simple_icons_macros::get_number_of_icons;
use types::SimpleIcon;

static DEFAULT_INITIAL_BRAND: &str = "Simple Icons";
static DEFAULT_INITIAL_COLOR: &str = "111111";
lazy_static! {
    static ref DEFAULT_INITIAL_PATH: String =
        sdk::svg_to_path(SiSimpleicons.data);
}

fn search_brand(value: &str) -> Option<&'static SimpleIcon> {
    let search_result = js_sys::Array::from(&search(value));
    let search_result_length = search_result.length();
    if search_result_length > 0 {
        let result_icon_array = js_sys::Array::from(&search_result.get(0));
        let icon_order_alpha = result_icon_array.get(1).as_f64().unwrap();
        return Some(&ICONS[icon_order_alpha as usize]);
    }
    None
}

fn default_icon() -> (String, String, String, Option<&'static SimpleIcon>) {
    (
        DEFAULT_INITIAL_BRAND.to_string(),
        DEFAULT_INITIAL_COLOR.to_string(),
        DEFAULT_INITIAL_PATH.to_string(),
        None,
    )
}

fn initial_icon() -> (String, String, String, Option<&'static SimpleIcon>) {
    match Url::params::get(&Url::params::Names::Query) {
        Some(value) => {
            if value.is_empty() {
                return default_icon();
            }
            match search_brand(&value) {
                Some(icon) => (
                    icon.title.to_string(),
                    icon.hex.to_string(),
                    "".to_string(),
                    Some(icon),
                ),
                None => default_icon(),
            }
        }
        None => default_icon(),
    }
}

/// Preview generator
#[component]
pub fn PreviewGenerator() -> impl IntoView {
    let (initial_brand, initial_color, initial_path, icon) = initial_icon();
    let (brand, set_brand) = create_signal(initial_brand);
    let (color, set_color) = create_signal(initial_color);
    let (path, set_path) = create_signal(initial_path);
    if path.get_untracked().is_empty() {
        spawn_local(async move {
            if let Some(svg) =
                fetch_text(&format!("/icons/{}.svg", icon.unwrap().slug)).await
            {
                set_path(sdk::svg_to_path(&svg));
            }
        });
    }

    let pixel_ratio = use_device_pixel_ratio();
    create_effect(move |_| update_preview_canvas(pixel_ratio()));

    view! {
        <div class="preview">
            <div>
                <BrandInput brand set_brand set_color />
                <ColorInput color set_color />
            </div>
            <PathInput path set_path />

            <PreviewFigure brand color path />
            <PreviewBadges color path />
            <PreviewButtons brand path set_brand set_color set_path />
        </div>
    }
}

#[component]
fn PreviewFigure(
    brand: ReadSignal<String>,
    color: ReadSignal<String>,
    path: ReadSignal<String>,
) -> impl IntoView {
    let fill_color = create_memo(move |_| contrast_color_for(&color()));

    view! {
        <figure class="preview-figure">
            <svg
                width=canvas::WIDTH
                height=canvas::HEIGHT - 70
                viewBox=format!("0 0 {} {}", canvas::WIDTH, canvas::HEIGHT - 70)
                xmlns="http://www.w3.org/2000/svg"
                class="pt-3"
            >
                <rect
                    fill=move || format!("#{}", color())
                    height=canvas::HEIGHT - 70
                    rx="10"
                    ry="10"
                    width=canvas::WIDTH
                    x="0"
                    y="0"
                ></rect>
                <svg viewBox="0 0 24 24" width="24" height="24" x="18" y="20">
                    <path d=path fill=fill_color></path>
                </svg>
                <svg viewBox="0 0 24 24" width="80" height="80" x="70" y="20">
                    <path d=path fill=fill_color></path>
                </svg>
                <svg viewBox="0 0 24 24" width="138" height="138" x="174" y="20">
                    <path d=path fill=fill_color></path>
                </svg>
                <svg viewBox="0 0 24 24" width="375" height="375" x="350" y="20">
                    <path d=path fill=fill_color></path>
                </svg>

                <g transform="translate(21,235)" style="font-family: Helvetica">
                    {move || {
                        let preview_title = format!("{} Preview", brand());
                        if preview_title.len() > 24 {
                            let mut title_1 = String::with_capacity(24);
                            let mut title_2 = String::with_capacity(24);
                            for part in preview_title.split(' ') {
                                if title_1.len() + part.len() < 24 {
                                    title_1.push_str(part);
                                    title_1.push(' ');
                                } else if title_2.len() + part.len() < 24 {
                                    title_2.push_str(part);
                                    title_2.push(' ');
                                } else {
                                    for ch in part.chars() {
                                        if title_2.len() + 1 < 24 {
                                            title_2.push(ch);
                                        } else {
                                            title_2.push('…');
                                            break;
                                        }
                                    }
                                    break;
                                }
                            }
                            view! {
                                <text fill=fill_color font-size="25" y="-31">
                                    {title_1}
                                </text>
                                <text fill=fill_color font-size="25">
                                    {title_2}
                                </text>
                            }
                                .into_view()
                        } else {
                            view! {
                                <text fill=fill_color font-size="25">
                                    {preview_title}
                                </text>
                            }
                                .into_view()
                        }
                    }}

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
                            <path d=DEFAULT_INITIAL_PATH.to_string() fill=fill_color></path>
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
            <canvas height=canvas::HEIGHT width=canvas::WIDTH></canvas>
        </figure>
    }
}

#[component]
fn PreviewBadges(
    color: ReadSignal<String>,
    path: ReadSignal<String>,
) -> impl IntoView {
    let white_svg =
        create_memo(move |_| svg_with_path_opt_fill(&path(), Some("FFF")));
    let color_svg =
        create_memo(move |_| svg_with_path_opt_fill(&path(), Some(&color())));

    view! {
        <div class="preview-badges">
            <PreviewBadge color svg=white_svg style="flat" />
            <PreviewBadge color svg=white_svg style="plastic" />
            <PreviewBadge color svg=white_svg style="for-the-badge" />
            <PreviewBadge color svg=color_svg style="social" />
            <PreviewBadge color svg=color_svg style="flat" />
            <PreviewBadge color svg=color_svg style="plastic" />
            <PreviewBadge color svg=color_svg style="for-the-badge" />
            <PreviewBadge color svg=color_svg style="social" text_color="4183c4" />
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
    let pixel_ratio = use_device_pixel_ratio();

    /// Get the URL of a badge
    fn badge_url(color_: &str, svg: &str, style: &str) -> String {
        format!(
            concat!(
                "https://img.shields.io/badge/{}-preview-{}.svg",
                "?style={}&logo=data:image/svg%2bxml;base64,{}",
            ),
            match style {
                "social" => "",
                _ => "simple%20icons",
            },
            color_,
            style,
            window().btoa(svg).unwrap(),
        )
    }

    let on_load = move |ev: web_sys::Event| {
        let target = event_target::<web_sys::HtmlInputElement>(&ev);

        if target.get_attribute("reloaded") == Some("true".into()) {
            target.set_attribute("reloaded", "false").unwrap();
            return;
        }
        if text_color.is_some() {
            spawn_local(async move {
                let url = badge_url(
                    &color.get_untracked(),
                    &svg.get_untracked(),
                    style,
                );
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

                update_preview_canvas(pixel_ratio.get_untracked());
            });
        } else {
            update_preview_canvas(pixel_ratio.get_untracked());
        }
    };

    view! {
        <div>
            <img src=move || badge_url(&color(), &svg(), style) on:load=on_load />
        </div>
    }
}
