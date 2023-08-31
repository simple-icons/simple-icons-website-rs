mod data;
mod deprecated;

pub use data::{
    get_simple_icons_data, SimpleIconData, SimpleIconDataAliases,
    SimpleIconDataLicense,
};
pub use deprecated::{fetch_deprecated_simple_icons, IconDeprecation};
use unicode_normalization::UnicodeNormalization;

fn title_to_slug_replace_chars(title: &str) -> String {
    let mut new_title = String::with_capacity(title.len());
    for c in title.chars() {
        match c {
            'a'..='z' | '0'..='9' => new_title.push(c),
            '+' => new_title.push_str("plus"),
            '.' => new_title.push_str("dot"),
            '&' => new_title.push_str("and"),
            'đ' => new_title.push('d'),
            'ħ' => new_title.push('h'),
            'ı' => new_title.push('i'),
            'ĸ' => new_title.push('k'),
            'ŀ' => new_title.push('l'),
            'ł' => new_title.push('l'),
            'ß' => new_title.push_str("ss"),
            'ŧ' => new_title.push('t'),
            // The next implementation differs from the one in Javascript
            // TODO: should this be reported to the unicode_normalization
            // crate? Investigate
            'á' | 'à' | 'ä' | 'ã' => new_title.push('a'),
            'é' | 'è' | 'ë' => new_title.push('e'),
            'í' | 'ì' | 'ï' => new_title.push('i'),
            'ó' | 'ò' | 'ö' => new_title.push('o'),
            'ú' | 'ù' | 'ü' => new_title.push('u'),
            'š' => new_title.push('s'),
            _ => continue,
        }
    }
    new_title
}

/// Convert a brand title to slug
pub fn title_to_slug(title: &str) -> String {
    title_to_slug_replace_chars(&title.to_lowercase())
        .nfd()
        .collect::<String>()
}
