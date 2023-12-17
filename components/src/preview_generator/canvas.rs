use leptos::*;
use wasm_bindgen::{closure::Closure, JsCast};

pub fn get_canvas_container() -> web_sys::HtmlCanvasElement {
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

pub(crate) fn create_badge_image_for_canvas(
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
    badge_img_for_canvas.class_list().add_1("hidden").unwrap();
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
    ($badge_index:literal, $x:literal, $y:literal$(,)?) => {{
        let badge_img_src = ::leptos::document()
            .get_elements_by_class_name("preview-badges")
            .item(0)
            .unwrap()
            .children()
            .item($badge_index)
            .unwrap()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap()
            .first_element_child()
            .unwrap()
            .dyn_into::<web_sys::HtmlImageElement>()
            .unwrap()
            .src();

        $crate::preview_generator::canvas::create_badge_image_for_canvas(
            $badge_index,
            &badge_img_src,
            $x as f64,
            $y as f64,
        )
    }};
}

/// Draw the current badges in the canvas
fn update_badges_in_canvas() {
    draw_badge_impl!(0, 28, 15);
    draw_badge_impl!(1, 207, 16);
    draw_badge_impl!(2, 385, 6);
    draw_badge_impl!(3, 630, 14);

    draw_badge_impl!(4, 28, 41);
    draw_badge_impl!(5, 207, 41);
    draw_badge_impl!(6, 385, 39);
    draw_badge_impl!(7, 630, 40);
}

/// Function triggered to update the canvas with the current SVG
pub fn update_preview_canvas() {
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
    preview_card_img.class_list().add_1("hidden").unwrap();
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
