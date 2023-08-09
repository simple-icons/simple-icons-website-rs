//! Header at the top of the page

pub mod nav;
mod title;

use leptos::*;
use nav::HeaderMenu;
use title::HeaderTitle;

/// State of the header
#[derive(Copy, Clone, Default, Debug)]
pub struct HeaderState {
    /// Indicates if the menu is currently open (only used on tablet and mobile screens)
    pub menu_open: bool,
}

impl HeaderState {
    /// Toggles the menu open state
    pub fn toggle_menu(&mut self) {
        self.menu_open = !self.menu_open;
    }
}

#[derive(Copy, Clone)]
pub struct HeaderStateSignal(pub RwSignal<HeaderState>);

/// Header at the top of the page
#[component]
pub fn Header() -> impl IntoView {
    provide_context(HeaderStateSignal(
        create_rw_signal(HeaderState::default()),
    ));

    view! {
        <header>
            <HeaderTitle/>
            <HeaderMenu/>
        </header>
    }
}
