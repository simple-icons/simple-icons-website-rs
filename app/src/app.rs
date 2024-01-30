use crate::head::Head;
use crate::pages::{AllIconsIndex, DeprecationsIndex, Error404, Preview};
use components::controls::color_scheme::initial_color_scheme;
use components::copy::CopyInput;
use components::footer::Footer;
use components::header::{
    nav::language_selector::provide_language_context, Header,
};
use components::modal::provide_modal_open_context;
use components::storage::LocalStorage;
use components::svg::SVGDefsDefinition;
use leptos::{html::Footer as FooterHtmlElement, wasm_bindgen::JsCast, *};
use leptos_router::{Route, Router, Routes};
use leptos_use::{
    use_color_mode_with_options, ColorMode, UseColorModeOptions,
    UseColorModeReturn,
};

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
            .storage_key(LocalStorage::Keys::ColorScheme.as_str())
            .target(document().body().unwrap())
            .attribute("class")
            .emit_auto(true)
            .initial_value(initial_color_scheme()),
    );

    provide_context::<(Signal<ColorMode>, WriteSignal<ColorMode>)>((
        color_mode,
        set_color_mode,
    ));

    let locale_signal = provide_language_context().0;
    create_effect(move |_| {
        let html = document()
            .document_element()
            .unwrap()
            .dyn_into::<web_sys::HtmlHtmlElement>()
            .unwrap();
        html.set_lang(&locale_signal().id.to_string());
    });

    // Create a context to store a node reference to the footer
    // to use it in other components of pages
    let footer_ref = create_node_ref::<FooterHtmlElement>();
    provide_context::<NodeRef<FooterHtmlElement>>(footer_ref);

    // Create a context to store the current opened modal
    provide_modal_open_context();

    view! {
        <Head/>
        <Header/>
        <SVGDefsDefinition/>
        <CopyInput/>
        <main>
            <Router>
                <Routes>
                    <Route path="/preview" view=Preview/>
                    <Route path="/deprecations" view=DeprecationsIndex/>
                    <Route path="/" view=AllIconsIndex/>
                    <Route path="/*any" view=Error404/>
                </Routes>
            </Router>
        </main>
        <Footer container_ref=footer_ref/>
    }
}
