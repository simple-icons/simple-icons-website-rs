use crate::head::Head;
use crate::pages::{Error404, Index, Preview};
use components::controls::color_scheme::{
    provide_color_scheme_context, ColorScheme,
};
use components::copy::CopyInput;
use components::footer::Footer;
use components::header::{
    nav::language_selector::provide_language_context, Header,
};
use components::modal::provide_modal_open_context;
use components::svg::SVGDefsDefinition;
use leptos::{html::Footer as FooterHtmlElement, *};
use leptos_router::{Route, Router, Routes};
use leptos_use::use_preferred_dark;
use wasm_bindgen::JsCast;

/// Title of the page
pub static TITLE: &str = "Simple Icons";

/// The main application component
#[component]
pub fn App() -> impl IntoView {
    let color_scheme = provide_color_scheme_context().0;
    let dark_preferred = use_preferred_dark();

    create_effect(move |_| {
        let body_class_list = document().body().unwrap().class_list();
        body_class_list.remove_2("dark", "light").unwrap();
        body_class_list
            .add_1(match color_scheme() {
                ColorScheme::Dark => "dark",
                ColorScheme::Light => "light",
                ColorScheme::System => match dark_preferred() {
                    true => "dark",
                    false => "light",
                },
            })
            .unwrap();
    });

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
                    <Route path="/" view=Index/>

                    <Route path="/*any" view=Error404/>
                </Routes>
            </Router>
        </main>
        <Footer container_ref=footer_ref/>
    }
}
