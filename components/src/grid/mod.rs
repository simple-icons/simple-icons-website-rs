mod ad;
pub mod icons_loader;
pub(crate) mod item;
mod scroll;

use crate::controls::layout::{Layout, LayoutSignal};
use crate::controls::order::{sort_icons, OrderMode, OrderModeVariant};
use crate::controls::search::search_icons_and_returns_first_page;
use icons_loader::{IconsLoader, IconsLoaderSignal};
use item::{details::IconDetailsModal, IconGridItem};
use leptos::{
    html::{Footer, HtmlElement},
    NodeRef, *,
};
use macros::{get_number_of_icons, icons_array};
use scroll::ScrollButtons;
use types::SimpleIcon;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{
    IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit,
};

pub const ICONS: [SimpleIcon; get_number_of_icons!()] = icons_array!();

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
    ) -> Self {
        let (icons, loaded_icons) =
            initial_icons_from_search_value_order_mode_and_layout(
                search_value,
                order_mode,
                layout,
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
) {
    provide_context(IconsGridSignal(create_rw_signal(IconsGrid::new(
        initial_search_value,
        &initial_order_mode.current,
        initial_layout,
    ))));
    provide_context(IconsLoaderSignal(
        create_rw_signal(IconsLoader::default()),
    ));
}

fn initial_icons_from_search_value_order_mode_and_layout(
    search_value: &str,
    order_mode: &OrderModeVariant,
    layout: &Layout,
) -> (Vec<&'static SimpleIcon>, Vec<&'static SimpleIcon>) {
    let icons_per_page: usize = layout.icons_per_page() as usize;
    if search_value.is_empty() {
        let mut icons: Vec<&'static SimpleIcon> = ICONS.iter().collect();
        if order_mode != &OrderModeVariant::Alphabetic {
            // Alphabetical is the default order of the icons in the static array
            sort_icons(order_mode, &mut icons);
        }
        let loaded_icons: Vec<&'static SimpleIcon> =
            icons.iter().take(icons_per_page).copied().collect();

        (icons, loaded_icons)
    } else {
        search_icons_and_returns_first_page(search_value, icons_per_page)
    }
}

/// Icons grid
///
/// The icons grid items are lazy loaded with pagination. The first page is
/// loaded on the first render. The next pages are loaded when the user
/// scrolls to the footer. See the `IntersectionObserver` used inside the
/// `Footer` component.
#[component]
pub fn Icons() -> impl IntoView {
    let icons_grid = use_context::<IconsGridSignal>().unwrap().0;

    view! {
        {move || {
            icons_grid()
                .loaded_icons
                .iter()
                .map(|icon: &&'static SimpleIcon| {
                    view! { <IconGridItem icon=*icon/> }
                })
                .collect::<Vec<_>>()
        }}
    }
}

/// Main grid
///
/// Includes the Carbon Ads ad and the icons
///
/// When the user scrolls nearly to the footer, the next page of icons are loaded.
/// This is accomplished by using an `IntersectionObserver`.
#[component]
pub fn Grid() -> impl IntoView {
    // Get layout view signal
    let layout = use_context::<LayoutSignal>().unwrap().0;

    // Provide the context for the current icon details view
    provide_context(CurrentIconViewSignal(create_rw_signal(None)));

    // Get the context of the page footer node reference and, when the footer element
    // has been created, load the intersection callback for loading more icons when
    // the viewport of the screen intersects into the footer
    let footer_ref = use_context::<NodeRef<Footer>>().unwrap();
    footer_ref.on_load(move |footer: HtmlElement<Footer>| {
        let icons_grid = use_context::<IconsGridSignal>().unwrap().0;
        let icons_loader: RwSignal<IconsLoader> =
            use_context::<IconsLoaderSignal>().unwrap().0;

        let intersection_callback: Closure<
            dyn Fn(Vec<IntersectionObserverEntry>, IntersectionObserver),
        > = Closure::wrap(Box::new(
            move |entries: Vec<IntersectionObserverEntry>,
                  _observer: IntersectionObserver| {
                let footer_entry = &entries[0];

                if footer_entry.is_intersecting() {
                    if icons_loader().load {
                        icons_grid
                            .update(|grid| grid.load_next_icons(&layout()));
                    }
                } else if !icons_loader().load {
                    icons_loader.update(|state| state.load = true);
                }
            },
        ));

        let intersection_observer = IntersectionObserver::new_with_options(
            intersection_callback.as_ref().unchecked_ref(),
            // 300px before the footer is reached, load the next page
            IntersectionObserverInit::new().root_margin("300px 0px 0px 0px"),
        )
        .unwrap();
        intersection_observer.observe(&footer);

        // TODO: this is a memory leak
        // https://rustwasm.github.io/docs/wasm-bindgen/examples/closures.html
        // See https://stackoverflow.com/a/68944438/9167585 for the possible solution
        // Seems so much complicated for such a small thing, not a priority
        intersection_callback.forget();
    });

    view! {
        <IconDetailsModal/>
        <ul class:layout-compact=move || layout() == Layout::Compact>
            <Icons/>
        </ul>
        <IconsLoader/>
        <ScrollButtons/>
    }
}
