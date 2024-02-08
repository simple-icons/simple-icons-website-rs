use crate::header::{nav::button::HeaderMenuButton, HeaderStateSignal};
use crate::modal::{Modal, ModalOpen, ModalOpenSignal};
use crate::storage::LocalStorage;
use crate::Url;
use leptos::{window, *};
use leptos_fluent::{I18n, Language};

static LANGUAGE_SELECTOR_ICON_SVG_PATH: &str = concat!(
    "m12.87 15.07-2.54-2.51.03-.03A17.52 17.52 0 0 0 14.07 6H17V4h-7V2H8v2",
    "H1v1.99h11.17C11.5 7.92 10.44 9.75 9 11.35 8.07 10.32 7.3 9.19 6.69 8",
    "h-2c.73 1.63 1.73 3.17 2.98 4.56l-5.09 5.02L4 19l5-5 3.11 3.11.76-2.04z",
    "M18.5 10h-2L12 22h2l1.12-3h4.75L21 22h2l-4.5-12zm-2.62 7 1.62-4.33",
    "L19.12 17h-3.24z",
);

pub fn initial_language(i18n: &I18n) -> &'static Language {
    match Url::params::get(&Url::params::Names::Language)
        .and_then(|value| i18n.language_from_str(value.as_str()))
    {
        Some(value) => {
            set_language_in_localstorage(value);
            value
        }
        None => match initial_language_from_localstorage(i18n) {
            Some(lang) => lang,
            None => match initial_language_from_navigator_languages(i18n) {
                Some(lang) => lang,
                None => i18n.default_language(),
            },
        },
    }
}

fn initial_language_from_navigator_languages(
    i18n: &I18n,
) -> Option<&'static Language> {
    let languages = window().navigator().languages().to_vec();
    for raw_language in languages {
        let language =
            raw_language.as_string().expect("Language not parseable");
        if let Some(lang) = i18n.language_from_str(language.as_str()) {
            return Some(lang);
        }
    }
    None
}

fn initial_language_from_localstorage(
    i18n: &I18n,
) -> Option<&'static Language> {
    LocalStorage::get(LocalStorage::Keys::Language)
        .and_then(|value| i18n.language_from_str(value.as_str()))
}

pub fn set_language_in_localstorage(lang: &'static Language) {
    LocalStorage::set(LocalStorage::Keys::Language, &lang.id.to_string());
}

/// Languages list
#[component]
pub fn LanguagesList() -> impl IntoView {
    let modal_open = expect_context::<ModalOpenSignal>();
    let i18n = expect_context::<I18n>();
    let current_language = Signal::derive(move || i18n.language.get());

    view! {
        <ul class="language-selector">
            <For
                each=move || i18n.languages
                key=move |lang| lang.id.to_string()
                children=move |lang: &&Language| {
                    view! {
                        <li
                            class=move || if *lang == current_language() { "hidden" } else { "" }
                            on:click=move |_| {
                                modal_open.set_none();
                                expect_context::<I18n>().language.set(lang);
                                set_language_in_localstorage(lang);
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
    let i18n = expect_context::<I18n>();

    view! {
        <HeaderMenuButton
            title=Signal::derive(move || i18n.tr("change-language"))
            on:click=move |_| modal_open.set_languages()
            svg_path=LANGUAGE_SELECTOR_ICON_SVG_PATH
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
    let i18n = expect_context::<I18n>();

    view! {
        <LanguageSelectorButton/>
        <Modal
            title=Signal::derive(move || i18n.tr("select-a-language"))
            is_open=Signal::derive(move || modal_open.0() == Some(ModalOpen::Languages))
            on_close=Signal::derive(move || modal_open.set_none())
            on_close_focus_search_bar=true
        >
            <LanguagesList/>
        </Modal>
    }
}
