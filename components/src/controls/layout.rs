use crate::controls::button::ControlButtonIcon;
use crate::storage::LocalStorage;
use crate::Url;
use icondata::{LuGrid2x2, LuGrid3x3};
use leptos::*;
use leptos_fluent::move_tr;
use simple_icons_website_config::CONFIG;
use std::fmt;
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
            Self::Comfortable => CONFIG
                .read()
                .unwrap()
                .get("icons_per_page_comfortable")
                .unwrap(),
            Self::Compact => CONFIG
                .read()
                .unwrap()
                .get("icons_per_page_compact")
                .unwrap(),
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

impl fmt::Display for Layout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
    provide_context(LayoutSignal(create_rw_signal(layout)));
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

    view! {
        <div class="control">
            <label>{move_tr!("layout")}</label>
            <div>
                <ControlButtonIcon
                    title=move_tr!("comfortable")
                    icon=LuGrid2x2
                    active=Signal::derive(move || layout() == Layout::Comfortable)
                    on:click=move |_| set_layout(Layout::Comfortable, &layout)
                />
                <ControlButtonIcon
                    title=move_tr!("compact")
                    icon=LuGrid3x3
                    active=Signal::derive(move || layout() == Layout::Compact)
                    on:click=move |_| set_layout(Layout::Compact, &layout)
                />
            </div>
        </div>
    }
}
