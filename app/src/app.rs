use crate::head::Head;
use crate::pages::{AllIconsIndex, DeprecationsIndex, Error404, Preview};
use leptos::{html::Footer as FooterHtmlElement, prelude::*};
use leptos_router::{
    components::{FlatRoutes, Route, Router},
    StaticSegment,
};
use leptos_use::{
    use_color_mode_with_options, ColorMode, UseColorModeOptions,
    UseColorModeReturn,
};
use simple_icons_website_footer::Footer;
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
pub fn App() -> impl IntoView {
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

    provide_context::<(Signal<ColorMode>, WriteSignal<ColorMode>)>((
        color_mode,
        set_color_mode,
    ));

    // Create a context to store a node reference to the footer
    // to use it in other components of pages
    let footer_ref = NodeRef::new();
    provide_context::<NodeRef<FooterHtmlElement>>(footer_ref);

    // Create a context to store the current opened modal
    provide_modal_open_context();

    // Create a context to store keyboard shortcuts
    let main_ref = NodeRef::new();

    view! {
        <I18n
            url_param=Url::params::Names::Language.as_str()
            localstorage_key=LocalStorage::Keys::Language.as_str()
        >
            <Head />
            <Header />
            <SVGDefsDefinition />
            <main node_ref=main_ref>
                <Router>
                    <FlatRoutes fallback=Error404>
                        <Route path=StaticSegment("/preview") view=Preview />
                        <Route path=StaticSegment("/deprecations") view=DeprecationsIndex />
                        <Route path=StaticSegment("/") view=AllIconsIndex />
                    </FlatRoutes>
                </Router>
            </main>
            <Footer container_ref=footer_ref />
        </I18n>
    }
}
