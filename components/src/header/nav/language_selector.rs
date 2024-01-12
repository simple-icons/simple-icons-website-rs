use crate::header::{nav::button::HeaderMenuButton, HeaderStateSignal};
use crate::modal::{Modal, ModalOpen, ModalOpenSignal};
use crate::storage::LocalStorage;
use crate::Url;
use i18n::{move_tr, Language, LocaleSignal, LANGUAGES};
use leptos::{window, *};
use std::str::FromStr;

static LANGUAGE_SELECTOR_ICON_SVG_PATH: &str = concat!(
    "m12.87 15.07-2.54-2.51.03-.03A17.52 17.52 0 0 0 14.07 6H17V4h-7V2H8v2",
    "H1v1.99h11.17C11.5 7.92 10.44 9.75 9 11.35 8.07 10.32 7.3 9.19 6.69 8",
    "h-2c.73 1.63 1.73 3.17 2.98 4.56l-5.09 5.02L4 19l5-5 3.11 3.11.76-2.04z",
    "M18.5 10h-2L12 22h2l1.12-3h4.75L21 22h2l-4.5-12zm-2.62 7 1.62-4.33",
    "L19.12 17h-3.24z",
);

pub fn provide_language_context() -> LocaleSignal {
    let locale_signal = LocaleSignal(create_rw_signal(initial_language()));
    provide_context(locale_signal);
    locale_signal
}

fn initial_language() -> &'static Language {
    match Url::params::get(&Url::params::Names::Language)
        .and_then(|value| <&'static Language>::from_str(value.as_str()).ok())
    {
        Some(value) => {
            set_language_in_localstorage(value);
            value
        }
        None => match initial_language_from_localstorage() {
            Some(lang) => lang,
            None => match initial_language_from_navigator_languages() {
                Some(lang) => lang,
                None => <&'static Language>::default(),
            },
        },
    }
}

fn initial_language_from_navigator_languages() -> Option<&'static Language> {
    let languages = window().navigator().languages().to_vec();
    for raw_language in languages {
        let language =
            raw_language.as_string().expect("Language not parseable");
        if let Ok(lang) = language.parse() {
            return Some(lang);
        }
    }
    None
}

fn initial_language_from_localstorage() -> Option<&'static Language> {
    LocalStorage::get(LocalStorage::Keys::Language)
        .and_then(|value| <&'static Language>::from_str(value.as_str()).ok())
}

pub fn set_language_in_localstorage(lang: &'static Language) {
    LocalStorage::set(LocalStorage::Keys::Language, &lang.id.to_string());
}

/// Languages list
#[component]
pub fn LanguagesList() -> impl IntoView {
    let locale_state = expect_context::<LocaleSignal>().0;
    let modal_open = expect_context::<ModalOpenSignal>();

    view! {
        <ul class="language-selector">
            <For
                each=move || LANGUAGES.iter()
                key=move |lang| lang.id.to_string()
                children=move |lang: &Language| {
                    view! {
                        <Show when=move || lang != locale_state()>
                            <li on:click=move |_| {
                                modal_open.set_none();
                                locale_state.update(|state| *state = lang);
                                set_language_in_localstorage(lang);
                            }>

                                {lang.name}
                            </li>
                        </Show>
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
