extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::quote;
use std::path::PathBuf;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    parse_macro_input, token, Ident, Result,
};

struct I18nLoader {
    locales_ident: syn::Ident,
    languages_json_file: PathBuf,
}

impl Parse for I18nLoader {
    fn parse(input: ParseStream) -> Result<Self> {
        let workspace_path = std::path::PathBuf::from(
            std::env::var("CARGO_MANIFEST_DIR")
                .unwrap_or_else(|_| String::from("./")),
        );

        let fields;
        braced!(fields in input);
        let mut locales_identifier: Option<syn::Ident> = None;
        let mut languages_json_path: Option<syn::LitStr> = None;

        while !fields.is_empty() {
            let k = fields.parse::<Ident>()?;
            fields.parse::<syn::Token![:]>()?;

            //if k == "customise" {
            //    customise = Some(fields.parse()?);
            //} else if k == "core_locales" {
            //    core_locales = Some(fields.parse()?);
            //} else if k == "fallback_language" {
            //    fallback_language = Some(fields.parse()?);
            if k == "locales" {
                locales_identifier = Some(fields.parse()?);
            } else if k == "languages_json" {
                languages_json_path = Some(fields.parse()?);
            } else {
                return Err(syn::Error::new(k.span(), "Not a valid parameter"));
            }

            if fields.is_empty() {
                break;
            }
            fields.parse::<token::Comma>()?;
        }

        // languages_json
        let languages_json = languages_json_path.ok_or_else(|| {
            syn::Error::new(input.span(), "Missing `languages_json` field")
        })?;

        let languages_json_file = workspace_path.join(languages_json.value());

        if std::fs::metadata(&languages_json_file).is_err() {
            return Err(syn::Error::new(languages_json.span(), format!("Couldn't read languages.json file, this path should be relative to your crate's `Cargo.toml`. Looking for: {:?}", languages_json_file)));
        }

        let locales_ident = locales_identifier.ok_or_else(|| {
            syn::Error::new(input.span(), "Missing `locales` field")
        })?;

        Ok(Self {
            locales_ident,
            languages_json_file,
        })
    }
}

#[proc_macro]
pub fn leptos_fluent_i18n(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let I18nLoader {
        locales_ident,
        languages_json_file,
    } = parse_macro_input!(input as I18nLoader);

    let languages = serde_json::from_str::<Vec<Vec<String>>>(
        std::fs::read_to_string(languages_json_file)
            .expect("Couldn't read languages.json file")
            .as_str(),
    )
    .expect("Invalid JSON")
    .iter()
    .map(|lang| (lang[0].clone(), lang[1].clone()))
    .collect::<Vec<(String, String)>>();

    let languages_quote = format!(
        "[{}]",
        languages
            .iter()
            .map(|(id, name)| {
                format!(
                    "&::leptos_fluent_i18n::Language{{ id: ::unic_langid::langid!(\"{}\"), name: \"{}\" }}",
                    id, name
                )
            })
            .collect::<Vec<String>>()
            .join(",")
    ).parse::<TokenStream>().unwrap();
    let n_languages = languages.len();

    let quote = quote! {
        const LANGUAGES: [
            &::leptos_fluent_i18n::Language; #n_languages
        ] = #languages_quote;
        ::leptos_fluent_i18n::I18n {
            language: ::std::rc::Rc::new(
                ::leptos_fluent_i18n::LanguageSignal(
                    ::leptos::create_rw_signal(LANGUAGES[0])
                )
            ),
            languages: &LANGUAGES,
            locales: &#locales_ident,
        }
    };

    // println!("{}", quote);
    proc_macro::TokenStream::from(quote)
}
