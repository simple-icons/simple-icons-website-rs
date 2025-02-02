use crate::{canvas::update_preview_canvas, helpers::is_valid_hex_color};
use leptos::{html::Input, prelude::*, task::spawn_local};
use leptos_fluent::{move_tr, tr};
use leptos_use::{on_click_outside, use_device_pixel_ratio};
use simple_icons::lint::errors::PathLintError;
use simple_icons_sdk as sdk;
use simple_icons_website_components::{
    controls::search::fuzzy::search, grid::ICONS,
};
use simple_icons_website_types::SimpleIcon;
use svg_path_bbox::svg_path_bbox;
use svg_path_cst::svg_path_cst;
use wasm_bindgen::JsCast;
use web_sys_simple_events::dispatch_input_event_on_input;
use web_sys_simple_fetch::fetch_text;

#[component]
pub fn ColorInput(
    color: ReadSignal<String>,
    set_color: WriteSignal<String>,
) -> impl IntoView {
    let pixel_ratio = use_device_pixel_ratio();

    view! {
        <div class="preview-input-group">
            <label for="preview-color">{move || tr!("color")}</label>
            <input
                type="text"
                style="width:68px"
                id="preview-color"
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
                    update_preview_canvas(pixel_ratio.get_untracked());
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
) -> impl IntoView {
    let pixel_ratio = use_device_pixel_ratio();

    let (path_lint_errors, set_path_lint_errors) =
        signal::<Vec<simple_icons::lint::LintError>>(vec![]);
    let (show_path_lint_errors, set_show_path_lint_errors) = signal(false);
    let input_ref = NodeRef::new();
    let input_group_ref = NodeRef::new();

    fn process_lint_errors(
        path: &str,
        set_path_lint_errors: WriteSignal<Vec<simple_icons::lint::LintError>>,
    ) {
        let mut new_lint_errors =
            simple_icons::lint::lint_path_characters(path);
        if !new_lint_errors.is_empty() {
            set_path_lint_errors(new_lint_errors);
            return;
        }

        let path_segments = match svg_path_cst(path.as_bytes()) {
            Ok(path_segments) => path_segments,
            Err(err) => {
                new_lint_errors.push((
                    PathLintError::SyntaxError {
                        message: err.to_string(),
                    },
                    None,
                    None,
                ));
                set_path_lint_errors(new_lint_errors);
                return;
            }
        };

        let maybe_path_bbox = svg_path_bbox(path);
        if let Err(err) = maybe_path_bbox {
            new_lint_errors.push((
                PathLintError::ViewboxSyntaxError { message: err },
                None,
                None,
            ));
            set_path_lint_errors(new_lint_errors);
            return;
        }
        let path_bbox = maybe_path_bbox.unwrap();

        new_lint_errors
            .extend(simple_icons::lint::lint_path_segments(&path_segments));
        new_lint_errors.extend(simple_icons::lint::lint_path_bbox(&path_bbox));
        set_path_lint_errors(new_lint_errors);
    }

    _ = on_click_outside(input_group_ref, move |_| {
        set_show_path_lint_errors(false)
    });

    let tr_lint_error = move |err: &PathLintError| -> String {
        match err {
            PathLintError::MustStartWithMovetoCommand { command } => tr!(
                "must-start-with-moveto-command",
                { "command" => command.to_string() }
            ),
            PathLintError::InvalidCharacterAtIndex { index, character } => {
                tr!(
                    "invalid-character-at-index",
                    {
                        "index" => index.to_string(),
                        "character" => character.to_string()
                    }
                )
            }
            PathLintError::FoundNegativeZeroAtIndex { index } => tr!(
                "found-negative-zero-at-index",
                {"index" => index.to_string()}
            ),
            PathLintError::ReportedSizeIsZero => {
                tr!("reported-svg-path-size-is-zero")
            }
            PathLintError::MaximumPrecisionMustBeLessThan {
                max_precision,
                precision,
                number,
            } => tr!(
                "maximum-precision-must-be-less-than",
                {
                    "max_precision" => max_precision.to_string(),
                    "precision" => precision.to_string(),
                    "number" => number.to_string()
                }
            ),
            PathLintError::IconMustBeCentered { x, y } => tr!(
                "icon-must-be-centered",
                {
                    "x" => x.to_string(),
                    "y" => y.to_string()
                }
            ),
            PathLintError::CollinearSegmentFoundAtCommand { command } => tr!(
                "collinear-segment-found-at-command",
                {"command" => command.to_string()}
            ),
            PathLintError::IncorrectIconSize { width, height } => tr!(
                "incorrect-svg-path-icon-size",
                {
                    "width" => width.to_string(),
                    "height" => height.to_string()
                }
            ),
            _ => err.to_string(),
        }
    };

    view! {
        <div node_ref=input_group_ref class="preview-input-group">
            <label for="preview-path">{move || tr!("path")}</label>
            <input
                node_ref=input_ref
                type="text"
                style="width:682px"
                id="preview-path"
                value=path
                prop:value=path
                class:warn=move || !path_lint_errors().is_empty()
                on:input=move |_| {
                    let p = input_ref.get().unwrap().value();
                    process_lint_errors(&p, set_path_lint_errors);
                    set_show_path_lint_errors(true);
                    set_path(p);
                    update_preview_canvas(pixel_ratio.get_untracked());
                }

                on:focus=move |_| {
                    let p = input_ref.get().unwrap().value();
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
                                    message=Signal::derive(move || tr_lint_error(&error.0))
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
        <button
            title=move_tr!("show")
            class="button"
            type="button"
            tabindex=0
            on:click=move |_| {
                let input = input_ref.get().unwrap();
                _ = input.focus();
                input.set_selection_start(Some(start)).unwrap();
                input.set_selection_end(Some(end)).unwrap();
            }
        >
            {move_tr!("show")}
        </button>
    }
}

#[component]
fn FixLintErrorButton(
    start: u32,
    end: u32,
    fixer: simple_icons::lint::LintErrorFixer,
    input_ref: NodeRef<Input>,
) -> impl IntoView {
    view! {
        <button
            title=move_tr!("fix")
            class="button"
            type="button"
            tabindex=0
            on:click=move |_| {
                let input = input_ref.get().unwrap();
                let (new_value, (start, end)) = fixer(&input.value(), (start, end));
                input.set_value(&new_value);
                dispatch_input_event_on_input(&input);
                set_timeout(
                    move || {
                        _ = input.focus();
                        input.select();
                        input.set_selection_start(Some(start)).unwrap();
                        input.set_selection_end(Some(end)).unwrap();
                    },
                    std::time::Duration::from_millis(3),
                );
            }
        >
            {move_tr!("fix")}
        </button>
    }
}

#[component]
fn LintError(
    message: Signal<String>,
    range: Option<(u32, u32)>,
    fixer: Option<simple_icons::lint::LintErrorFixer>,
    input_ref: NodeRef<Input>,
) -> impl IntoView {
    view! {
        <li>
            <span>{message}</span>
            <div>
                <Show when=move || range.is_some()>
                    <ShowLintErrorButton start=range.unwrap().0 end=range.unwrap().1 input_ref />
                </Show>
                <Show when=move || fixer.is_some()>
                    <FixLintErrorButton
                        input_ref
                        start=range.unwrap().0
                        end=range.unwrap().1
                        fixer=fixer.unwrap()
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
    let pixel_ratio = use_device_pixel_ratio();

    let (brand_suggestions, set_brand_suggestions) =
        signal(Vec::<&SimpleIcon>::with_capacity(7));
    let (more_brand_suggestions, set_more_brand_suggestions) =
        signal(Vec::<&SimpleIcon>::new());
    let (show_brand_suggestions, set_show_brand_suggestions) = signal(false);
    let (show_more_brand_suggestions, set_show_more_brand_suggestions) =
        signal(false);

    let input_ref = NodeRef::new();
    _ = on_click_outside(input_ref, move |_| {
        set_show_brand_suggestions(false);
        set_show_more_brand_suggestions(false);
    });

    view! {
        <div class="preview-input-group">
            <label for="preview-brand">{move || tr!("brand")}</label>
            <input
                node_ref=input_ref
                type="text"
                class="mr-7 w-[524px]"
                id="preview-brand"
                value=brand
                prop:value=brand
                on:input=move |ev| {
                    let value = event_target_value::<web_sys::Event>(&ev);
                    let (bs, more_bs) = search_brand_suggestions(&value);
                    let more_bs_length = more_bs.len();
                    set_brand(value.clone());
                    update_preview_canvas(pixel_ratio.get_untracked());
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
                show=Signal::derive(move || {
                    show_brand_suggestions() && !brand_suggestions().is_empty()
                })
                show_more_brand_suggestions
                brand_suggestions
                more_brand_suggestions
                set_brand
                set_color
                set_show_more_brand_suggestions
            />
        </div>
    }
}

#[component]
fn BrandSuggestions(
    show: Signal<bool>,
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
                "preview-brand-suggestions{}{}",
                if show_more_brand_suggestions() { " overflow-y-scroll" } else { "" },
                if show() { "" } else { " hidden" },
            )
        }>

            <For
                each=brand_suggestions
                key=move |icon| icon.slug
                children=move |icon: &'static SimpleIcon| {
                    view! {
                        <BrandSuggestion
                            icon=icon
                            set_brand=set_brand
                            set_color=set_color
                            hidden=Signal::derive(|| false)
                        />
                    }
                }
            />

            <Show when=move || {
                !show_more_brand_suggestions() && !more_brand_suggestions().is_empty()
            }>
                <li
                    class="more-suggestions"
                    role="button"
                    title=move || tr!("load-more-icons")
                    on:click=move |_| {
                        set_show_more_brand_suggestions(true);
                        if let Some(input) = document().get_element_by_id("preview-brand") {
                            _ = input.unchecked_into::<web_sys::HtmlInputElement>().focus();
                        }
                    }
                >

                    <span>+</span>
                </li>
            </Show>
            <For
                each=more_brand_suggestions
                key=move |icon| icon.slug
                children=move |icon| {
                    view! {
                        <BrandSuggestion
                            icon
                            set_brand
                            set_color
                            hidden=Signal::derive(move || !show_more_brand_suggestions())
                        />
                    }
                }
            />
        </ul>
    }
}

#[component]
fn BrandSuggestion(
    icon: &'static SimpleIcon,
    set_brand: WriteSignal<String>,
    set_color: WriteSignal<String>,
    hidden: Signal<bool>,
) -> impl IntoView {
    view! {
        <li
            class:hidden=hidden
            on:click=move |_| {
                set_brand(icon.title.to_string());
                set_color(icon.hex.to_string());
                spawn_local(async move {
                    match fetch_text(&format!("/icons/{}.svg", icon.slug)).await {
                        Ok(svg) => {
                            let path_input = document()
                                .get_element_by_id("preview-path")
                                .unwrap()
                                .unchecked_into::<web_sys::HtmlInputElement>();
                            let path = sdk::svg_to_path(&svg);
                            path_input.set_value(&path);
                            dispatch_input_event_on_input(&path_input);
                        }
                        Err(err) => leptos::logging::error!("{}", err),
                    }
                });
            }
        >
            <span>
                <img src=format!("./icons/{}.svg", icon.slug) width="24px" height="24px" />
                <span>{icon.title}</span>
            </span>
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
