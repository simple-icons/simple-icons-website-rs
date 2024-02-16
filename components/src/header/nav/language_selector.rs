use crate::header::{nav::button::HeaderMenuButton, HeaderStateSignal};
use crate::modal::{Modal, ModalOpen, ModalOpenSignal};
use icondata::IoLanguageSharp;
use leptos::*;
use leptos_fluent::{i18n, move_tr, Language};

/// Languages list
#[component]
pub fn LanguagesList() -> impl IntoView {
    let modal_open = expect_context::<ModalOpenSignal>();
    let current_language = Signal::derive(move || i18n().language.get());

    view! {
        <ul class="language-selector">
            <For
                each=move || i18n().languages
                key=move |lang| lang.id.to_string()
                children=move |lang: &&Language| {
                    view! {
                        <li
                            class=move || if *lang == current_language() { "hidden" } else { "" }
                            on:click=move |_| {
                                modal_open.set_none();
                                i18n().set_language_with_localstorage(lang);
                            }
                        >

                            {lang.name}
                        </li>
                    }
                }
            />

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
