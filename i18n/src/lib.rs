use fluent_templates::loader::Loader;
use fluent_templates::{
    fluent_bundle::FluentValue, static_loader, LanguageIdentifier,
};
use leptos::*;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Clone)]
pub struct Language {
    pub id: LanguageIdentifier,
    pub name: &'static str,
}

// Exposes LANGUAGES as a public static variable
include!(concat!(env!("OUT_DIR"), "/languages.rs"));

static_loader! {
    // Declare our `StaticLoader` named `LOCALES`.
    pub static LOCALES = {
        // The directory of localisations and fluent resources.
        locales: "../i18n/locales",
        // The language to falback on if something is not present.
        fallback_language: "en-US",
        customise: |bundle| bundle.set_use_isolating(false),
    };
}

impl PartialEq for Language {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Default for &'static Language {
    fn default() -> Self {
        &LANGUAGES[0]
    }
}

impl FromStr for &'static Language {
    type Err = ();

    fn from_str(code: &str) -> Result<Self, Self::Err> {
        match LanguageIdentifier::from_str(code) {
            Ok(target_lang) => match LANGUAGES
                .iter()
                .find(|lang| lang.id.matches(&target_lang, false, false))
            {
                Some(lang) => Ok(lang),
                None => {
                    let mut lazy_target_lang = target_lang.clone();
                    lazy_target_lang.region = None;
                    match LANGUAGES.iter().find(|lang| {
                        lang.id.matches(&lazy_target_lang, true, true)
                    }) {
                        Some(lang) => Ok(lang),
                        None => Err(()),
                    }
                }
            },
            Err(_) => Err(()),
        }
    }
}

#[derive(Copy, Clone)]
pub struct LocaleSignal(pub RwSignal<&'static Language>);

pub fn lookup(key: &'static str) -> String {
    let lang_id = &expect_context::<LocaleSignal>().0().id;
    LOCALES.lookup(lang_id, key).unwrap_or_else(|| {
        panic!(
            "Translation for key '{}' not found in locale '{}'",
            key, lang_id
        )
    })
}

pub fn lookup_with_args(
    key: &'static str,
    args: &HashMap<String, FluentValue<'_>>,
) -> String {
    let lang_id = &expect_context::<LocaleSignal>().0().id;
    LOCALES
        .lookup_with_args(lang_id, key, args)
        .unwrap_or_else(|| {
            panic!(
                "Translation for key '{}' not found in locale '{}'",
                key, lang_id
            )
        })
}

/// Macro to translate strings in the website
///
/// Use it like this:
///
/// ```rust,ignore
/// <p>{move || tr!("hello-world")}</p>
/// ```
///
/// You need to wrap in a `move` closure because is the way that Leptos
/// has to know that the string is reactive.
#[macro_export]
macro_rules! tr {
    ($key:expr) => {
        $crate::lookup($key)
    };
    ($key:expr, $args:expr) => {
        $crate::lookup_with_args($key, $args)
    };
}

/// Macro to generate a closure that returns a translated string
///
/// Convenient wrapper for Leptos interactivity closures.
///
/// Use it like this:
/// ```rust,ignore
/// <p>{move_tr!("hello-world")}</p>
/// ```
///
/// The previous code is the same as:
/// ```rust,ignore
/// <p>{move || tr!("hello-world")}</p>
/// ```
#[macro_export]
macro_rules! move_tr {
    ($key:expr) => {
        Signal::derive(move || $crate::tr!($key))
    };
    ($key:expr, $args:expr) => {
        Signal::derive(move || $crate::tr!($key, $args))
    };
}
