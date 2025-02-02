mod ad;
pub mod icons_loader;
pub(crate) mod item;
mod scroll;

use crate::controls::layout::{Layout, LayoutSignal};
use crate::controls::order::{sort_icons, OrderMode, OrderModeVariant};
use crate::controls::search::search_icons_and_returns_first_page;
use crate::modal::ModalOpen;
use ad::CarbonAdsAdGridItem;
use icons_loader::{IconsLoader, IconsLoaderSignal};
use item::{details::IconDetailsModal, IconGridItem};
use leptos::{
    html::Footer,
    prelude::{NodeRef, *},
};
use leptos_use::use_intersection_observer;
use scroll::ScrollButtons;
use simple_icons_macros::{
    deprecated_icons_array, get_number_of_deprecated_icons,
    get_number_of_icons, icons_array,
};
use simple_icons_website_types::SimpleIcon;
use simple_icons_website_url as Url;
use wasm_bindgen::JsCast;

pub static ICONS: [SimpleIcon; get_number_of_icons!()] = icons_array!();
pub static DEPRECATED_ICONS: [SimpleIcon; get_number_of_deprecated_icons!()] =
    deprecated_icons_array!();

/// Icons rendered in a page
#[derive(Clone)]
pub struct IconsIndexSignal(pub Vec<&'static SimpleIcon>);

/// Icons grid
#[derive(Clone)]
pub struct IconsGrid {
    /// Icons currently loaded
    pub loaded_icons: Vec<&'static SimpleIcon>,
    /// Icons in order of the grid
    pub icons: Vec<&'static SimpleIcon>,
}

impl IconsGrid {
    pub fn new(
        search_value: &str,
        order_mode: &OrderModeVariant,
        layout: &Layout,
        icons: Vec<&'static SimpleIcon>,
    ) -> Self {
        let (icons, loaded_icons) =
            initial_icons_from_search_value_order_mode_and_layout(
                search_value,
                order_mode,
                layout,
                icons,
            );
        Self {
            icons,
            loaded_icons,
        }
    }

    pub fn load_next_icons(&mut self, layout: &Layout) {
        let icons_per_page: usize = layout.icons_per_page() as usize;
        for i in self.loaded_icons.len()..self.icons.len() {
            if self.loaded_icons.len() == self.icons.len() {
                break;
            }
            self.loaded_icons.push(self.icons[i]);
            if self.loaded_icons.len() % icons_per_page == 0 {
                break;
            }
        }
    }
}

/// Signal to control the icons grid
#[derive(Copy, Clone)]
pub struct IconsGridSignal(pub RwSignal<IconsGrid>);

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

fn initial_icons_from_search_value_order_mode_and_layout(
    search_value: &str,
    order_mode: &OrderModeVariant,
    layout: &Layout,
    icons: Vec<&'static SimpleIcon>,
) -> (Vec<&'static SimpleIcon>, Vec<&'static SimpleIcon>) {
    let icons_per_page: usize = layout.icons_per_page() as usize;
    if search_value.is_empty() {
        if order_mode != &OrderModeVariant::Alphabetic {
            let mut icons_copy = icons.clone();
            // Alphabetical is the default order of the icons in the static array
            sort_icons(order_mode, &mut icons_copy);
        }
        let loaded_icons: Vec<&'static SimpleIcon> =
            icons.iter().take(icons_per_page).copied().collect();

        (icons, loaded_icons)
    } else {
        search_icons_and_returns_first_page(search_value, icons_per_page)
    }
}

fn wait_for_first_grid_item_and_open_details(attempt: u32) {
    if let Some(el) = document()
        .query_selector(
            "main > ul > :first-child > :last-child > :nth-child(2)",
        )
        .unwrap()
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
                view! { <IconGridItem icon=icon /> }
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
