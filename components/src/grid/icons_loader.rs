use crate::controls::layout::LayoutSignal;
use crate::grid::IconsGridSignal;
use i18n::move_tr;
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

    // TODO: Currently, a spinner can't be displayed  because the rendering
    // of icon grid items is blocking the main thread
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
                    icons_grid_signal.update(|grid| grid.load_next_icons(&layout()));
                    icons_loader
                        .update(|state| {
                            state.loading = false;
                            state.load = true;
                        });
                }
            >

                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="20"
                    height="20"
                    viewBox="0 0 24 24"
                    class="-mt-0.5 mr-0.5"
                >
                    <path d="M0 0h24v24H0z" fill="none"></path>
                    <path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z"></path>
                </svg>
                {move_tr!("load-more-icons")}
            </button>
        </div>
    }
}
