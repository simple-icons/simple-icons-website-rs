use i18n::move_tr;
use leptos::{html::Input, *};
use macros::simple_icon_svg_path;
use simple_icons::color;

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
        }
    }
    input.set_value(&original_value);

    if opts.update_body_bg {
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
    }
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

#[component]
pub fn PreviewBox() -> impl IntoView {
    let color_input_ref = create_node_ref::<Input>();
    color_input_ref.on_load(move |input| {
        input.set_value("111111");
        on_color_input(color_input_ref, OnColorInputOptions::new(false))
    });
    let on_color_input_opts = OnColorInputOptions::new(true);

    let path_input_ref = create_node_ref::<Input>();

    view! {
        <div class="preview">
            <div>
                <div class="preview-input-group">
                    <label for="preview-brand">{move_tr!("brand")}</label>
                    <input
                        type="text"
                        class="mr-7"
                        style="width:524px"
                        name="preview-brand"
                        value="Simple Icons"
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
                    height="476"
                    viewBox="0 0 740 476"
                    xmlns="http://www.w3.org/2000/svg"
                    width="740"
                    class="pt-3"
                >
                    <rect
                        fill="#111111"
                        height="400"
                        id="preview-body-bg"
                        rx="10"
                        ry="10"
                        width="100%"
                        x="0"
                        y="0"
                    ></rect>
                    <svg viewBox="0 0 24 24" width="24" height="24">
                        <path
                            d=simple_icon_svg_path!("simpleicons")
                            class="preview-path"
                            fill="white"
                        ></path>
                    </svg>
                    <svg viewBox="0 0 24 24" width="180" height="80">
                        <path
                            d=simple_icon_svg_path!("simpleicons")
                            class="preview-path"
                            fill="white"
                        ></path>
                    </svg>
                    <svg viewBox="0 0 24 24" width="430" height="135">
                        <path
                            d=simple_icon_svg_path!("simpleicons")
                            class="preview-path"
                            fill="white"
                        ></path>
                    </svg>
                    <svg viewBox="0 0 24 24" width="1100" height="380">
                        <path
                            d=simple_icon_svg_path!("simpleicons")
                            class="preview-path"
                            fill="white"
                        ></path>
                    </svg>
                </svg>
            </figure>
        </div>
    }
}
