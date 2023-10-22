use crate::button::Button;
use crate::controls::search::fuzzy::search;
use crate::fetch::fetch_text;
use crate::grid::ICONS;
use crate::js_libs::svg::{svg_path_bbox, svg_path_segments};
use crate::preview_generator::{
    canvas::update_preview_canvas, helpers::is_valid_hex_color,
};
use i18n::move_tr;
use leptos::{html::Input, *};
use simple_icons::sdk;
use std::time::Duration;
use types::SimpleIcon;
use wasm_bindgen::{closure::Closure, JsCast};

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
                    let input = event_target::<web_sys::HtmlInputElement>(&ev);
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
pub fn PathInput(
    path: ReadSignal<String>,
    set_path: WriteSignal<String>,
) -> impl IntoView
where {
    let (path_lint_errors, set_path_lint_errors) =
        create_signal::<Vec<sdk::lint::LintError>>(Vec::new());
    let (show_path_lint_errors, set_show_path_lint_errors) =
        create_signal(false);
    let input_ref = create_node_ref::<Input>();

    fn process_lint_errors(
        path: &str,
        path_lint_errors: ReadSignal<Vec<sdk::lint::LintError>>,
        set_path_lint_errors: WriteSignal<Vec<sdk::lint::LintError>>,
    ) {
        let mut new_lint_errors = path_lint_errors().clone();
        let (path_segments, path_segments_error) = svg_path_segments(path);
        if let Some(err) = path_segments_error {
            new_lint_errors.push((err, None, None));
            set_path_lint_errors(new_lint_errors);
            return;
        }
        let (path_bbox, path_bbox_error) = svg_path_bbox(path);
        if let Some(err) = path_bbox_error {
            new_lint_errors.push((err, None, None));
            set_path_lint_errors(new_lint_errors);
            return;
        }
        let lint_errors =
            sdk::lint::lint_path(path, &path_bbox, &path_segments);
        set_path_lint_errors(lint_errors);
    }

    let body = document().body().unwrap();
    let closure: Closure<dyn FnMut(web_sys::MouseEvent)> =
        Closure::new(move |ev: web_sys::MouseEvent| {
            let target = event_target::<web_sys::HtmlElement>(&ev);
            // Hide the brand suggestions when the user clicks outside the input
            if target.get_attribute("name") == Some("preview-path".to_string())
            {
                return;
            }

            let input_group = input_ref
                .get()
                .unwrap()
                .parent_element()
                .unwrap()
                .dyn_into::<web_sys::HtmlElement>()
                .unwrap();
            let composed_path = ev.composed_path().iter().collect::<Vec<_>>();
            if composed_path.contains(&input_group) {
                return;
            }
            set_show_path_lint_errors(false);
        });
    body.add_event_listener_with_callback(
        "click",
        closure.as_ref().unchecked_ref(),
    )
    .unwrap();
    closure.forget();

    view! {
        <div class="preview-input-group">
            <label for="preview-path">{move_tr!("path")}</label>
            <input
                _ref=input_ref
                type="text"
                style="width:682px"
                name="preview-path"
                value=path
                prop:value=path
                class:warn=move || !path_lint_errors().is_empty()
                on:input=move |_| {
                    let p = input_ref.get().unwrap().value();
                    process_lint_errors(&p, path_lint_errors, set_path_lint_errors);
                    set_show_path_lint_errors(true);
                    set_path(p);
                    update_preview_canvas();
                }

                on:focus=move |_| {
                    let p = input_ref.get().unwrap().value();
                    process_lint_errors(&p, path_lint_errors, set_path_lint_errors);
                    set_show_path_lint_errors(true);
                }
            />

            <ul class="preview-path-lint-errors" class:hidden=move || !show_path_lint_errors()>
                {move || {
                    path_lint_errors()
                        .into_iter()
                        .map(|error| {
                            view! {
                                <LintError
                                    message=error.0
                                    range=error.1
                                    fixer=error.2
                                    input_ref=input_ref
                                />
                            }
                        })
                        .collect_view()
                }}

            </ul>
        </div>
    }
}

#[component]
fn ShowLintErrorButton(
    start: u32,
    end: u32,
    input_ref: NodeRef<Input>,
) -> impl IntoView {
    view! {
        <Button
            title=move || "Show".to_string()
            on:click=move |_| {
                let input = input_ref.get().unwrap();
                input.focus().unwrap();
                input.set_selection_start(Some(start)).unwrap();
                input.set_selection_end(Some(end)).unwrap();
            }
        />
    }
}

#[component]
fn FixLintErrorButton(
    start: u32,
    end: u32,
    fixer: sdk::lint::LintErrorFixer,
    input_ref: NodeRef<Input>,
) -> impl IntoView {
    view! {
        <Button
            title=move || "Fix".to_string()
            on:click=move |_| {
                let input = input_ref.get().unwrap();
                let (new_value, (start, end)) = fixer(&input.value(), (start, end));
                input.set_value(&new_value);
                let event = web_sys::Event::new_with_event_init_dict(
                        "input",
                        web_sys::EventInit::new().bubbles(true),
                    )
                    .unwrap();
                input.dispatch_event(&event).unwrap();
                _ = set_timeout_with_handle(
                    move || {
                        input.focus().unwrap();
                        input.select();
                        input.set_selection_start(Some(start)).unwrap();
                        input.set_selection_end(Some(end)).unwrap();
                    },
                    Duration::from_millis(3),
                );
            }
        />
    }
}

#[component]
fn LintError(
    message: String,
    range: Option<(u32, u32)>,
    fixer: Option<sdk::lint::LintErrorFixer>,
    input_ref: NodeRef<Input>,
) -> impl IntoView {
    view! {
        <li>
            <span>{message}</span>
            <div>
                {move || {
                    let mut buttons = vec![];
                    if let Some(range) = range {
                        buttons
                            .push(
                                view! { <ShowLintErrorButton start=range.0 end=range.1 input_ref/> },
                            );
                        if let Some(fixer) = fixer {
                            buttons
                                .push(
                                    view! {
                                        <FixLintErrorButton
                                            start=range.0
                                            end=range.1
                                            fixer=fixer
                                            input_ref=input_ref
                                        />
                                    },
                                );
                        }
                    }
                    buttons
                }}

            </div>

        </li>
    }
}

#[component]
pub fn BrandInput(
    brand: ReadSignal<String>,
    set_brand: WriteSignal<String>,
    set_color: WriteSignal<String>,
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
                    let value = event_target_value::<web_sys::Event>(&ev);
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
                    let value = event_target_value::<web_sys::Event>(&ev);
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
    set_show_brand_suggestions: WriteSignal<bool>,
    set_show_more_brand_suggestions: WriteSignal<bool>,
) -> impl IntoView {
    let body = document().body().unwrap();
    let closure: Closure<dyn FnMut(web_sys::MouseEvent)> =
        Closure::new(move |ev: web_sys::MouseEvent| {
            let target = event_target::<web_sys::HtmlElement>(&ev);
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
    body.add_event_listener_with_callback(
        "click",
        closure.as_ref().unchecked_ref(),
    )
    .unwrap();
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
                                <BrandSuggestion icon=icon set_brand=set_brand set_color=set_color/>
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
) -> impl IntoView {
    view! {
        <li on:click=move |_| {
            set_brand(icon.title.to_string());
            set_color(icon.hex.to_string());
            spawn_local(async move {
                if let Some(svg) = fetch_text(&format!("/icons/{}.svg", icon.slug)).await {
                    let path_input = document()
                        .get_elements_by_name("preview-path")
                        .item(0)
                        .unwrap()
                        .dyn_into::<web_sys::HtmlInputElement>()
                        .unwrap();
                    path_input.set_value(&sdk::svg_to_path(&svg));
                    let event = web_sys::Event::new_with_event_init_dict(
                            "input",
                            web_sys::EventInit::new().bubbles(true),
                        )
                        .unwrap();
                    path_input.dispatch_event(&event).unwrap();
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
    let mut initial_icons: Vec<&'static SimpleIcon> = Vec::with_capacity(6);
    let mut more_icons: Vec<&'static SimpleIcon> = Vec::new();
    let search_result = js_sys::Array::from(&search(value));
    let search_result_length = search_result.length();
    for i in 0..search_result_length {
        let result_icon_array = js_sys::Array::from(&search_result.get(i));
        let icon_order_alpha = result_icon_array.get(1).as_f64().unwrap();
        if i > 5 {
            more_icons.push(&ICONS[icon_order_alpha as usize]);
        } else {
            initial_icons.push(&ICONS[icon_order_alpha as usize]);
        }
    }
    (initial_icons, more_icons)
}
