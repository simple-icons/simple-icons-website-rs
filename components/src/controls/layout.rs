use super::button::ControlButtonIcon;
use crate::grid::icons_loader::{IconsLoader, IconsLoaderSignal};
use icondata::{LuGrid2x2, LuGrid3x3};
use leptos::prelude::*;
use leptos_fluent::move_tr;
use simple_icons_website_storage::LocalStorage;
use simple_icons_website_url as Url;
use std::str::FromStr;

#[derive(Default, Copy, Clone, PartialEq)]
pub enum Layout {
    #[default]
    Comfortable,
    Compact,
}

impl Layout {
    pub fn icons_per_page(&self) -> u32 {
        match self {
            Self::Comfortable => 30,
            Self::Compact => 60,
        }
    }
}

impl FromStr for Layout {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "comfortable" => Ok(Self::Comfortable),
            "compact" => Ok(Self::Compact),
            _ => Err(()),
        }
    }
}

impl core::fmt::Display for Layout {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Comfortable => write!(f, "comfortable"),
            Self::Compact => write!(f, "compact"),
        }
    }
}

#[derive(Copy, Clone)]
pub struct LayoutSignal(pub RwSignal<Layout>);

pub fn provide_layout_context() -> Layout {
    let layout = initial_layout();
    provide_context(LayoutSignal(RwSignal::new(layout)));
    layout
}

fn initial_layout() -> Layout {
    match Url::params::get(&Url::params::Names::Layout)
        .and_then(|layout| layout.parse().ok())
    {
        Some(layout) => {
            set_layout_on_localstorage(&layout);
            layout
        }
        None => layout_from_localstorage().unwrap_or_default(),
    }
}

fn layout_from_localstorage() -> Option<Layout> {
    LocalStorage::get(LocalStorage::Keys::Layout)
        .as_ref()
        .and_then(|value| Layout::from_str(value).ok())
}

fn set_layout_on_localstorage(layout: &Layout) {
    LocalStorage::set(LocalStorage::Keys::Layout, &layout.to_string())
}

fn set_layout(layout: Layout, layout_signal: &RwSignal<Layout>) {
    layout_signal.update(move |state| *state = layout);
    set_layout_on_localstorage(&layout);
}

#[component]
pub fn LayoutControl() -> impl IntoView {
    let layout = expect_context::<LayoutSignal>().0;

    let icons_loader: RwSignal<IconsLoader> =
        expect_context::<IconsLoaderSignal>().0;

    let activate_compact_layout = move || {
        set_layout(Layout::Compact, &layout);

        // load more icons if needed, see:
        // https://github.com/simple-icons/simple-icons-website-rs/issues/290
        if icons_loader.get_untracked().load {
            let page_height = document()
                .query_selector("body")
                .unwrap()
                .unwrap()
                .client_height() as f64;
            let scroll_y = window().scroll_y().unwrap_or_default();
            let window_height = window()
                .inner_height()
                .unwrap_or_default()
                .as_f64()
                .unwrap_or_default();
            let footer = document().query_selector("footer").unwrap().unwrap();
            let footer_height = footer.client_height() as f64;

            let separation = 800.0;
            let mut cutoff = page_height;
            if footer_height < cutoff {
                cutoff -= footer_height;
            }
            if cutoff > separation {
                cutoff -= separation;
            }

            if scroll_y + window_height >= cutoff {
                footer.scroll_into_view();
                icons_loader.update(|state| state.load = false);
            }
        }
    };

    view! {
        <div class="control">
            <label>{move_tr!("layout")}</label>
            <div>
                <ControlButtonIcon
                    title=move_tr!("comfortable")
                    icon=LuGrid2x2
                    active=Signal::derive(move || layout() == Layout::Comfortable)
                    on:click=move |_| {
                        if layout.get_untracked() != Layout::Comfortable {
                            set_layout(Layout::Comfortable, &layout);
                        }
                    }
                />
                <ControlButtonIcon
                    title=move_tr!("compact")
                    icon=LuGrid3x3
                    active=Signal::derive(move || layout() == Layout::Compact)
                    on:click=move |_| {
                        if layout.get_untracked() != Layout::Compact {
                            activate_compact_layout();
                        }
                    }
                />
            </div>
        </div>
    }
}
