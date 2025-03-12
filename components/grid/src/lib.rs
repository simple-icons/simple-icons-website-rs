#![feature(stmt_expr_attributes)]

mod ad;
pub(crate) mod item;
mod scroll;

use ad::CarbonAdsAdGridItem;
use item::{details::IconDetailsModal, IconGridItem};
use leptos::{
    html::Footer,
    prelude::{NodeRef, *},
};
use leptos_use::use_intersection_observer;
use scroll::ScrollButtons;
use simple_icons_website_controls_layout_types::{Layout, LayoutSignal};
use simple_icons_website_controls_order_types::OrderMode;
use simple_icons_website_grid_icons_loader::{IconsLoader, IconsLoaderSignal};
use simple_icons_website_grid_types::{IconsGrid, IconsGridSignal};
use simple_icons_website_modal::ModalOpen;
use simple_icons_website_types::SimpleIcon;
use simple_icons_website_url as Url;
use wasm_bindgen::JsCast;

/// Signal to control the current detail view modal of icons
#[derive(Copy, Clone)]
pub struct CurrentIconViewSignal(pub RwSignal<Option<&'static SimpleIcon>>);

pub fn provide_icons_grid_contexts(
    initial_search_value: &str,
    initial_order_mode: &OrderMode,
    initial_layout: &Layout,
    icons: Vec<&'static SimpleIcon>,
) {
    provide_context(IconsGridSignal(RwSignal::new(IconsGrid::new(
        initial_search_value,
        &initial_order_mode.current,
        initial_layout,
        icons,
    ))));
    provide_context(IconsLoaderSignal(RwSignal::new(IconsLoader::default())));
}

fn wait_for_first_grid_item_and_open_details(attempt: u32) {
    if let Some(el) = document()
        .query_selector(
            "main > ul > :nth-child(2) > :last-child > :nth-child(2)",
        )
        .unwrap_or(None)
    {
        el.unchecked_into::<web_sys::HtmlElement>().click();
    } else if attempt < 2000 {
        set_timeout(
            move || wait_for_first_grid_item_and_open_details(attempt + 1),
            std::time::Duration::from_millis(5),
        );
    }
}

/// Icons grid
///
/// The icons grid items are lazy loaded with pagination. The first page is
/// loaded on the first render. The next pages are loaded when the user
/// scrolls to the footer.
#[component]
pub fn Icons() -> impl IntoView {
    let icons_grid = expect_context::<IconsGridSignal>().0;

    view! {
        <For
            each=move || icons_grid().loaded_icons
            key=move |icon| icon.slug
            children=move |icon: &'static SimpleIcon| {
                view! { <IconGridItem icon /> }
            }
        />
    }
}

/// Main grid
///
/// Includes the Carbon Ads ad and the icons
///
/// When the user scrolls nearly to the footer, the next page of icons
/// are loaded. This is accomplished by using an `IntersectionObserver`,
/// see [`use_intersection_observer`](https://leptos-use.rs/elements/use_intersection_observer.html).
#[component]
pub fn Grid() -> impl IntoView {
    // Get layout view signal
    let layout = expect_context::<LayoutSignal>().0;

    // Provide the context for the current icon details view
    provide_context(CurrentIconViewSignal(RwSignal::new(None)));

    let icons_grid = expect_context::<IconsGridSignal>().0;
    let icons_loader: RwSignal<IconsLoader> =
        expect_context::<IconsLoaderSignal>().0;

    let footer_ref = expect_context::<NodeRef<Footer>>();
    use_intersection_observer(footer_ref, move |entries, _| {
        let footer_entry = &entries[0];

        if footer_entry.is_intersecting() {
            if icons_loader.get_untracked().load {
                icons_grid.update(|grid| {
                    grid.load_next_icons(&layout.get_untracked())
                });
            }
        } else if !icons_loader.get_untracked().load {
            icons_loader.update(|state| state.load = true);
        }
    });

    let icons_list_ref = NodeRef::new();
    icons_list_ref.on_load(move |_| {
        let modal_param = Url::params::get(&Url::params::Names::Modal);
        if modal_param == Some(ModalOpen::Icon.to_string()) {
            wait_for_first_grid_item_and_open_details(1);
        }
    });

    view! {
        <IconDetailsModal />
        <ul node_ref=icons_list_ref class:layout-compact=move || layout() == Layout::Compact>
            <CarbonAdsAdGridItem />
            <Icons />
        </ul>
        <IconsLoader />
        <ScrollButtons />
    }
}
