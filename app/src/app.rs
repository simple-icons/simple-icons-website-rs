use crate::head::Head;
use crate::pages::Index;
use components::controls::color_scheme::{
    provide_color_scheme_context, ColorScheme,
};
use components::footer::Footer;
use components::header::{
    nav::language_selector::provide_language_context, Header,
};
use components::modal::provide_modal_open_context;
use leptos::{
    document, html::Footer as FooterHtmlElement, provide_context, window, *,
};
use leptos_router::{Route, Router, Routes};
use wasm_bindgen::JsCast;

/// Title of the page
pub static TITLE: &str = "Simple Icons";

/// The main application component
#[component]
pub fn App() -> impl IntoView {
    let color_scheme = provide_color_scheme_context().0;

    create_effect(move |_| {
        let body = document()
            .get_elements_by_tag_name("body")
            .get_with_index(0)
            .unwrap()
            .dyn_into::<web_sys::Element>()
            .unwrap();
        let body_class_list = body.class_list();
        body_class_list.remove_2("dark", "light").unwrap();
        body_class_list
            .add_1(match color_scheme() {
                ColorScheme::Dark => "dark",
                ColorScheme::Light => "light",
                ColorScheme::System => {
                    if window()
                        .match_media("(prefers-color-scheme: dark)")
                        .unwrap()
                        .unwrap()
                        .matches()
                    {
                        "dark"
                    } else {
                        "light"
                    }
                }
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
        html.set_lang(locale_signal().language.to_string().as_str());
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
        <Router>
            <Routes>
                <Route
                    path="/"
                    view=move || {
                        view! { <Index/> }
                    }
                />
            </Routes>
        </Router>
        <Footer container_ref=footer_ref/>
    }
}
