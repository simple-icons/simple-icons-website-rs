use crate::header::{nav::button::HeaderMenuButton, HeaderStateSignal};
use crate::modal::{Modal, ModalOpen, ModalOpenSignal};
use icondata::IoLanguageSharp;
use leptos::*;
use leptos_fluent::{expect_i18n, move_tr, Language};

fn render_language(lang: &'static Language) -> impl IntoView {
    let modal_open = expect_context::<ModalOpenSignal>();

    view! {
        <li
            class:hidden=lang.is_active()
            on:click=move |_| {
                modal_open.set_none();
                lang.activate();
            }
        >

            {lang.name}
        </li>
    }
}

/// Languages list
#[component]
pub fn LanguagesList() -> impl IntoView {
    view! {
        <ul class="language-selector">
            {move || {
                expect_i18n().languages.iter().map(|lang| render_language(lang)).collect::<Vec<_>>()
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
            title=move_tr!("change-language")
            on:click=move |_| modal_open.set_languages()
            icon=IoLanguageSharp
            class=Signal::derive(move || match header_state().menu_open {
                true => "block".to_string(),
                false => "hidden lg:block".to_string(),
            })
        />
    }
}

/// Language selector
#[component]
pub fn LanguageSelector() -> impl IntoView {
    let modal_open = expect_context::<ModalOpenSignal>();

    view! {
        <LanguageSelectorButton/>
        <Modal
            title=move_tr!("select-a-language")
            is_open=Signal::derive(move || modal_open.0() == Some(ModalOpen::Languages))
            on_close=Signal::derive(move || modal_open.set_none())
            on_close_focus_search_bar=true
        >
            <LanguagesList/>
        </Modal>
    }
}
