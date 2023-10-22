use crate::controls::search::fuzzy::search;
use crate::fetch::fetch_text;
use crate::grid::ICONS;
use crate::preview_generator::{
    canvas::update_preview_canvas, helpers::is_valid_hex_color,
};
use i18n::move_tr;
use leptos::*;
use simple_icons::sdk;
use types::SimpleIcon;
use wasm_bindgen::{closure::Closure, JsCast};

#[component]
pub fn PathInput(
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
pub fn ColorInput(
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
pub fn BrandInput(
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
