use crate::controls::download::download;
use i18n::move_tr;
use leptos::{html::Input, *};
use macros::{get_number_of_icons, simple_icon_svg_path};
use simple_icons::{color, sdk::title_to_slug};
use wasm_bindgen::{closure::Closure, JsCast};

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

fn badge_url(slug: &str, color: &str, svg: &str, style: &str) -> String {
    format!(
        "https://img.shields.io/badge/{}-preview-{}.svg?style={}&logo=data:image/svg%2bxml;base64,{}",
        slug,
        color,
        style,
        window().btoa(svg).unwrap(),
    )
}

enum PreviewButtonSvgPath {
    Upload,
    Download,
    Save,
}

impl PreviewButtonSvgPath {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Upload => "M9,16V10H5L12,3L19,10H15V16H9M5,20V18H19V20H5",
            Self::Download => "M5,20H19V18H5M19,9H15V3H9V9H5L12,16L19,9",
            Self::Save => "M15,9H5V5H15M12,19A3,3 0 0,1 9,16A3,3 0 0,1 12,13A3,3 0 0,1 15,16A3,3 0 0,1 12,19M17,3H5C3.89,3 3,3.9 3,5V19A2,2 0 0,0 5,21H19A2,2 0 0,0 21,19V7L17,3Z",
        }
    }
}

fn get_preview_canvas_context() -> web_sys::CanvasRenderingContext2d {
    let container = document()
        .get_elements_by_class_name("preview-body")
        .item(0)
        .unwrap();
    let figure = container.dyn_into::<web_sys::HtmlElement>().unwrap();
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
    ctx.set_font("1rem sans");
    ctx
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
        let badge_url = badge_img.src();

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
                &format!("preview-badge-image-for-canvas-{}", $badge_index),
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
                    $badge_index
                ))
                .unwrap()
                .dyn_into::<web_sys::HtmlImageElement>()
                .unwrap();

            let ctx = get_preview_canvas_context();
            ctx.draw_image_with_html_image_element(
                &img,
                $x as f64,
                420.0 + $y as f64,
            )
            .unwrap();
            document().body().unwrap().remove_child(&img).unwrap();
        });
        badge_img_for_canvas.set_onload(Some(closure.as_ref().unchecked_ref()));
        closure.forget();

        badge_img_for_canvas
            .set_attribute("src", badge_url.as_str())
            .unwrap();
    }};
}

fn update_badges_in_canvas() {
    // Draw the badges in the canvas
    draw_badge_impl!(0, 15, 15);
    draw_badge_impl!(1, 173, 16);
    draw_badge_impl!(2, 335, 6);
    draw_badge_impl!(3, 562, 15);

    draw_badge_impl!(4, 15, 41);
    draw_badge_impl!(5, 173, 41);
    draw_badge_impl!(6, 335, 39);
    draw_badge_impl!(7, 560, 41);
}

fn update_canvas() {
    let container = document()
        .get_elements_by_class_name("preview-body")
        .item(0);
    if container.is_none() {
        return;
    }

    let figure = document()
        .get_elements_by_class_name("preview-body")
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

#[component]
fn PreviewButton(
    svg_path: PreviewButtonSvgPath,
    title: &'static str,
    #[prop(optional)] class: &'static str,
) -> impl IntoView {
    view! {
        <button title=title class=class>
            <svg aria-hidden="true" viewBox="0 0 24 24" width="24" height="24">
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
        let is_light_hex =
            is_valid_hex_color(hex) && color::is_relatively_light_icon_hex(hex);
        if is_light_hex { "black" } else { "white" }.to_string()
    }

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
                            let selection_start = input.selection_start().unwrap();
                            let selection_end = input.selection_end().unwrap();
                            let normalized_value = input.value().to_uppercase().replace('#', "");
                            input.set_value(&normalized_value);
                            input.set_selection_start(selection_start).unwrap();
                            input.set_selection_end(selection_end).unwrap();
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

                    <g transform="translate(21,235)" style="font-family: Helvetica">
                        <text fill=move || contrast_color_for(&color()) font-size="25">
                            {move || format!("{} Preview", brand())}
                        </text>
                        <text fill=move || contrast_color_for(&color()) font-size="17" y="25">
                            {move || format!("{}.svg", title_to_slug(&brand()))}
                        </text>
                        <text fill=move || contrast_color_for(&color()) font-size="16" y="61">
                            {move || format!("Brand: {}", brand())}
                        </text>
                        <text fill=move || contrast_color_for(&color()) font-size="16" y="84">
                            {move || format!("Color: #{}", color())}
                        </text>

                        <g transform="translate(3, 142)" style="font-family: Helvetica">
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
                <div>
                    <img src=move || badge_url(
                        &title_to_slug(&brand()),
                        &color(),
                        &build_svg(&path(), Some("FFF")),
                        "flat",
                    )/>
                </div>
                <div>
                    <img src=move || badge_url(
                        &title_to_slug(&brand()),
                        &color(),
                        &build_svg(&path(), Some("FFF")),
                        "plastic",
                    )/>
                </div>
                <div>
                    <img src=move || badge_url(
                        &title_to_slug(&brand()),
                        &color(),
                        &build_svg(&path(), Some("FFF")),
                        "for-the-badge",
                    )/>
                </div>
                <div>
                    <img src=move || badge_url(
                        &title_to_slug(&brand()),
                        &color(),
                        &build_svg(&path(), Some("FFF")),
                        "flat-square",
                    )/>
                </div>
                <div>
                    <img src=move || badge_url(
                        &title_to_slug(&brand()),
                        &color(),
                        &build_svg(&path(), Some(&color())),
                        "flat",
                    )/>
                </div>
                <div>
                    <img src=move || badge_url(
                        &title_to_slug(&brand()),
                        &color(),
                        &build_svg(&path(), Some(&color())),
                        "plastic",
                    )/>
                </div>
                <div>
                    <img src=move || badge_url(
                        &title_to_slug(&brand()),
                        &color(),
                        &build_svg(&path(), Some(&color())),
                        "for-the-badge",
                    )/>
                </div>
                <div>
                    <img
                        src=move || badge_url(
                            &title_to_slug(&brand()),
                            &color(),
                            &build_svg(&path(), Some("000")),
                            "social",
                        )

                        on:load=move |_| update_canvas()
                    />
                </div>
            </div>
            <div class="preview-buttons">
                <PreviewButton svg_path=PreviewButtonSvgPath::Upload title="Upload SVG"/>
                <PreviewButton
                    svg_path=PreviewButtonSvgPath::Save
                    title="Save preview"
                    class="float-right ml-4"
                    on:click=move |el| {
                        let figure = document()
                            .get_elements_by_class_name("preview-body")
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
                        let filename = format!("{}.png", title_to_slug(&brand()));
                        let url = canvas.to_data_url().unwrap();
                        download(&filename, &url);
                        let target = el.target().unwrap();
                        target.dyn_into::<web_sys::HtmlElement>().unwrap().blur().unwrap();
                    }
                />

                <PreviewButton
                    svg_path=PreviewButtonSvgPath::Download
                    title="Download SVG"
                    class="float-right"
                    on:click=move |el| {
                        let filename = format!("{}.svg", title_to_slug(&brand()));
                        let url = format!(
                            "data:image/svg+xml;utf8,{}",
                            js_sys::encode_uri_component(&build_svg(&path(), None)),
                        );
                        download(&filename, &url);
                        let target = el.target().unwrap();
                        target.dyn_into::<web_sys::HtmlElement>().unwrap().blur().unwrap();
                    }
                />

            </div>
        </div>
    }
}
