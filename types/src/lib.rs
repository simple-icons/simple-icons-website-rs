mod deprecation;
mod third_party_extension;

pub use deprecation::IconDeprecation;
pub use third_party_extension::ThirdPartyExtension;

#[derive(Clone)]
pub struct SimpleIconAliases {
    pub aka: Option<&'static [&'static str]>,
    pub dup: Option<&'static [&'static str]>,
    pub loc: Option<&'static [(&'static str, &'static str)]>,
}

/// Static Simple Icon
#[derive(Clone)]
pub struct SimpleIcon {
    pub slug: &'static str,
    pub title: &'static str,
    pub hex: &'static str,
    pub hex_is_relatively_light: bool,
    pub source: &'static str,
    pub guidelines: Option<&'static str>,
    pub license_url: Option<&'static str>,
    pub license_type: Option<&'static str>,
    pub aliases: Option<&'static SimpleIconAliases>,
    pub order_alpha: usize,
    pub order_color: usize,
    pub deprecation: Option<&'static IconDeprecation>,
}

impl SimpleIcon {
    pub fn plain_aliases(&self) -> Vec<&'static str> {
        let mut aliases = Vec::new();
        if let Some(aliases_) = &self.aliases {
            if let Some(aka) = aliases_.aka {
                aliases.extend_from_slice(aka);
            }
            if let Some(dup) = aliases_.dup {
                aliases.extend_from_slice(dup);
            }
            if let Some(loc) = aliases_.loc {
                aliases.extend(loc.iter().map(|(_, v)| *v));
            }
        }
        aliases
    }
}
