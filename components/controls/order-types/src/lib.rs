use simple_icons_website_types::SimpleIcon;
use std::str::FromStr;

#[derive(Default, Copy, Clone, PartialEq)]
pub enum OrderModeVariant {
    Alphabetic,
    AlphabeticReverse,
    Color,
    ColorReverse,
    SearchMatch,
    #[default]
    Random,
}

impl From<&str> for OrderModeVariant {
    fn from(order_mode: &str) -> Self {
        match order_mode {
            "alpha" => Self::Alphabetic,
            "alpha-reverse" => Self::AlphabeticReverse,
            "color" => Self::Color,
            "color-reverse" => Self::ColorReverse,
            "random" => Self::Random,
            _ => Self::SearchMatch,
        }
    }
}

impl FromStr for OrderModeVariant {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl core::fmt::Display for OrderModeVariant {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Alphabetic => write!(f, "alpha"),
            Self::AlphabeticReverse => write!(f, "alpha-reverse"),
            Self::Color => write!(f, "color"),
            Self::ColorReverse => write!(f, "color-reverse"),
            Self::SearchMatch => write!(f, "search"),
            Self::Random => write!(f, "random"),
        }
    }
}

impl OrderModeVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Alphabetic => "alpha",
            Self::AlphabeticReverse => "alpha-reverse",
            Self::Color => "color",
            Self::ColorReverse => "color-reverse",
            Self::SearchMatch => "search",
            Self::Random => "random",
        }
    }
}

#[derive(Copy, Clone, Default)]
pub struct OrderMode {
    /// The order mode preferred by the user
    pub favorite: OrderModeVariant,
    /// The order mode currently in use
    pub current: OrderModeVariant,
}

impl FromStr for OrderMode {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            favorite: OrderModeVariant::from(value),
            current: OrderModeVariant::from(value),
        })
    }
}

pub fn sort_icons(order_mode: &OrderModeVariant, icons: &mut Vec<&SimpleIcon>) {
    match order_mode {
        OrderModeVariant::Alphabetic => {
            icons.sort_by(|a, b| a.order_alpha.cmp(&b.order_alpha));
        }
        OrderModeVariant::AlphabeticReverse => {
            icons.sort_by(|a, b| b.order_alpha.cmp(&a.order_alpha));
        }
        OrderModeVariant::Color => {
            icons.sort_by(|a, b| a.order_color.cmp(&b.order_color));
        }
        OrderModeVariant::ColorReverse => {
            icons.sort_by(|a, b| b.order_color.cmp(&a.order_color));
        }
        OrderModeVariant::Random => {
            // Durstenfeld shuffle
            for i in 0..icons.len() {
                let j = js_sys::Math::floor(
                    js_sys::Math::random() * (i + 1) as f64,
                ) as usize;
                icons.swap(i, j);
            }
        }
        _ => {
            // Search match order is handled by the search control
        }
    }
}
