//! App controls

mod button;
pub mod color_scheme;
pub mod download;
pub mod layout;
pub mod order;
pub mod search;

use crate::svg_def::SVGDef;
use color_scheme::ColorSchemeControl;
use download::DownloadFileTypeControl;
use i18n::tr;
use layout::LayoutControl;
use leptos::*;
use order::OrderControl;
use search::SearchControl;

/// State of the controls panel
#[derive(Copy, Clone, Default)]
pub struct ControlsState {
    /// Indicate wheter the buttons group or the search bar is open
    /// (only used on tablet and mobile screens)
    pub buttons_group_open: bool,
}

#[derive(Copy, Clone)]
struct ControlsStateSignal(RwSignal<ControlsState>);

#[component]
pub fn Controls() -> impl IntoView {
    let controls_state = create_rw_signal(ControlsState::default());
    provide_context(ControlsStateSignal(controls_state));

    view! {
        <menu>
            <div class=move || {
                let mut class = "controls-group flex-grow sm:flex-grow-0 mr-4".to_string();
                if controls_state().buttons_group_open {
                    class.push_str(" hidden lg:flex");
                }
                class
            }>
                <SearchControl/>
            </div>
            <div class=move || {
                let mut class = "controls-group".to_string();
                if !controls_state().buttons_group_open {
                    class.push_str(" hidden lg:flex");
                } else {
                    class.push_str(" flex flex-grow");
                }
                class
            }>
                <OrderControl/>
                <ColorSchemeControl/>
                <DownloadFileTypeControl/>
                <LayoutControl/>
            </div>
            <div class=move || {
                let mut class = "relative lg:hidden".to_string();
                if !controls_state().buttons_group_open {
                    class.push_str(" flex-grow");
                }
                class
            }>
                <ControlsToggler/>
            </div>
        </menu>
    }
}

/// Control to toggle between the search bar and the controls buttons
///
/// This is only used on tablet and mobile screens. Renders different icons
/// depending on the state of the controls panel.
#[component]
pub fn ControlsToggler() -> impl IntoView {
    let controls_state = use_context::<ControlsStateSignal>().unwrap().0;
    let toggler_svg_use_id = match controls_state().buttons_group_open {
        true => SVGDef::View.id(),
        false => SVGDef::Controls.id(),
    };

    view! {
        <div class="control">
            <label class="block">""</label>
            <button
                class="absolute right-0 bottom-0 rounded"
                title=move || {
                    if controls_state().buttons_group_open {
                        tr!("open-search-bar")
                    } else {
                        tr!("open-controls")
                    }
                }

                on:click=move |_| {
                    controls_state
                        .update(|state| {
                            state.buttons_group_open = !state.buttons_group_open;
                        });
                }
            >

                <svg role="button" viewBox="0 0 24 24">
                    {move || {
                        view! { <use_ href=format!("#{}", toggler_svg_use_id)></use_> }
                    }}

                </svg>
            </button>
        </div>
    }
}
