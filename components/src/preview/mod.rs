use i18n::move_tr;
use leptos::{html::Input, *};
use macros::{
    get_number_of_icons, simple_icon_svg_content, simple_icon_svg_path,
};
use simple_icons::{color, sdk::title_to_slug};

fn on_brand_input(input_ref: NodeRef<Input>) {
    let input = input_ref.get().unwrap();
    let value = input.value();
    let info = document()
        .get_element_by_id("preview-info")
        .unwrap()
        .children();
    info.get_with_index(0)
        .unwrap()
        .set_inner_html(&format!("{} Preview", value));
    info.get_with_index(1)
        .unwrap()
        .set_inner_html(&format!("{}.svg", title_to_slug(&value)));
    info.get_with_index(2)
        .unwrap()
        .set_inner_html(&format!("Brand: {}", value));
}

#[derive(Copy, Clone)]
struct OnColorInputOptions {
    update_body_bg: bool,
}

impl OnColorInputOptions {
    fn new(update_body_bg: bool) -> Self {
        Self { update_body_bg }
    }
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

fn normalize_hex_color(value: &str) -> (String, String) {
    let original_value = value.to_uppercase().replace('#', "");
    let mut result = original_value.clone();
    if result.len() == 3 {
        let mut normalized = String::with_capacity(6);
        for c in result.chars() {
            normalized.push(c);
            normalized.push(c);
        }
        result = normalized;
    }
    (result, original_value)
}

fn on_color_input(input_ref: NodeRef<Input>, opts: OnColorInputOptions) {
    let input = input_ref.get().unwrap();
    let (value, original_value) = normalize_hex_color(&input.value());

    if !is_valid_hex_color(&value) {
        input.class_list().add_1("invalid").unwrap();
    } else {
        input.class_list().remove_1("invalid").unwrap();
        if opts.update_body_bg {
            let rect = document().get_element_by_id("preview-body-bg").unwrap();
            rect.set_attribute("fill", &format!("#{}", value)).unwrap();

            let is_light_hex = color::is_relatively_light_icon_hex(&value);

            let paths = document().get_elements_by_class_name("preview-path");
            for i in 0..paths.length() {
                let path = paths.get_with_index(i).unwrap();
                path.set_attribute(
                    "fill",
                    if is_light_hex { "black" } else { "white" },
                )
                .unwrap();
            }

            let preview_info_container = document()
                .get_element_by_id("preview-info")
                .unwrap()
                .children()
                .get_with_index(3)
                .unwrap();
            preview_info_container
                .set_inner_html(&format!("{}: #{}", "Color", value));
        }
    }
    input.set_value(&original_value);
}

fn on_path_input(input_ref: NodeRef<Input>) {
    let input = input_ref.get().unwrap();
    let value = input.value();
    let paths = document().get_elements_by_class_name("preview-path");
    for i in 0..paths.length() {
        let path = paths.get_with_index(i).unwrap();
        path.set_attribute("d", &value).unwrap();
    }
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
        "https://img.shields.io/badge/{}-preview-{}.svg?style={}&color={}&logo=data:image/svg%2bxml;base64,{}",
        slug,
        color,
        style,
        "green",
        window().btoa(&colored_svg).unwrap(),
    )
}

#[component]
fn PreviewBadge(
    slug: &'static str,
    color: &'static str,
    svg: &'static str,
    style: &'static str,
    svg_color: &'static str,
) -> impl IntoView {
    let url = badge_url(slug, color, svg, style, svg_color);
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
    let brand_input_ref = create_node_ref::<Input>();

    let color_input_ref = create_node_ref::<Input>();
    color_input_ref.on_load(move |input| {
        input.set_value("111111");
        on_color_input(color_input_ref, OnColorInputOptions::new(false))
    });
    let on_color_input_opts = OnColorInputOptions::new(true);

    let path_input_ref = create_node_ref::<Input>();

    let simpleicons_svg_content = simple_icon_svg_content!("simpleicons");

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
                        value="Simple Icons"
                        on:input=move |_| { on_brand_input(brand_input_ref) }
                    />
                </div>
                <div class="preview-input-group">
                    <label for="preview-color">Color</label>
                    <input
                        _ref=color_input_ref
                        type="text"
                        style="width:68px"
                        name="preview-color"
                        on:input=move |_| { on_color_input(color_input_ref, on_color_input_opts) }
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
                    on:input=move |_| { on_path_input(path_input_ref) }
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
                        fill="#111111"
                        height="420"
                        id="preview-body-bg"
                        rx="10"
                        ry="10"
                        width="100%"
                        x="0"
                        y="0"
                        class="pt-3"
                    ></rect>
                    <svg viewBox="0 0 24 24" width="24" height="24" x="18" y="20">
                        <path
                            d=simple_icon_svg_path!("simpleicons")
                            class="preview-path"
                            fill="white"
                        ></path>
                    </svg>
                    <svg viewBox="0 0 24 24" width="80" height="80" x="70" y="20">
                        <path
                            d=simple_icon_svg_path!("simpleicons")
                            class="preview-path"
                            fill="white"
                        ></path>
                    </svg>
                    <svg viewBox="0 0 24 24" width="138" height="138" x="174" y="20">
                        <path
                            d=simple_icon_svg_path!("simpleicons")
                            class="preview-path"
                            fill="white"
                        ></path>
                    </svg>
                    <svg viewBox="0 0 24 24" width="375" height="375" x="350" y="20">
                        <path
                            d=simple_icon_svg_path!("simpleicons")
                            class="preview-path"
                            fill="white"
                        ></path>
                    </svg>

                    <g id="preview-info" transform="translate(21,235)">
                        <text fill="white" font-size="25">
                            Simple Icons Preview
                        </text>
                        <text fill="white" font-size="17" y="25">
                            simpleicons.svg
                        </text>
                        <text fill="white" font-size="16" y="60">
                            Brand: Simple Icons
                        </text>
                        <text fill="white" font-size="16" y="80">
                            Color: #111111
                        </text>

                        <g transform="translate(3, 142)">
                            <svg viewBox="0 0 24 24" width="24" height="24">
                                <path
                                    d=simple_icon_svg_path!("simpleicons")
                                    class="preview-path"
                                    fill="white"
                                ></path>
                            </svg>
                            <text fill="white" x="30" y="7" font-size="12">
                                {format!("{} Free SVG brand icons", get_number_of_icons!())}
                            </text>
                            <text fill="white" x="30" y="25" font-size="12">
                                available at simpleicons.org
                            </text>
                        </g>
                    </g>
                </svg>
                <canvas height="490" width="721"></canvas>
            </figure>
            <div class="preview-badges">
                <PreviewBadge
                    slug="simpleicons"
                    color="111111"
                    svg=simpleicons_svg_content
                    style="flat"
                    svg_color="white"
                />
                <PreviewBadge
                    slug="simpleicons"
                    color="111111"
                    svg=simpleicons_svg_content
                    style="plastic"
                    svg_color="white"
                />
                <PreviewBadge
                    slug="simpleicons"
                    color="111111"
                    svg=simpleicons_svg_content
                    style="for-the-badge"
                    svg_color="white"
                />
                <PreviewBadge
                    slug="simpleicons"
                    color="111111"
                    svg=simpleicons_svg_content
                    style="flat-square"
                    svg_color="white"
                />
                <PreviewBadge
                    slug="simpleicons"
                    color="111111"
                    svg=simpleicons_svg_content
                    style="flat"
                    svg_color="111111"
                />
                <PreviewBadge
                    slug="simpleicons"
                    color="111111"
                    svg=simpleicons_svg_content
                    style="plastic"
                    svg_color="111111"
                />
                <PreviewBadge
                    slug="simpleicons"
                    color="111111"
                    svg=simpleicons_svg_content
                    style="for-the-badge"
                    svg_color="111111"
                />
                <PreviewBadge
                    slug="simpleicons"
                    color="111111"
                    svg=simpleicons_svg_content
                    style="social"
                    svg_color="black"
                />
            </div>
            <div class="preview-buttons">
                <PreviewButton svg_path=PreviewButtonSvgPath::Upload title="Upload SVG"/>
            </div>
        </div>
    }
}
