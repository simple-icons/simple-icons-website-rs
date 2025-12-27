mod head;

use head::Head;
use leptos::{html::Footer as FooterHtmlElement, prelude::*};
use leptos_use::{
    ColorMode, UseColorModeOptions, UseColorModeReturn,
    use_color_mode_with_options,
};
use simple_icons_website_controls::{
    Controls, download::provide_download_type_context,
    layout::provide_layout_context, order::provide_order_mode_context,
    search::provide_search_context,
};
use simple_icons_website_footer::Footer;
use simple_icons_website_grid::{Grid, provide_icons_grid_contexts};
use simple_icons_website_grid_types::IconsIndexSignal;
use simple_icons_website_header::Header;
use simple_icons_website_i18n::I18n;
use simple_icons_website_modal::provide_modal_open_context;
use simple_icons_website_storage::LocalStorage;
use simple_icons_website_svg_defs::SVGDefsDefinition;
use simple_icons_website_url as Url;

/// Title of the page
pub static TITLE: &str = "Simple Icons";

/// The main application component
#[component]
pub fn SimpleIconsApp(
    children: Children,
    #[prop(default = TITLE)] title: &'static str,
) -> impl IntoView {
    let UseColorModeReturn {
        mode: color_mode,
        set_mode: set_color_mode,
        ..
    } = use_color_mode_with_options(
        UseColorModeOptions::default()
            .storage_enabled(true)
            .storage(leptos_use::storage::StorageType::Local)
            .storage_key(LocalStorage::Keys::ColorScheme.as_str())
            .target("body")
            .attribute("class")
            .emit_auto(true)
            .initial_value_from_url_param(
                Url::params::Names::ColorScheme.as_str(),
            )
            .initial_value_from_url_param_to_storage(true),
    );

    provide_context::<Signal<ColorMode>>(color_mode);
    provide_context::<WriteSignal<ColorMode>>(set_color_mode);

    // Create a context to store a node reference to the footer
    // to use it in other components of pages
    let footer_ref = NodeRef::new();
    provide_context::<NodeRef<FooterHtmlElement>>(footer_ref);

    // Create a context to store the current opened modal
    provide_modal_open_context();

    let main_ref = NodeRef::new();

    view! {
        <I18n
            url_param=Url::params::Names::Language.as_str()
            local_storage_key=LocalStorage::Keys::Language.as_str()
        >
            <Head title />
            <Header />
            <SVGDefsDefinition />
            <main node_ref=main_ref>{children()}</main>
            <Footer container_ref=footer_ref />
        </I18n>
    }
}

#[component]
pub fn Index() -> AnyView {
    let icons = expect_context::<IconsIndexSignal>().0;
    let initial_search_value = provide_search_context(icons.clone());
    let initial_order_mode = provide_order_mode_context(&initial_search_value);
    provide_download_type_context();
    let initial_layout = provide_layout_context();

    provide_icons_grid_contexts(
        &initial_search_value,
        &initial_order_mode,
        &initial_layout,
        icons,
    );

    view! {
        <Controls />
        <Grid />
    }
    .into_any()
}
