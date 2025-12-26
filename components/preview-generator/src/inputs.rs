use crate::{
    Brand, canvas::update_preview_canvas, helpers::is_valid_hex_color,
};
use fast_fuzzy::search;
use leptos::{html::Input, prelude::*, task::spawn_local};
use leptos_fluent::{move_tr, tr};
use leptos_use::{on_click_outside, use_device_pixel_ratio};
use simple_icons::lint::{LintError, LintErrorFixer, errors::PathLintError};
use simple_icons_sdk as sdk;
use simple_icons_website_grid_constants::ICONS;
use simple_icons_website_ids::Ids;
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
                id=Ids::PreviewColor
                value=color
                prop:value=color
                autocomplete="off"
                class:invalid=move || !is_valid_hex_color(&color())
                on:input=move |ev| {
                    let input = event_target::<web_sys::HtmlInputElement>(&ev);
                    let original_value = input.value();
                    let selection_start = input.selection_start().ok().flatten();
                    let selection_end = input.selection_end().ok().flatten();
                    let mut normalized_value = original_value.to_uppercase().replace('#', "");
                    normalized_value.truncate(6);
                    input.set_value(&normalized_value);
                    if let (Some(start), Some(end)) = (selection_start, selection_end) {
                        let hashes_before_start = original_value[..start
                                .min(original_value.len() as u32) as usize]
                            .matches('#')
                            .count() as u32;
                        let hashes_before_end = original_value[..end
                                .min(original_value.len() as u32) as usize]
                            .matches('#')
                            .count() as u32;
                        let new_start = start
                            .saturating_sub(hashes_before_start)
                            .min(normalized_value.len() as u32);
                        let new_end = end
                            .saturating_sub(hashes_before_end)
                            .min(normalized_value.len() as u32);
                        let _ = input.set_selection_start(Some(new_start));
                        let _ = input.set_selection_end(Some(new_end));
                    }
                    set_color(normalized_value);
                    update_preview_canvas(pixel_ratio.get_untracked());
                }
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
        signal::<Vec<LintError>>(vec![]);
    let (show_path_lint_errors, set_show_path_lint_errors) = signal(false);
    let input_ref = NodeRef::new();
    let input_group_ref = NodeRef::new();

    fn process_lint_errors(
        path: &str,
        set_path_lint_errors: WriteSignal<Vec<LintError>>,
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
                id=Ids::PreviewPath
                value=path
                prop:value=path
                autocomplete="off"
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
    let title = move_tr!("show");
    view! {
        <button
            title=title
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
            {title}
        </button>
    }
}

#[component]
fn FixLintErrorButton(
    start: u32,
    end: u32,
    fixer: LintErrorFixer,
    input_ref: NodeRef<Input>,
) -> impl IntoView {
    let title = move_tr!("fix");
    view! {
        <button
            title=title
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
            {title}
        </button>
    }
}

#[component]
fn LintError(
    message: Signal<String>,
    range: Option<(u32, u32)>,
    fixer: Option<LintErrorFixer>,
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

#[derive(Clone)]
struct BrandSuggestionsState {
    brand_suggestions: Vec<&'static SimpleIcon>,
    more_brand_suggestions: Vec<&'static SimpleIcon>,
    input_value: String,
    show_brand_suggestions: bool,
    show_more_brand_suggestions: bool,
}

impl Default for BrandSuggestionsState {
    fn default() -> Self {
        Self {
            brand_suggestions: Vec::with_capacity(
                Self::MAX_MINIMAL_SUGGESTIONS,
            ),
            more_brand_suggestions: Vec::new(),
            input_value: String::new(),
            show_brand_suggestions: false,
            show_more_brand_suggestions: false,
        }
    }
}

impl BrandSuggestionsState {
    const MIN_INPUT_LENGTH: usize = 4;
    const MAX_MINIMAL_SUGGESTIONS: usize = 6;

    fn show_brand_suggestions(&self) -> bool {
        !self.input_value.is_empty() && self.show_brand_suggestions
    }

    fn show_more_brand_suggestions(&self) -> bool {
        self.input_value.len() >= Self::MIN_INPUT_LENGTH
            && !self.more_brand_suggestions.is_empty()
            && self.show_brand_suggestions
            && self.show_more_brand_suggestions
    }
}

#[component]
pub fn BrandInput(set_color: WriteSignal<String>) -> impl IntoView {
    let brand = expect_context::<RwSignal<Brand>>();
    let pixel_ratio = use_device_pixel_ratio();

    let brand_suggestions_state =
        RwSignal::new(BrandSuggestionsState::default());
    provide_context(brand_suggestions_state);

    let container_ref = NodeRef::new();
    _ = on_click_outside(container_ref, move |_| {
        brand_suggestions_state.update(|state| {
            state.show_brand_suggestions = false;
            state.show_more_brand_suggestions = false;
        });
    });

    view! {
        <div node_ref=container_ref class="preview-input-group">
            <label for="preview-brand">{move || tr!("brand")}</label>
            <input
                type="text"
                class="mr-7 w-[524px]"
                id=Ids::PreviewBrand
                value=move || brand().0.clone()
                prop:value=move || brand().0.clone()
                autocomplete="off"
                on:input=move |ev| {
                    let current_brand_slug = &brand().1;
                    let value = event_target_value::<web_sys::Event>(&ev);
                    let (bs, more_bs) = search_brand_suggestions(&value, current_brand_slug);
                    brand
                        .update(|b| {
                            b.0 = value.clone();
                        });
                    update_preview_canvas(pixel_ratio.get_untracked());
                    brand_suggestions_state
                        .update(|state| {
                            state.input_value = value;
                            state.brand_suggestions = bs;
                            state.more_brand_suggestions = more_bs;
                            state.show_brand_suggestions = true;
                            state.show_more_brand_suggestions = false;
                        });
                }
                on:focus=move |ev| {
                    let current_brand_slug = &brand().1;
                    let value = event_target_value::<web_sys::Event>(&ev);
                    let (bs, more_bs) = search_brand_suggestions(&value, current_brand_slug);
                    brand_suggestions_state
                        .update(|state| {
                            state.input_value = value;
                            state.brand_suggestions = bs;
                            state.more_brand_suggestions = more_bs;
                            state.show_brand_suggestions = true;
                        });
                }
            />

            <BrandSuggestions
                show=Signal::derive(move || { brand_suggestions_state().show_brand_suggestions() })
                set_color
            />
        </div>
    }
}

#[component]
fn BrandSuggestions(
    show: Signal<bool>,
    set_color: WriteSignal<String>,
) -> impl IntoView {
    let brand_suggestions_state =
        expect_context::<RwSignal<BrandSuggestionsState>>();

    view! {
        <ul class=move || {
            format!(
                "preview-brand-suggestions{}{}",
                if brand_suggestions_state().show_more_brand_suggestions() {
                    " overflow-y-scroll"
                } else {
                    ""
                },
                if show() { "" } else { " hidden" },
            )
        }>
            {move || {
                let state = brand_suggestions_state();
                if !state.show_brand_suggestions() {
                    return vec![];
                }
                state
                    .brand_suggestions
                    .iter()
                    .map(|icon| {
                        view! { <BrandSuggestion icon set_color /> }
                    })
                    .collect::<Vec<_>>()
            }}
            <Show when=move || {
                let state = brand_suggestions_state();
                !state.more_brand_suggestions.is_empty() && !state.show_more_brand_suggestions()
                    && state.input_value.len() >= BrandSuggestionsState::MIN_INPUT_LENGTH
            }>
                <li
                    class="more-suggestions"
                    role="button"
                    title=move || tr!("load-more-icons")
                    on:click=move |_| {
                        brand_suggestions_state
                            .update(|state| {
                                state.show_brand_suggestions = true;
                                state.show_more_brand_suggestions = true;
                            });
                        if let Some(input) = document().get_element_by_id("preview-brand") {
                            _ = input.unchecked_into::<web_sys::HtmlInputElement>().focus();
                        }
                    }
                >
                    <span>+</span>
                </li>
            </Show>
            {move || {
                let state = brand_suggestions_state();
                if !state.show_more_brand_suggestions() {
                    return vec![];
                }
                state
                    .more_brand_suggestions
                    .into_iter()
                    .map(|icon| view! { <BrandSuggestion icon set_color /> })
                    .collect::<Vec<_>>()
            }}
        </ul>
    }
}

#[component]
fn BrandSuggestion(
    icon: &'static SimpleIcon,
    set_color: WriteSignal<String>,
) -> impl IntoView {
    let brand_suggestions_state =
        expect_context::<RwSignal<BrandSuggestionsState>>();
    let brand = expect_context::<RwSignal<Brand>>();

    view! {
        <li on:click=move |_| {
            brand_suggestions_state
                .update(|state| {
                    state.show_brand_suggestions = false;
                    state.show_more_brand_suggestions = false;
                });
            brand
                .update(|b| {
                    *b = (icon.title.to_string(), icon.slug.to_string());
                });
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
        }>
            <span>
                <img src=format!("./icons/{}.svg", icon.slug) width="24px" height="24px" />
                <span>{icon.title}</span>
            </span>
        </li>
    }
}

fn search_brand_suggestions(
    value: &str,
    current_slug: &str,
) -> (Vec<&'static SimpleIcon>, Vec<&'static SimpleIcon>) {
    let mut initial_icons: Vec<&'static SimpleIcon> =
        Vec::with_capacity(BrandSuggestionsState::MAX_MINIMAL_SUGGESTIONS);
    let mut more_icons: Vec<&'static SimpleIcon> = vec![];
    let search_result = js_sys::Array::from(&search(value));
    let search_result_length = search_result.length();
    for i in 0..search_result_length {
        let result_icon_array = js_sys::Array::from(&search_result.get(i));
        let icon_order_alpha = result_icon_array.get(1).as_f64().unwrap();
        let icon = &ICONS[icon_order_alpha as usize];
        if icon.slug == current_slug {
            continue;
        }
        if initial_icons.len() >= BrandSuggestionsState::MAX_MINIMAL_SUGGESTIONS
        {
            more_icons.push(icon);
        } else {
            initial_icons.push(icon);
        }
    }
    (initial_icons, more_icons)
}
