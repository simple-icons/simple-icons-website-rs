extern crate proc_macro;

use core::str::FromStr;
use fluent_templates::{
    fluent_bundle::FluentValue, loader::Loader, LanguageIdentifier,
    StaticLoader,
};
use leptos::{
    provide_context, RwSignal, SignalGet, SignalGetUntracked, SignalSet,
};
pub use leptos_fluent_i18n_macros::leptos_fluent_i18n;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone)]
pub struct Language {
    pub id: LanguageIdentifier,
    pub name: &'static str,
}

impl PartialEq for Language {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Copy, Clone)]
pub struct LanguageSignal(pub RwSignal<&'static Language>);

impl SignalGet for LanguageSignal {
    type Value = &'static Language;

    fn get(&self) -> Self::Value {
        self.0.get()
    }

    fn try_get(&self) -> Option<Self::Value> {
        self.0.try_get()
    }
}

impl SignalGetUntracked for LanguageSignal {
    type Value = &'static Language;

    fn get_untracked(&self) -> Self::Value {
        self.0.get_untracked()
    }

    fn try_get_untracked(&self) -> Option<Self::Value> {
        self.0.try_get_untracked()
    }
}

impl SignalSet for LanguageSignal {
    type Value = &'static Language;

    fn set(&self, value: Self::Value) {
        self.0.set(value);
    }

    fn try_set(&self, value: Self::Value) -> Option<Self::Value> {
        self.0.try_set(value)
    }
}

pub struct I18n {
    pub language: Rc<LanguageSignal>,
    pub languages: &'static [&'static Language],
    pub locales: &'static Lazy<StaticLoader>,
}

impl Clone for I18n {
    fn clone(&self) -> Self {
        Self {
            language: Rc::clone(&self.language),
            languages: self.languages,
            locales: self.locales,
        }
    }
}

impl I18n {
    pub fn provide_context(&self, initial_language: &'static Language) {
        self.language.0.set(initial_language);
        provide_context::<I18n>(self.clone());
    }

    pub fn tr(&self, key: &str) -> String {
        let lang_id = &self.language.0().id;
        self.locales.lookup(lang_id, key).unwrap_or_else(|| {
            panic!(
                "Translation for key '{}' not found in locale '{}'",
                key, lang_id
            )
        })
    }

    pub fn trs(
        &self,
        key: &str,
        args: &HashMap<String, FluentValue<'_>>,
    ) -> String {
        let lang_id = &self.language.0().id;
        self.locales
            .lookup_with_args(lang_id, key, args)
            .unwrap_or_else(|| {
                panic!(
                    "Translation for key '{}' not found in locale '{}'",
                    key, lang_id
                )
            })
    }

    pub fn default_language(&self) -> &'static Language {
        self.languages[0]
    }

    pub fn language_from_str(&self, code: &str) -> Option<&'static Language> {
        match LanguageIdentifier::from_str(code) {
            Ok(target_lang) => match self
                .languages
                .iter()
                .find(|lang| lang.id.matches(&target_lang, false, false))
            {
                Some(lang) => Some(lang),
                None => {
                    let mut lazy_target_lang = target_lang.clone();
                    lazy_target_lang.region = None;
                    match self.languages.iter().find(|lang| {
                        lang.id.matches(&lazy_target_lang, true, true)
                    }) {
                        Some(lang) => Some(lang),
                        None => None,
                    }
                }
            },
            Err(_) => None,
        }
    }
}
