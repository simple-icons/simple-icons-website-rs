use crate::controls::button::ControlButtonSVGPath;
use crate::storage::LocalStorage;
use crate::Url;
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
        None => match layout_from_localstorage() {
            Some(layout) => layout,
            None => Layout::default(),
        },
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
                <ControlButtonSVGPath
                    title=move_tr!("comfortable")
                    svg_path="M19 2a2 2 0 0 1 2 2v2a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h14zm0 4V4H5v2h14zm0 10a2 2 0 0 1 2 2v2a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-2a2 2 0 0 1 2-2h14zm0 4v-2H5v2h14zm0-11a2 2 0 0 1 2 2v2a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-2a2 2 0 0 1 2-2h14zm0 4v-2H5v2h14z"
                    active=Signal::derive(move || layout() == Layout::Comfortable)
                    on:click=move |_| set_layout(Layout::Comfortable, &layout)
                />
                <ControlButtonSVGPath
                    title=move_tr!("compact")
                    svg_path="M2 5.5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v13a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2v-13zm9 0H4v3h7v-3zm2 0v3h7v-3h-7zm7 5h-7v3h7v-3zm0 5h-7v3h7v-3zm-9 3v-3H4v3h7zm-7-5h7v-3H4v3z"
                    active=Signal::derive(move || layout() == Layout::Compact)
                    on:click=move |_| set_layout(Layout::Compact, &layout)
                />
            </div>
        </div>
    }
}
