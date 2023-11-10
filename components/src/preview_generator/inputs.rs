use crate::button::Button;
use crate::controls::search::fuzzy::search;
use crate::event::dispatch_input_event_on_input;
use crate::fetch::fetch_text;
use crate::grid::ICONS;
use crate::js_libs::svg::svg_path_bbox;
use crate::preview_generator::{
    canvas::update_preview_canvas, helpers::is_valid_hex_color,
};
use i18n::move_tr;
use leptos::{
    html::{Div, Input},
    *,
};
use leptos_use::on_click_outside;
use simple_icons::sdk;
use svg_path_cst::svg_path_cst;
use types::SimpleIcon;
use wasm_bindgen::JsCast;

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
        create_signal::<Vec<sdk::lint::LintError>>(vec![]);
    let (show_path_lint_errors, set_show_path_lint_errors) =
        create_signal(false);
    let input_ref = create_node_ref::<Input>();
    let input_group_ref = create_node_ref::<Div>();

    fn process_lint_errors(
        path: &str,
        set_path_lint_errors: WriteSignal<Vec<sdk::lint::LintError>>,
    ) {
        let mut new_lint_errors = sdk::lint::lint_path_characters(path);
        if !new_lint_errors.is_empty() {
            set_path_lint_errors(new_lint_errors);
            return;
        }

        let path_segments = match svg_path_cst(path) {
            Ok(path_segments) => path_segments,
            Err(err) => {
                new_lint_errors.push((err.to_string(), None, None));
                set_path_lint_errors(new_lint_errors);
                return;
            }
        };

        let (path_bbox, path_bbox_error) = svg_path_bbox(path);
        if let Some(err) = path_bbox_error {
            new_lint_errors.push((err, None, None));
            set_path_lint_errors(new_lint_errors);
            return;
        }

        new_lint_errors.extend(sdk::lint::lint_path_segments(&path_segments));
        new_lint_errors.extend(sdk::lint::lint_path_bbox(&path_bbox));
        set_path_lint_errors(new_lint_errors);
    }

    _ = on_click_outside(input_group_ref, move |_| {
        set_show_path_lint_errors(false)
    });

    view! {
        <div node_ref=input_group_ref class="preview-input-group">
            <label for="preview-path">{move_tr!("path")}</label>
            <input
                node_ref=input_ref
                type="text"
                style="width:682px"
                name="preview-path"
                value=path
                prop:value=path
                class:warn=move || !path_lint_errors().is_empty()
                on:input=move |_| {
                    let p = input_ref().unwrap().value();
                    process_lint_errors(&p, set_path_lint_errors);
                    set_show_path_lint_errors(true);
                    set_path(p);
                    update_preview_canvas();
                }

                on:focus=move |_| {
                    let p = input_ref().unwrap().value();
                    process_lint_errors(&p, set_path_lint_errors);
                    set_show_path_lint_errors(true);
                }
            />

            <Show when=show_path_lint_errors>
                <ul class="preview-path-lint-errors">
                    <For
                        each=path_lint_errors
                        key=move |error| {
                            format!(
                                "{}{}",
                                error.0,
                                match error.1 {
                                    Some(range) => format!("-{}-{}", range.0, range.1),
                                    None => "".to_string(),
                                },
                            )
                        }

                        children=move |error| {
                            view! {
                                <LintError
                                    message=error.0
                                    range=error.1
                                    fixer=error.2
                                    input_ref=input_ref
                                />
                            }
                        }
                    />

                </ul>
            </Show>
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
                let input = input_ref().unwrap();
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
                let input = input_ref().unwrap();
                let (new_value, (start, end)) = fixer(&input.value(), (start, end));
                input.set_value(&new_value);
                dispatch_input_event_on_input(&input);
                set_timeout(
                    move || {
                        input.focus().unwrap();
                        input.select();
                        input.set_selection_start(Some(start)).unwrap();
                        input.set_selection_end(Some(end)).unwrap();
                    },
                    std::time::Duration::from_millis(3),
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
                <Show when=move || range.is_some()>
                    <ShowLintErrorButton start=range.unwrap().0 end=range.unwrap().1 input_ref/>
                </Show>
                <Show when=move || fixer.is_some()>
                    <FixLintErrorButton
                        start=range.unwrap().0
                        end=range.unwrap().1
                        fixer=fixer.unwrap()
                        input_ref=input_ref
                    />
                </Show>
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
        create_signal(Vec::<&SimpleIcon>::with_capacity(7));
    let (more_brand_suggestions, set_more_brand_suggestions) =
        create_signal(Vec::<&SimpleIcon>::new());
    let (show_brand_suggestions, set_show_brand_suggestions) =
        create_signal(false);
    let (show_more_brand_suggestions, set_show_more_brand_suggestions) =
        create_signal(false);

    let input_ref = create_node_ref::<Input>();
    _ = on_click_outside(input_ref, move |_| {
        set_show_brand_suggestions(false);
        set_show_more_brand_suggestions(false);
    });

    view! {
        <div class="preview-input-group">
            <label for="preview-brand">{move_tr!("brand")}</label>
            <input
                node_ref=input_ref
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

            <Show when=move || show_brand_suggestions() && !brand_suggestions().is_empty()>
                <BrandSuggestions
                    show_more_brand_suggestions=show_more_brand_suggestions
                    brand_suggestions=brand_suggestions
                    more_brand_suggestions=more_brand_suggestions
                    set_brand=set_brand
                    set_color=set_color
                    set_show_more_brand_suggestions=set_show_more_brand_suggestions
                />
            </Show>
        </div>
    }
}

#[component]
fn BrandSuggestions(
    show_more_brand_suggestions: ReadSignal<bool>,
    brand_suggestions: ReadSignal<Vec<&'static SimpleIcon>>,
    more_brand_suggestions: ReadSignal<Vec<&'static SimpleIcon>>,
    set_brand: WriteSignal<String>,
    set_color: WriteSignal<String>,
    set_show_more_brand_suggestions: WriteSignal<bool>,
) -> impl IntoView {
    view! {
        <ul class=move || {
            format!(
                "preview-brand-suggestions{}",
                if show_more_brand_suggestions() { " overflow-y-scroll" } else { "" },
            )
        }>

            <For
                each=brand_suggestions
                key=move |icon| icon.slug
                children=move |icon: &'static SimpleIcon| {
                    view! { <BrandSuggestion icon=icon set_brand=set_brand set_color=set_color/> }
                }
            />

            <Show when=move || {
                !show_more_brand_suggestions() && !more_brand_suggestions().is_empty()
            }>
                <li
                    class="more-suggestions"
                    role="button"
                    title=move_tr!("load-more-icons")
                    on:click=move |_| {
                        set_show_more_brand_suggestions(true);
                        let input = document()
                            .get_elements_by_name("preview-brand")
                            .item(0)
                            .unwrap()
                            .dyn_into::<web_sys::HtmlInputElement>()
                            .unwrap();
                        input.focus().unwrap();
                    }
                >

                    <span>+</span>
                </li>
            </Show>
            <Show when=show_more_brand_suggestions>
                <For
                    each=more_brand_suggestions
                    key=move |icon| icon.slug
                    children=move |icon| {
                        view! {
                            <BrandSuggestion icon=icon set_brand=set_brand set_color=set_color/>
                        }
                    }
                />

            </Show>
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
                    dispatch_input_event_on_input(&path_input);
                    update_preview_canvas();
                }
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
    let mut initial_icons: Vec<&'static SimpleIcon> = Vec::with_capacity(7);
    let mut more_icons: Vec<&'static SimpleIcon> = vec![];
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
