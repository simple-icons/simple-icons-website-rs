use fluent_templates::{
    fluent_bundle::FluentValue, loader::langid, static_loader,
    LanguageIdentifier, Loader,
};
use leptos::*;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Clone)]
pub struct Language {
    pub language: LanguageIdentifier,
    pub name: &'static str,
}

pub static LANGUAGES: [Language; 5] = [
    Language {
        language: langid!("en-US"),
        name: "English",
    },
    Language {
        language: langid!("es-ES"),
        name: "Español",
    },
    Language {
        language: langid!("fr-FR"),
        name: "Français",
    },
    Language {
        language: langid!("it-IT"),
        name: "Italiano",
    },
    Language {
        language: langid!("pt-PT"),
        name: "Português",
    },
    /*
    Language {
        code: "ja-JP",
        name: "日本語",
    },
    Language {
        code: "zh-CN",
        name: "中文 (简体)",
    },
    Language {
        code: "zh-HK",
        name: "中文 (香港)",
    },
    Language {
        code: "zh-TW",
        name: "中文 (繁體)",
    }, */
];

static_loader! {
    // Declare our `StaticLoader` named `LOCALES`.
    static LOCALES = {
        // The directory of localisations and fluent resources.
        locales: "../i18n/locales",
        // The language to falback on if something is not present.
        fallback_language: "en-US",
    };
}

impl PartialEq for Language {
    fn eq(&self, other: &Self) -> bool {
        self.language == other.language
    }
}

impl Language {
    pub fn translate(&self, key: &'static str) -> String {
        LOCALES.lookup(&self.language, key).unwrap()
    }

    pub fn translate_with_args(
        &self,
        key: &'static str,
        args: &HashMap<String, FluentValue<'_>>,
    ) -> String {
        LOCALES.lookup_with_args(&self.language, key, args).unwrap()
    }
}

impl Default for Language {
    fn default() -> Self {
        LANGUAGES[0].clone()
    }
}

impl FromStr for Language {
    type Err = ();

    fn from_str(code: &str) -> Result<Self, Self::Err> {
        match LANGUAGES
            .iter()
            .find(|lang| lang.language.to_string() == *code)
        {
            Some(lang) => Ok(lang.clone()),
            None => Err(()),
        }
    }
}

#[derive(Copy, Clone)]
pub struct LocaleSignal(pub RwSignal<Language>);

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
///
/// ```
#[macro_export]
macro_rules! tr {
    ($key:expr) => {
        ((&use_context::<::i18n::LocaleSignal>().unwrap().0)().translate($key))
            .to_string()
    };
    ($key:expr, $args:expr) => {
        ((&use_context::<::i18n::LocaleSignal>().unwrap().0)()
            .translate_with_args($key, $args))
        .to_string()
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
        move || $crate::tr!($key)
    };
    ($key:expr, $args:expr) => {
        move || $crate::tr!($key, $args)
    };
}
