use leptos::prelude::RwSignal;
use simple_icons_website_controls_layout_type::Layout;

#[derive(Copy, Clone)]
pub struct LayoutSignal(pub RwSignal<Layout>);
