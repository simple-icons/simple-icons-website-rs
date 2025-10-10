use leptos::prelude::*;
use simple_icons_website_ids::Ids;
use wasm_bindgen::{JsCast, closure::Closure};

pub(crate) static WIDTH: u32 = 740;
pub(crate) static HEIGHT: u32 = 520;

pub fn canvas() -> web_sys::HtmlCanvasElement {
    document()
        .query_selector(".preview-figure canvas")
        .unwrap()
        .unwrap()
        .unchecked_into::<web_sys::HtmlCanvasElement>()
}

fn canvas_ctx(
    canvas_container: &web_sys::HtmlCanvasElement,
) -> web_sys::CanvasRenderingContext2d {
    canvas_container
        .get_context("2d")
        .unwrap()
        .unwrap()
        .unchecked_into::<web_sys::CanvasRenderingContext2d>()
}

/// Function triggered to update the canvas with the current SVG
pub fn update_preview_canvas(pixel_ratio: f64) {
    let ratio = js_sys::Math::max(pixel_ratio, 1.0);

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
        .unchecked_into::<web_sys::HtmlElement>();
    let canvas = figure
        .get_elements_by_tag_name("canvas")
        .item(0)
        .unwrap()
        .unchecked_into::<web_sys::HtmlCanvasElement>();
    canvas
        .set_attribute(
            "width",
            &format!("{}", js_sys::Math::floor(WIDTH as f64 * ratio)),
        )
        .unwrap();
    canvas
        .set_attribute(
            "height",
            &format!("{}", js_sys::Math::floor(HEIGHT as f64 * ratio)),
        )
        .unwrap();
    canvas
        .set_attribute(
            "style",
            &format!("width: {WIDTH}px; height: {HEIGHT}px;"),
        )
        .unwrap();

    let ctx = canvas_ctx(&canvas);
    ctx.set_font("1rem sans");
    ctx.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());
    ctx.scale(ratio, ratio).unwrap();

    // Draw the SVG of the preview card in the canvas
    let preview_card_svg =
        figure.get_elements_by_tag_name("svg").item(0).unwrap();
    let preview_card_img = document()
        .create_element("img")
        .unwrap()
        .unchecked_into::<web_sys::HtmlImageElement>();
    preview_card_img.class_list().add_1("hidden").unwrap();
    preview_card_img
        .set_attribute("id", Ids::PreviewCardImageForCanvas.as_str())
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
            .get_element_by_id(Ids::PreviewCardImageForCanvas.as_str())
            .unwrap()
            .unchecked_into::<web_sys::HtmlImageElement>();
        ctx.draw_image_with_html_image_element(&preview_card_img, 0.0, 0.0)
            .unwrap();
        document()
            .body()
            .unwrap()
            .remove_child(&preview_card_img)
            .unwrap();
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
