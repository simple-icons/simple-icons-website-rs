use crate::controls::layout::LayoutSignal;
use crate::grid::IconsGridSignal;
use crate::spinners::TripleDotsSpinner;
use i18n::move_gettext;
use leptos::*;

/// Data structure to control the loading of more icons
#[derive(Clone, Copy)]
pub struct IconsLoader {
    /// If more icons should be loaded currently
    pub load: bool,
    /// If more icons are being currently loaded
    pub loading: bool,
}

impl Default for IconsLoader {
    fn default() -> Self {
        Self {
            load: true,
            loading: false,
        }
    }
}

#[derive(Copy, Clone)]
pub struct IconsLoaderSignal(pub RwSignal<IconsLoader>);

/// Icons loader component
///
/// The icons loader includes:
///
/// - A button to load more icons. This is displayed when the user scrolls
///   to the footer of the page and there are more icons to load.
/// - A loading indicator. This is displayed when more icons are being loaded.
#[component]
pub fn IconsLoader() -> impl IntoView {
    let icons_grid_signal = use_context::<IconsGridSignal>().unwrap().0;
    let icons_loader = use_context::<IconsLoaderSignal>().unwrap().0;
    let layout = use_context::<LayoutSignal>().unwrap().0;

    let hide_load_more_icons_button = move || {
        let loader_state = icons_loader();
        if loader_state.loading {
            return true;
        }

        let icons_grid = icons_grid_signal();
        if icons_grid.loaded_icons.len() == icons_grid.icons.len() {
            return true;
        }
        // if more icons should be loaded currently, then the loader is hidden
        loader_state.load
    };

    // TODO: Currently, we need to set a timeout to display the spinner
    // I suspect that this is happening because the rendering of icon
    // grid items is blocking the main thread
    // See https://stackoverflow.com/q/10180391/9167585
    view! {
        <div class="icons-loader">
            <button
                class:hidden=hide_load_more_icons_button
                on:click=move |_| {
                    icons_loader
                        .update(|state| {
                            state.loading = true;
                        });
                    icons_grid_signal.update(
                        |grid| grid.load_next_icons(&layout())
                    );
                    icons_loader
                        .update(|state| {
                            state.loading = false;
                            state.load = true;
                        });
                }
            >
                {move_gettext!("Load more icons")}
            </button>
            <TripleDotsSpinner hidden_frames=1 hidden=move || !icons_loader().loading/>
        </div>
    }
}
