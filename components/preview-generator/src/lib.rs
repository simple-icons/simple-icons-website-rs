mod buttons;
mod canvas;
mod deps;
mod helpers;
mod inputs;
mod keyboard;

use badge_maker::make_badge;
use buttons::PreviewButtons;
use canvas::update_preview_canvas;
pub use deps::add_preview_generator_scripts;
use fast_fuzzy::search;
use helpers::contrast_color_for;
use helpers::is_valid_hex_color;
use inputs::{BrandInput, ColorInput, PathInput};
use leptos::{prelude::*, task::spawn_local};
use leptos_use::use_device_pixel_ratio;
use simple_icons_macros::{get_number_of_icons, get_simple_icon_svg_path};
use simple_icons_sdk as sdk;
use simple_icons_website_grid_constants::ICONS;
use simple_icons_website_svg_icon::svg_with_path_opt_fill;
use simple_icons_website_types::SimpleIcon;
use simple_icons_website_url as Url;
use web_sys_simple_fetch::fetch_text;

static DEFAULT_INITIAL_BRAND: &str = "Simple Icons";
static DEFAULT_INITIAL_SLUG: &str = "simpleicons";
static DEFAULT_INITIAL_COLOR: &str = "111111";
static DEFAULT_INITIAL_PATH: &str = get_simple_icon_svg_path!("simpleicons");

pub(crate) type Brand = (String, String); // (title, slug)

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

fn default_icon() -> (Brand, String, String, Option<&'static SimpleIcon>) {
    (
        (
            DEFAULT_INITIAL_BRAND.to_string(),
            DEFAULT_INITIAL_SLUG.to_string(),
        ),
        DEFAULT_INITIAL_COLOR.to_string(),
        DEFAULT_INITIAL_PATH.to_string(),
        None,
    )
}

fn initial_icon() -> (Brand, String, String, Option<&'static SimpleIcon>) {
    match Url::params::get(&Url::params::Names::Query) {
        Some(value) => {
            if value.is_empty() {
                return default_icon();
            }
            match search_brand(&value) {
                Some(icon) => (
                    (icon.title.to_string(), icon.slug.to_string()),
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
    let (initial_brand, initial_color, initial_path, initial_icon) =
        initial_icon();
    let brand = RwSignal::new(initial_brand);
    let (color, set_color) = signal(initial_color);
    let (path, set_path) = signal(initial_path.clone());

    provide_context::<RwSignal<Brand>>(brand);

    if path.get_untracked().is_empty() {
        spawn_local(async move {
            match fetch_text(&format!(
                "/icons/{}.svg",
                initial_icon.unwrap().slug
            ))
            .await
            {
                Ok(svg) => set_path(sdk::svg_to_path(&svg)),
                Err(_) => set_path(initial_path.clone()),
            }
        });
    }

    let pixel_ratio = use_device_pixel_ratio();
    Effect::new(move |_| update_preview_canvas(pixel_ratio()));

    keyboard::listen_keyboard_shortcuts();

    view! {
        <div class="preview">
            <div>
                <BrandInput set_color />
                <ColorInput color set_color />
            </div>
            <PathInput path set_path />

            <PreviewFigure color path />
            <PreviewButtons path set_color set_path />
        </div>
    }
}

#[component]
fn PreviewFigure(
    color: ReadSignal<String>,
    path: ReadSignal<String>,
) -> impl IntoView {
    let fill_color = Memo::new(move |_| {
        let color = color();
        if !is_valid_hex_color(&color) {
            contrast_color_for("000000")
        } else {
            contrast_color_for(&color)
        }
    });
    let color_or_error_color = Memo::new(move |_| {
        let color = color();
        if !is_valid_hex_color(&color) {
            "CC0000".to_string()
        } else {
            color
        }
    });
    let brand = expect_context::<RwSignal<Brand>>();

    let (width, height) = (canvas::WIDTH, canvas::HEIGHT);
    const MAX_LENGTH: usize = 32;

    view! {
        <figure class="preview-figure">
            <svg
                width=width
                height=height
                viewBox=format!("0 0 {width} {height}")
                xmlns="http://www.w3.org/2000/svg"
                class="pt-3"
            >
                <rect
                    fill=move || format!("#{}", color())
                    height=height - 100
                    rx="10"
                    ry="10"
                    width=width
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
                        let title = brand().0.clone();
                        let preview_title = truncate(&format!("{title} Preview"), MAX_LENGTH);

                        view! {
                            <text fill=fill_color font-size="25">
                                {preview_title}
                            </text>
                        }
                            .into_any()
                    }}

                    <text fill=fill_color font-size="17" y="25">
                        {move || {
                            format!("{}.svg", truncate(&sdk::title_to_slug(&brand().0), MAX_LENGTH))
                        }}
                    </text>
                    <text fill=fill_color font-size="16" y="61">
                        {move || format!("Brand: {}", truncate(&brand().0, MAX_LENGTH))}
                    </text>
                    <text fill=fill_color font-size="16" y="84">
                        {move || format!("Color: #{}", color())}
                    </text>

                    <g transform="translate(3, 142)" style="font-family: Helvetica">
                        <svg viewBox="0 0 24 24" width="24" height="24">
                            <path d=DEFAULT_INITIAL_PATH.to_string() fill=fill_color></path>
                        </svg>
                        <text fill=fill_color x="30" y="7" font-size="12">
                            {format!("{} SVG brand icons", get_number_of_icons!())}
                        </text>
                        <text fill=fill_color x="30" y="25" font-size="12">
                            available at simpleicons.org
                        </text>
                    </g>
                </g>
                <PreviewBadges color=color_or_error_color.into() path=path.into() />
            </svg>
            <canvas width=width height=height></canvas>
        </figure>
    }
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        let mut truncated = s[..max_len].to_string();
        truncated.push('…');
        truncated
    }
}

#[component]
fn PreviewBadges(color: Signal<String>, path: Signal<String>) -> impl IntoView {
    let pixel_ratio = use_device_pixel_ratio();

    let white_svg = Signal::derive(move || {
        svg_with_path_opt_fill(
            &path(),
            if is_valid_hex_color(&color()) {
                Some("FFFFFF".to_string())
            } else {
                Some("CC0000".to_string())
            },
        )
    });
    let color_svg = Signal::derive(move || {
        svg_with_path_opt_fill(
            &path(),
            if is_valid_hex_color(&color()) {
                Some(color().to_string())
            } else {
                Some("CC0000".to_string())
            },
        )
    });

    let badge_maker_loaded = RwSignal::new(false);

    let badge_maker_loaded_interval = set_interval_with_handle(
        move || {
            if deps::is_badge_maker_loaded() {
                badge_maker_loaded(true);

                // Execute at the next re-paint
                set_timeout(
                    move || {
                        update_preview_canvas(
                            use_device_pixel_ratio().get_untracked(),
                        );
                    },
                    std::time::Duration::from_millis(0),
                );
            }
        },
        std::time::Duration::from_millis(100),
    )
    .unwrap();

    Effect::new(move |_| {
        // Update canvas after changing the path or color
        let _path = path();
        let _color = color();
        set_timeout(
            move || {
                update_preview_canvas(pixel_ratio.get_untracked());
            },
            std::time::Duration::from_millis(0),
        );
    });

    let translate_y = 34;

    view! {
        <g transform="translate(10,437) scale(1.03)">
            {move || {
                match badge_maker_loaded() {
                    false => {
                        view! {
                            <text fill="white" transform="translate(310,35) scale(1.5)">
                                ...
                            </text>
                        }
                            .into_any()
                    }
                    true => {
                        badge_maker_loaded_interval.clear();
                        view! {
                            <PreviewBadge
                                color
                                svg=white_svg
                                style="flat"
                                translate_x=10
                                translate_y=3
                                id="b1"
                            />
                            <PreviewBadge
                                color
                                svg=color_svg
                                style="flat"
                                translate_x=10
                                translate_y=translate_y
                                id="b2"
                            />
                            <PreviewBadge
                                color
                                svg=white_svg
                                style="plastic"
                                translate_x=188
                                translate_y=7
                                id="b3"
                            />
                            <PreviewBadge
                                color
                                svg=color_svg
                                style="plastic"
                                translate_x=188
                                translate_y=translate_y + 1
                                id="b4"
                            />
                            <PreviewBadge
                                color
                                svg=white_svg
                                style="for-the-badge"
                                translate_x=365
                                translate_y=-3
                                id="b5"
                            />
                            <PreviewBadge
                                color
                                svg=color_svg
                                style="for-the-badge"
                                translate_x=365
                                translate_y=translate_y - 3
                                id="b6"
                            />
                            <PreviewBadge
                                color
                                svg=color_svg
                                style="social"
                                translate_x=610
                                translate_y=3
                                id="b7"
                            />
                            <PreviewBadge
                                color
                                svg=color_svg
                                style="social"
                                text_color="4183c4"
                                translate_x=610
                                translate_y=translate_y - 2
                                id="b8"
                            />
                        }
                            .into_any()
                    }
                }
            }}
        </g>
    }
}

#[component]
fn PreviewBadge(
    color: Signal<String>,
    svg: Signal<String>,
    style: &'static str,
    #[prop(optional)] text_color: Option<&'static str>,
    translate_x: usize,
    translate_y: isize,
    id: &'static str,
) -> impl IntoView {
    fn badge_svg(color_: &str, svg_: &str, style_: &str, id: &str) -> String {
        make_badge(
            match style_ {
                "social" => "",
                _ => "simple icons",
            },
            "preview",
            color_,
            style_,
            &format!(
                "data:image/svg+xml;base64,{}",
                window().btoa(svg_).unwrap()
            ),
        )
        // Internal identifiers for gradients must be unique in the parent SVG document
        .replace(r#"id="r""#, format!(r#"id="{id}-r""#).as_str())
        .replace("url(#r)", format!("url(#{id}-r)").as_str())
        .replace(r#"id="s""#, format!(r#"id="{id}-s""#).as_str())
        .replace("url(#s)", format!("url(#{id}-s)").as_str())
        .replace(r#"id="a""#, format!(r#"id="{id}-a""#).as_str())
        .replace("url(#a)", format!("url(#{id}-a)").as_str())
        .replace(r#"id="b""#, format!(r#"id="{id}-b""#).as_str())
        .replace("url(#b)", format!("url(#{id}-b)").as_str())
    }

    view! {
        <g
            transform=format!("translate({translate_x},{translate_y})")
            inner_html=move || {
                let mut badge_svg = badge_svg(&color(), &svg(), style, id);
                if let Some(text_color) = text_color {
                    badge_svg = badge_svg
                        .replace(
                            "text id=\"rlink\"",
                            &format!("text id=\"rlink\" fill=\"#{}\"", &text_color),
                        );
                }
                badge_svg
            }
        />
    }
}
