use core::str::FromStr;

#[derive(Default, Copy, Clone, PartialEq)]
pub enum Layout {
    #[default]
    Comfortable,
    Compact,
}

impl Layout {
    pub fn icons_per_page(&self) -> u8 {
        match self {
            Self::Comfortable => 30,
            Self::Compact => 60,
        }
    }
}

impl FromStr for Layout {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "comfortable" => Ok(Self::Comfortable),
            "compact" => Ok(Self::Compact),
            _ => Err(()),
        }
    }
}

impl core::fmt::Display for Layout {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Comfortable => write!(f, "comfortable"),
            Self::Compact => write!(f, "compact"),
        }
    }
}
