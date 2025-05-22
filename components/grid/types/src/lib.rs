use leptos::prelude::RwSignal;
use simple_icons_website_controls_layout_type::Layout;
use simple_icons_website_controls_order_types::{sort_icons, OrderModeVariant};
use simple_icons_website_controls_search::search_icons_and_returns_first_page;
use simple_icons_website_types::SimpleIcon;

/// Icons rendered in a page
#[derive(Clone)]
pub struct IconsIndexSignal(pub Vec<&'static SimpleIcon>);

/// Icons grid
#[derive(Clone)]
pub struct IconsGrid {
    /// Icons currently loaded
    pub loaded_icons: Vec<&'static SimpleIcon>,
    /// Icons of the grid
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
        let icons_length = self.icons.len();
        for i in self.loaded_icons.len()..icons_length {
            let loaded_icons_length = self.loaded_icons.len();
            if loaded_icons_length == icons_length {
                break;
            }
            self.loaded_icons.push(self.icons[i]);
            if (loaded_icons_length + 1) % icons_per_page == 0 {
                break;
            }
        }
    }
}

/// Signal to control the icons grid
#[derive(Copy, Clone)]
pub struct IconsGridSignal(pub RwSignal<IconsGrid>);

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
