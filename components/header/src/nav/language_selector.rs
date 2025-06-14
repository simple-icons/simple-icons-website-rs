use crate::{HeaderStateSignal, nav::button::HeaderMenuButton};
use icondata::IoLanguageSharp;
use leptos::prelude::*;
use leptos_fluent::{I18n, Language, move_tr};
use simple_icons_website_modal::{Modal, ModalOpen, ModalOpenSignal};

fn render_language(lang: &'static Language) -> impl IntoView {
    let modal_open = expect_context::<ModalOpenSignal>();
    let i18n = expect_context::<I18n>();

    view! {
        <li
            class:hidden=i18n.language.get() == lang
            on:click=move |_| {
                modal_open.set_none();
                set_timeout(move || i18n.language.set(lang), std::time::Duration::from_millis(0));
            }
        >
            {lang.name}
        </li>
    }
}

/// Languages list
#[component]
pub fn LanguagesList() -> impl IntoView {
    let i18n = expect_context::<I18n>();
    view! {
        <ul class="language-selector">
            {move || {
                i18n.languages.iter().map(|lang| render_language(lang)).collect::<Vec<_>>()
            }}
        </ul>
    }
}

/// Language selector button
#[component]
pub fn LanguageSelectorButton() -> impl IntoView {
    let header_state = expect_context::<HeaderStateSignal>().0;
    let modal_open = expect_context::<ModalOpenSignal>();

    view! {
        <HeaderMenuButton
            on:click=move |_| modal_open.set_languages()
            icon=IoLanguageSharp
            attr:class=move || if header_state().menu_open { "block" } else { "hidden lg:block" }
            attr:title=move_tr!("change-language")
        />
    }
}

/// Language selector
#[component]
pub fn LanguageSelector() -> impl IntoView {
    let modal_open = expect_context::<ModalOpenSignal>();

    view! {
        <LanguageSelectorButton />
        <Modal
            title=move_tr!("select-a-language")
            is_open=Signal::derive(move || modal_open.is_open(ModalOpen::Languages))
            on_close=Signal::derive(move || modal_open.set_none())
            on_close_focus_search_bar=true
        >
            <LanguagesList />
        </Modal>
    }
}
