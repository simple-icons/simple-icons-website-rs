use i18n::move_tr;
use leptos::{html::Input, *};
use macros::{get_number_of_icons, simple_icon_svg_path};
use simple_icons::{color, sdk::title_to_slug};

fn initial_brand_value() -> String {
    "Simple Icons".to_string()
}

fn initial_color() -> String {
    "111111".to_string()
}

fn initial_path() -> String {
    simple_icon_svg_path!("simpleicons").to_string()
}

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

fn badge_url(
    slug: &str,
    color: &str,
    svg: &str,
    style: &str,
    svg_color: &str,
) -> String {
    let mut colored_svg = String::from(svg);
    if !color::is_relatively_light_icon_hex(color) {
        let replacement = format!("<path fill=\"{}\" ", &svg_color);
        colored_svg = svg.replace("<path ", &replacement);
    }
    format!(
        "https://img.shields.io/badge/{}-preview-{}.svg?style={}&logo=data:image/svg%2bxml;base64,{}",
        slug,
        color,
        style,
        window().btoa(&colored_svg).unwrap(),
    )
}

#[component]
fn PreviewBadge<S, C, G, V>(
    slug: S,
    color: C,
    svg: G,
    style: &'static str,
    svg_color: V,
) -> impl IntoView
where
    S: Fn() -> String + 'static,
    C: Fn() -> String + 'static,
    V: Fn() -> String + 'static,
    G: Fn() -> String + 'static,
{
    let url = badge_url(&slug(), &color(), &svg(), style, &svg_color());
    view! {
        <div>
            <img src=url/>
        </div>
    }
}

enum PreviewButtonSvgPath {
    Upload,
}

impl PreviewButtonSvgPath {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Upload => "M9,16V10H5L12,3L19,10H15V16H9M5,20V18H19V20H5",
        }
    }
}

fn update_canvas() {}

#[component]
fn PreviewButton(
    svg_path: PreviewButtonSvgPath,
    title: &'static str,
) -> impl IntoView {
    view! {
        <button title=title>
            <svg aria-hidden="true" viewBox="0 0 24 24">
                <path d=svg_path.as_str()></path>
            </svg>
            {title}
        </button>
    }
}

#[component]
pub fn PreviewBox() -> impl IntoView {
    let (brand, set_brand) = create_signal(initial_brand_value());
    let brand_input_ref = create_node_ref::<Input>();
    let (color, set_color) = create_signal(initial_color());
    let color_input_ref = create_node_ref::<Input>();
    let (path, set_path) = create_signal(initial_path());
    let path_input_ref = create_node_ref::<Input>();

    fn contrast_color_for(hex: &str) -> String {
        let is_light_hex = color::is_relatively_light_icon_hex(hex);
        if is_light_hex { "black" } else { "white" }.to_string()
    }

    fn build_svg(path: &str, fill: Option<String>) -> String {
        format!(
            "<svg role=\"img\" viewBox=\"0 0 24 24\" xmlns=\"http://www.w3.org/2000/svg\"><path d=\"{}\"{}/></svg>",
            path,
            match fill {
                Some(fill) => format!(" fill=\"{}\"", fill),
                None => "".to_string(),
            }
        )
    }

    view! {
        <div class="preview">
            <div>
                <div class="preview-input-group">
                    <label for="preview-brand">{move_tr!("brand")}</label>
                    <input
                        _ref=brand_input_ref
                        type="text"
                        class="mr-7"
                        style="width:524px"
                        name="preview-brand"
                        value=brand
                        on:input=move |_| {
                            set_brand(brand_input_ref.get().unwrap().value());
                            update_canvas();
                        }
                    />

                </div>
                <div class="preview-input-group">
                    <label for="preview-color">Color</label>
                    <input
                        _ref=color_input_ref
                        type="text"
                        style="width:68px"
                        name="preview-color"
                        value=color
                        prop:value=color
                        on:input=move |_| {
                            let input = color_input_ref.get().unwrap();
                            let normalized_value = input.value().to_uppercase().replace('#', "");
                            input.set_value(&normalized_value);
                            set_color(normalized_value);
                            update_canvas();
                        }

                        class:invalid=move || !is_valid_hex_color(&color())
                        maxlength=6
                    />

                </div>
            </div>
            <div class="preview-input-group">
                <label for="preview-path">Path</label>
                <input
                    _ref=path_input_ref
                    type="text"
                    style="width:682px"
                    name="preview-path"
                    value=simple_icon_svg_path!("simpleicons")
                    on:input=move |_| {
                        set_path(path_input_ref.get().unwrap().value());
                        update_canvas();
                    }
                />

            </div>

            <figure class="preview-body">
                <svg
                    height="420"
                    viewBox="0 0 740 420"
                    xmlns="http://www.w3.org/2000/svg"
                    width="740"
                    class="pt-3"
                >
                    <rect
                        fill=move || format!("#{}", color())
                        height="420"
                        rx="10"
                        ry="10"
                        width="100%"
                        x="0"
                        y="0"
                        class="pt-3"
                    ></rect>
                    <svg viewBox="0 0 24 24" width="24" height="24" x="18" y="20">
                        <path d=move || path() fill=move || contrast_color_for(&color())></path>
                    </svg>
                    <svg viewBox="0 0 24 24" width="80" height="80" x="70" y="20">
                        <path d=move || path() fill=move || contrast_color_for(&color())></path>
                    </svg>
                    <svg viewBox="0 0 24 24" width="138" height="138" x="174" y="20">
                        <path d=move || path() fill=move || contrast_color_for(&color())></path>
                    </svg>
                    <svg viewBox="0 0 24 24" width="375" height="375" x="350" y="20">
                        <path d=move || path() fill=move || contrast_color_for(&color())></path>
                    </svg>

                    <g transform="translate(21,235)">
                        <text fill=move || contrast_color_for(&color()) font-size="25">
                            {move || format!("{} Preview", brand())}
                        </text>
                        <text fill=move || contrast_color_for(&color()) font-size="17" y="25">
                            {move || format!("{}.svg", title_to_slug(&brand()))}
                        </text>
                        <text fill=move || contrast_color_for(&color()) font-size="16" y="60">
                            {move || format!("Brand: {}", brand())}
                        </text>
                        <text fill=move || contrast_color_for(&color()) font-size="16" y="80">
                            {move || format!("Color: #{}", color())}
                        </text>

                        <g transform="translate(3, 142)">
                            <svg viewBox="0 0 24 24" width="24" height="24">
                                <path
                                    d=simple_icon_svg_path!("simpleicons")
                                    fill=move || contrast_color_for(&color())
                                ></path>
                            </svg>
                            <text
                                fill=move || contrast_color_for(&color())
                                x="30"
                                y="7"
                                font-size="12"
                            >
                                {format!("{} Free SVG brand icons", get_number_of_icons!())}
                            </text>
                            <text
                                fill=move || contrast_color_for(&color())
                                x="30"
                                y="25"
                                font-size="12"
                            >
                                available at simpleicons.org
                            </text>
                        </g>
                    </g>
                </svg>
                <canvas height="490" width="721"></canvas>
            </figure>
            <div class="preview-badges">
                <PreviewBadge
                    slug=move || title_to_slug(&brand())
                    color=color
                    svg=move || build_svg(&path(), None)
                    style="flat"
                    svg_color=move || "white".to_string()
                />
                <PreviewBadge
                    slug=move || title_to_slug(&brand())
                    color=color
                    svg=move || build_svg(&path(), None)
                    style="plastic"
                    svg_color=move || "white".to_string()
                />
                <PreviewBadge
                    slug=move || title_to_slug(&brand())
                    color=color
                    svg=move || build_svg(&path(), None)
                    style="for-the-badge"
                    svg_color=move || "white".to_string()
                />
                <PreviewBadge
                    slug=move || title_to_slug(&brand())
                    color=color
                    svg=move || build_svg(&path(), None)
                    style="flat-square"
                    svg_color=move || "white".to_string()
                />
                <PreviewBadge
                    slug=move || title_to_slug(&brand())
                    color=color
                    svg=move || build_svg(&path(), None)
                    style="flat"
                    svg_color=color
                />
                <PreviewBadge
                    slug=move || title_to_slug(&brand())
                    color=color
                    svg=move || build_svg(&path(), None)
                    style="plastic"
                    svg_color=color
                />
                <PreviewBadge
                    slug=move || title_to_slug(&brand())
                    color=color
                    svg=move || build_svg(&path(), None)
                    style="for-the-badge"
                    svg_color=color
                />
                <PreviewBadge
                    slug=move || title_to_slug(&brand())
                    color=color
                    svg=move || build_svg(&path(), None)
                    style="social"
                    svg_color=move || "black".to_string()
                />
            </div>
            <div class="preview-buttons">
                <PreviewButton svg_path=PreviewButtonSvgPath::Upload title="Upload SVG"/>
            </div>
        </div>
    }
}
