//! Macros for simple-icons-website
//!
//! These macros are used to generate code at compile time.

use proc_macro::TokenStream;
use simple_icons::{
    color::{is_relatively_light_icon_hex, sort_hexes},
    get_simple_icon_svg_path, get_simple_icons,
    sdk::fetch_deprecated_simple_icons,
};
use simple_icons_website_config::CONFIG;
use std::fs;
use std::path::Path;
use syn::{parse_macro_input, LitStr};

fn get_max_icons_from_config() -> Option<usize> {
    CONFIG
        .read()
        .unwrap()
        .get::<Option<usize>>("max_icons")
        .unwrap()
}

/// Get number of icons available in the simple-icons npm package
#[proc_macro]
pub fn get_number_of_icons(_: TokenStream) -> TokenStream {
    let max_icons = get_max_icons_from_config();
    if let Some(max_icons) = max_icons {
        max_icons.to_string().parse().unwrap()
    } else {
        Path::new("node_modules/simple-icons/icons")
            .read_dir()
            .unwrap()
            .count()
            .to_string()
            .parse()
            .unwrap()
    }
}

/// Get number of deprecated icons available in the simple-icons npm package
#[proc_macro]
pub fn get_number_of_deprecated_icons(_: TokenStream) -> TokenStream {
    fetch_deprecated_simple_icons()
        .len()
        .to_string()
        .parse()
        .unwrap()
}

fn get_simple_icons_3rd_party_extensions_libraries_impl(
    section_name: &'static str,
) -> TokenStream {
    let readme_file_content =
        fs::read_to_string(Path::new("node_modules/simple-icons/README.md"))
            .unwrap();

    let mut extensions_array_code = "&[".to_string();

    let table_lines = readme_file_content
        .split_once(section_name)
        .unwrap()
        .1
        .split("|\n\n")
        .next()
        .unwrap()
        .split("|\n|")
        .skip(2);

    for line in table_lines {
        if line.trim().is_empty() {
            break;
        }

        let name = line.split_once('[').unwrap().1.split_once("](").unwrap().0;
        let url = line.split_once("](").unwrap().1.split_once(')').unwrap().0;

        let author_part = line.split_once('|').unwrap().1;
        let author_name = author_part
            .split_once('[')
            .unwrap()
            .1
            .split_once(']')
            .unwrap()
            .0;
        let author_url = author_part
            .split_once("](")
            .unwrap()
            .1
            .split_once(')')
            .unwrap()
            .0;
        let icon_slug = line
            .split_once("<img src=\"")
            .unwrap()
            .1
            .split_once('"')
            .unwrap()
            .0
            .split('/')
            .last()
            .unwrap()
            .split_once(".svg")
            .unwrap()
            .0;

        extensions_array_code.push_str(&format!(
            concat!(
                "::types::ThirdPartyExtension{{",
                "name: \"{}\",",
                "url: \"{}\",",
                "author_name: \"{}\",",
                "author_url: \"{}\",",
                "icon_slug: \"{}\",",
                "}},"
            ),
            name,
            url,
            author_name,
            author_url,
            get_simple_icon_svg_path(icon_slug),
        ));
    }

    extensions_array_code.push(']');
    extensions_array_code.parse().unwrap()
}

/// Get the extensions of Simple Icons from the README file of the npm package
#[proc_macro]
pub fn get_simple_icons_3rd_party_extensions(_: TokenStream) -> TokenStream {
    get_simple_icons_3rd_party_extensions_libraries_impl(
        "Third-Party Extensions",
    )
}

#[proc_macro]
pub fn get_simple_icons_3rd_party_libraries(_: TokenStream) -> TokenStream {
    get_simple_icons_3rd_party_extensions_libraries_impl(
        "Third-Party Libraries",
    )
}

fn icons_array_impl(only_include_deprecated: bool) -> String {
    let max_icons = get_max_icons_from_config();
    let simple_icons = get_simple_icons(max_icons);

    let hexes = simple_icons
        .iter()
        .map(|icon| icon.hex.clone())
        .collect::<Vec<_>>();
    let sorted_hexes = sort_hexes(&hexes);

    let deprecated_icons = fetch_deprecated_simple_icons();

    let mut icons_array_code = "[".to_string();
    for (i, icon) in simple_icons.iter().enumerate() {
        // color order index
        let order_color = sorted_hexes
            .iter()
            .position(|hex| *hex == icon.hex)
            .unwrap();

        let deprecated_icon = deprecated_icons
            .iter()
            .find(|deprecated_icon| *deprecated_icon.slug == icon.slug);

        if only_include_deprecated && deprecated_icon.is_none() {
            continue;
        }

        let icon_code = &format!(
            concat!(
                "::types::SimpleIcon{{",
                "slug: \"{}\",",
                "title: \"{}\",",
                "hex: \"{}\",",
                "hex_is_relatively_light: {},",
                "source: \"{}\",",
                "guidelines: {},",
                "license_url: {},",
                "license_type: {},",
                "aliases: {},",
                // `get_simple_icons` function returns icons in alphabetical order
                // because they are extracted from the `simple-icons.json` file
                "order_alpha: {},",
                "order_color: {},",
                "deprecation: {},",
                "}},"
            ),
            icon.slug,
            icon.title,
            icon.hex,
            is_relatively_light_icon_hex(&icon.hex),
            icon.source,
            match icon.guidelines {
                Some(ref url) => format!("Some(\"{}\")", url),
                None => "None".to_string(),
            },
            match icon.license {
                Some(ref license) => match license.url {
                    Some(ref url) => format!("Some(\"{}\")", url),
                    None => "None".to_string(),
                },
                None => "None".to_string(),
            },
            match icon.license {
                Some(ref license) => format!("Some(\"{}\")", license.type_),
                None => "None".to_string(),
            },
            match &icon.aliases {
                None => "None".to_string(),
                Some(aliases) => format!(
                    "Some(&::types::SimpleIconAliases {{{}, {}, {}}})",
                    {
                        &format!(
                            "aka: {}",
                            match &aliases.aka {
                                Some(aka) => format!(
                                    "Some(&[{}])",
                                    aka.iter()
                                        .map(|aka| format!("\"{}\"", aka))
                                        .collect::<Vec<_>>()
                                        .join(", ")
                                ),
                                None => "None".to_string(),
                            }
                        )
                    },
                    {
                        &format!(
                            "dup: {}",
                            match &aliases.dup {
                                Some(dup) => format!(
                                    "Some(&[{}])",
                                    dup.iter()
                                        .map(|dup| format!(
                                            "\"{}\"",
                                            dup.title.clone()
                                        ))
                                        .collect::<Vec<_>>()
                                        .join(", ")
                                ),
                                None => "None".to_string(),
                            }
                        )
                    },
                    {
                        &format!("loc: {}", {
                            let mut result = "".to_string();
                            let mut alias_dup_locs: Vec<(String, String)> =
                                vec![];
                            if aliases.dup.is_some() {
                                for alias_dup in aliases.dup.as_ref().unwrap() {
                                    if alias_dup.loc.is_some() {
                                        alias_dup_locs.extend(
                                            alias_dup
                                                .loc
                                                .as_ref()
                                                .unwrap()
                                                .iter()
                                                .map(|(lang, title)| {
                                                    (
                                                        lang.clone(),
                                                        title.clone(),
                                                    )
                                                }),
                                        );
                                    }
                                }
                            }

                            if aliases.loc.is_some()
                                || !alias_dup_locs.is_empty()
                            {
                                result.push_str("Some(&[");
                                if aliases.loc.is_some() {
                                    result.push_str(
                                        &aliases
                                            .loc
                                            .as_ref()
                                            .unwrap()
                                            .iter()
                                            .map(|(lang, title)| {
                                                format!(
                                                    "(\"{}\", \"{}\")",
                                                    lang, title
                                                )
                                            })
                                            .collect::<Vec<_>>()
                                            .join(", "),
                                    );
                                }
                                if !alias_dup_locs.is_empty() {
                                    if aliases.loc.is_some() {
                                        result.push_str(", ");
                                    }
                                    for (lang, title) in alias_dup_locs {
                                        result.push_str(&format!(
                                            "(\"{}\", \"{}\"),",
                                            lang, title
                                        ));
                                    }
                                }
                                result.push_str("])");
                            } else {
                                result.push_str("None");
                            }
                            result
                        })
                    }
                ),
            },
            i,
            order_color,
            match deprecated_icon {
                Some(icon) => {
                    format!(
                        concat!(
                            "Some(",
                            "&::types::IconDeprecation{{",
                            "removal_at_version: \"{}\",",
                            "milestone_number: {},",
                            "milestone_due_on: \"{}\",",
                            "pull_request_number: {},",
                            "}}",
                            ")",
                        ),
                        icon.removal_at_version,
                        icon.milestone_number,
                        icon.milestone_due_on,
                        icon.pull_request_number,
                    )
                }
                None => "None".to_string(),
            },
        );
        icons_array_code.push_str(icon_code)
    }
    icons_array_code.push(']');

    icons_array_code
}

#[proc_macro]
pub fn icons_array(_: TokenStream) -> TokenStream {
    icons_array_impl(false).parse().unwrap()
}

#[proc_macro]
pub fn deprecated_icons_array(_: TokenStream) -> TokenStream {
    icons_array_impl(true).parse().unwrap()
}

/// Get JS library version from package.json
#[proc_macro]
pub fn js_library_version(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let path = format!("node_modules/{}/package.json", input.value());
    let package_json_content = fs::read_to_string(Path::new(&path)).unwrap();
    let package_json: serde_json::Value =
        serde_json::from_str(package_json_content.as_str()).unwrap();
    let version = package_json["version"].as_str().unwrap();
    format!("\"{}\"", version).parse().unwrap()
}
