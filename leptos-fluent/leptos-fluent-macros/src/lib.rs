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
    sync_html_tag_lang: bool,
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
        let mut sync_html_tag_lang_litbool: Option<syn::LitBool> = None;

        while !fields.is_empty() {
            let k = fields.parse::<Ident>()?;
            fields.parse::<syn::Token![:]>()?;

            if k == "locales" {
                locales_identifier = Some(fields.parse()?);
            } else if k == "languages_json" {
                languages_json_path = Some(fields.parse()?);
            } else if k == "sync_html_tag_lang" {
                sync_html_tag_lang_litbool = Some(fields.parse()?);
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
            sync_html_tag_lang: match sync_html_tag_lang_litbool {
                Some(lit) => lit.value,
                None => false,
            },
        })
    }
}

#[proc_macro]
pub fn leptos_fluent(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let I18nLoader {
        locales_ident,
        languages_json_file,
        sync_html_tag_lang,
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
                    concat!(
                        "&::leptos_fluent::Language{{",
                        " id: ::unic_langid::langid!(\"{}\"),",
                        " name: \"{}\"",
                        " }}",
                    ),
                    id, name
                )
            })
            .collect::<Vec<String>>()
            .join(",")
    )
    .parse::<TokenStream>()
    .unwrap();
    let n_languages = languages.len();

    let sync_html_tag_lang_quote = if sync_html_tag_lang {
        quote! {
            use leptos::wasm_bindgen::JsCast;
            ::leptos::create_effect(move |_| ::leptos::document()
                .document_element()
                .unwrap()
                .unchecked_into::<::web_sys::HtmlHtmlElement>()
                .set_lang(
                    &::leptos::expect_context::<::leptos_fluent::I18n>().language.get().id.to_string())
            );
        }
    } else {
        quote! {}
    };

    let quote = quote! {
        const LANGUAGES: [
            &::leptos_fluent::Language; #n_languages
        ] = #languages_quote;
        let i18n = ::leptos_fluent::I18n {
            language: ::std::rc::Rc::new(
                ::leptos_fluent::LanguageSignal(
                    ::leptos::create_rw_signal(LANGUAGES[0])
                )
            ),
            languages: &LANGUAGES,
            locales: &#locales_ident,
        };
        #sync_html_tag_lang_quote;
        i18n
    };

    // println!("{}", quote);
    proc_macro::TokenStream::from(quote)
}
