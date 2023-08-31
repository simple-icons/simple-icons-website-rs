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
            // For all other characters, decompose in multiple code
            // points and only keep the alfanumeric ones.
            // See https://unicode.org/reports/tr15/#Canon_Compat_Equivalence
            _ => {
                for codepoint in c.nfd() {
                    if codepoint.is_alphanumeric() {
                        new_title.push(codepoint);
                    }
                }
            }
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
