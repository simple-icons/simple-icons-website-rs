use crate::controls::button::ControlButtonSVGPath;
use crate::controls::layout::{Layout, LayoutSignal};
use crate::controls::search::{
    fire_on_search_event, get_search_value_from_localstorage, SearchValueSignal,
};
use crate::grid::{IconsGrid, IconsGridSignal, ICONS};
use crate::storage::LocalStorage;
use i18n::move_tr;
use leptos::*;
use std::fmt;
use std::str::FromStr;
use types::SimpleIcon;

#[derive(Default, Copy, Clone, PartialEq)]
pub enum OrderModeVariant {
    #[default]
    Alphabetic,
    Color,
    SearchMatch,
}

#[derive(Copy, Clone, Default)]
pub struct OrderMode {
    /// The order mode preferred by the user
    pub favorite: OrderModeVariant,
    /// The order mode currently in use
    pub current: OrderModeVariant,
}

pub fn provide_order_mode_context(initial_search_value: &str) -> OrderMode {
    let initial_order_mode =
        get_order_mode_from_localstorage_and_search_value(initial_search_value);
    provide_context(OrderModeSignal(create_rw_signal(initial_order_mode)));
    initial_order_mode
}

pub fn sort_icons(
    order_mode: &OrderModeVariant,
    icons: &mut [&'static SimpleIcon],
) {
    match order_mode {
        OrderModeVariant::Alphabetic => {
            icons.sort_by(|a, b| a.order_alpha.cmp(&b.order_alpha));
        }
        OrderModeVariant::Color => {
            icons.sort_by(|a, b| a.order_color.cmp(&b.order_color));
        }
        _ => {
            // Search match order is handled by the search control
        }
    }
}

impl From<&str> for OrderModeVariant {
    fn from(order_mode: &str) -> Self {
        match order_mode {
            "alpha" => Self::Alphabetic,
            "color" => Self::Color,
            _ => Self::SearchMatch,
        }
    }
}

impl FromStr for OrderMode {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            favorite: OrderModeVariant::from(value),
            current: OrderModeVariant::from(value),
        })
    }
}

impl fmt::Display for OrderModeVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Alphabetic => write!(f, "alpha"),
            Self::Color => write!(f, "color"),
            Self::SearchMatch => write!(f, "search"),
        }
    }
}

#[derive(Copy, Clone)]
pub struct OrderModeSignal(pub RwSignal<OrderMode>);

fn get_order_mode_from_localstorage() -> Option<OrderMode> {
    LocalStorage::get(LocalStorage::Keys::OrderMode)
        .as_ref()
        .and_then(|value| OrderMode::from_str(value).ok())
}

fn get_order_mode_from_localstorage_and_search_value(
    search_value: &str,
) -> OrderMode {
    let order_mode = get_order_mode_from_localstorage();
    if !search_value.is_empty() {
        let mut order = order_mode.unwrap();
        order.current = OrderModeVariant::SearchMatch;
        return order;
    }
    match order_mode {
        Some(order_mode) => order_mode,
        None => OrderMode::default(),
    }
}

fn set_order_mode_on_localstorage(order_mode: &OrderModeVariant) {
    LocalStorage::set(LocalStorage::Keys::OrderMode, &order_mode.to_string());
}

pub fn set_order_mode(
    order_mode: &OrderModeVariant,
    order_mode_signal: &RwSignal<OrderMode>,
    icons_grid_signal: &RwSignal<IconsGrid>,
    layout_signal: Option<&Layout>,
    update_grid: bool,
) {
    order_mode_signal.update(move |state| {
        state.current = *order_mode;
        if *order_mode != OrderModeVariant::SearchMatch {
            state.favorite = *order_mode;
            set_order_mode_on_localstorage(order_mode);
        }
    });
    if update_grid {
        match order_mode {
            &OrderModeVariant::Alphabetic | &OrderModeVariant::Color => {
                icons_grid_signal.update(|grid| {
                    let search_value = get_search_value_from_localstorage();
                    if search_value.is_some() {
                        // If we are searching, just update the order of the current
                        // icons
                        sort_icons(order_mode, &mut grid.icons);
                        sort_icons(order_mode, &mut grid.loaded_icons);
                    } else {
                        // If not searching, load the new icons in order
                        grid.icons = ICONS.iter().collect();
                        grid.loaded_icons = Vec::with_capacity(ICONS.len());
                        sort_icons(order_mode, &mut grid.icons);
                        if let Some(layout_signal) = layout_signal {
                            grid.load_next_icons(layout_signal);
                        }
                    }
                });
            }
            &OrderModeVariant::SearchMatch => {
                // Fire a search event to update the grid
                fire_on_search_event();
            }
        }
    }
}

#[component]
pub fn OrderControl() -> impl IntoView {
    let order_mode = expect_context::<OrderModeSignal>().0;
    let icons_grid = expect_context::<IconsGridSignal>().0;
    let search_signal = expect_context::<SearchValueSignal>().0;
    let layout_signal = expect_context::<LayoutSignal>().0;

    let render_buttons = move || {
        let mut buttons = vec![
            view! {
                <ControlButtonSVGPath
                    title=move_tr!("sort-alphabetically")
                    svg_path="M10.096 18.857H7.525V.429A.414.414 0 0 0 7.096 0H4.525a.414.414 0 0 0-.429.429v18.429H1.525c-.196 0-.331.089-.402.268-.072.17-.04.326.094.469l4.286 4.286c.098.079.2.119.308.119.116 0 .219-.04.308-.12l4.272-4.272a.506.506 0 0 0 .134-.321.414.414 0 0 0-.429-.43zm10.006 3.617H16.78c-.188 0-.322.009-.402.026l-.188.026V22.5l.148-.147c.133-.16.227-.276.281-.348l4.941-7.099v-1.191h-7.594v3.066h1.607v-1.54h3.107c.16 0 .295-.014.4-.04a.856.856 0 0 0 .102-.007c.039-.004.068-.007.086-.007v.04l-.146.121c-.08.08-.176.2-.281.361L13.9 22.795V24h7.82v-3.12h-1.619v1.594h.001zm1.875-13.608L18.895 0h-2.168l-3.082 8.866h-.936v1.419h3.842V8.866h-1.004l.631-1.929h3.254l.629 1.929h-1.004v1.419h3.857V8.866h-.937zm-5.358-3.402.977-2.92c.037-.107.07-.236.102-.388s.047-.232.047-.241l.039-.268h.055c0 .036.008.125.025.268l.162.629.963 2.92h-2.37z"
                    active=move || order_mode().current == OrderModeVariant::Alphabetic
                    on:click=move |_| set_order_mode(
                        &OrderModeVariant::Alphabetic,
                        &order_mode,
                        &icons_grid,
                        Some(&layout_signal()),
                        true,
                    )
                />
            },
            view! {
                <ControlButtonSVGPath
                    title=move_tr!("sort-by-color")
                    svg_path="M3.44 0a.42.42 0 0 0-.31.121.42.42 0 0 0-.12.309v18.427H.44a.4.4 0 0 0-.403.268c-.07.17-.04.326.094.469l4.287 4.287c.098.08.199.119.307.119a.449.449 0 0 0 .308-.12l4.272-4.273a.502.502 0 0 0 .134-.318.418.418 0 0 0-.43-.432H6.44V.43A.415.415 0 0 0 6.01 0H3.44zm14.738 4.178c-2.082.008-4.25.926-5.875 2.69a7.722 7.722 0 0 0 .445 10.91 7.722 7.722 0 0 0 10.912-.444 1.287 1.287 0 0 0-.074-1.818 1.26 1.26 0 0 0-.857-.336 1.333 1.333 0 0 1-.854-.342 1.287 1.287 0 0 1-.074-1.818l1.03-1.118a4.29 4.29 0 0 0-.247-6.06c-1.22-1.125-2.787-1.67-4.406-1.664zm-2.063 1.869a1.287 1.287 0 0 1 .723.332 1.287 1.287 0 0 1 .074 1.818 1.287 1.287 0 0 1-1.818.074 1.287 1.287 0 0 1-.074-1.818 1.287 1.287 0 0 1 1.095-.406zm4.268.433a1.287 1.287 0 0 1 .722.332 1.287 1.287 0 0 1 .075 1.819 1.287 1.287 0 0 1-1.819.074 1.287 1.287 0 0 1-.074-1.818 1.287 1.287 0 0 1 1.096-.407zm-7.176 2.721a1.287 1.287 0 0 1 .723.332 1.287 1.287 0 0 1 .074 1.819 1.287 1.287 0 0 1-1.818.074 1.287 1.287 0 0 1-.075-1.819 1.287 1.287 0 0 1 1.096-.406zm.78 4.219a1.287 1.287 0 0 1 .722.332 1.287 1.287 0 0 1 .074 1.818 1.287 1.287 0 0 1-1.818.075 1.287 1.287 0 0 1-.074-1.819 1.287 1.287 0 0 1 1.095-.406z"
                    active=move || order_mode().current == OrderModeVariant::Color
                    on:click=move |_| set_order_mode(
                        &OrderModeVariant::Color,
                        &order_mode,
                        &icons_grid,
                        Some(&layout_signal()),
                        true,
                    )
                />
            },
        ];
        if !search_signal().is_empty() {
            buttons
                .push(
                    view! {
                        <ControlButtonSVGPath
                            title=move_tr!("sort-by-search-match")
                            svg_path="M1.226 7.709q-.521 0-.874-.353T0 6.483q0-.521.353-.874.353-.354.873-.352h3.678q.521 0 .874.353.354.353.352.873 0 .52-.353.874-.353.353-.873.352zm0 6.13q-.521 0-.874-.353T0 12.613q0-.521.353-.874t.873-.352h3.678q.521 0 .874.353.354.353.352.873 0 .521-.353.874t-.873.352zm20.72 5.272-3.862-3.862q-.735.521-1.61.782-.874.262-1.761.26-2.545 0-4.338-1.794-1.794-1.793-1.793-4.336 0-2.544 1.794-4.338t4.337-1.792q2.544 0 4.337 1.793 1.794 1.794 1.793 4.337 0 .889-.261 1.763-.262.874-.781 1.609l3.862 3.862q.337.337.337.858 0 .521-.337.858-.338.338-.859.338-.52 0-.858-.338zm-7.233-5.272q1.532 0 2.605-1.073t1.073-2.605q0-1.533-1.073-2.605-1.073-1.073-2.605-1.073-1.533 0-2.606 1.073-1.073 1.072-1.073 2.605 0 1.532 1.073 2.605t2.606 1.073zM1.226 19.97q-.521 0-.874-.353T0 18.743q0-.52.353-.874.353-.353.873-.352h9.809q.52 0 .874.354.353.353.351.872 0 .522-.353.875t-.873.351z"
                            active=move || order_mode().current == OrderModeVariant::SearchMatch
                            on:click=move |_| set_order_mode(
                                &OrderModeVariant::SearchMatch,
                                &order_mode,
                                &icons_grid,
                                Some(&layout_signal()),
                                true,
                            )
                        />
                    },
                )
        }
        buttons
    };

    view! {
        <div class="control">
            <label>{move_tr!("order")}</label>
            <div class="flex flex-row">{render_buttons}</div>
        </div>
    }
}
