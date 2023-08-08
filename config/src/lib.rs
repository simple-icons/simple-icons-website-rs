pub const DEPRECATED_ICONS_FILE_NAME: &str = "simple-icons-deprecated.txt";

/// Application configuration
pub struct Config {
    /// Number of icons to load
    pub max_icons: Option<usize>,
    /// Number of icons per page in the grid
    /// when comfortable layout is selected
    pub icons_per_page_comfortable: u32,
    /// Number of icons per page in the grid
    /// when compact layout is selected
    pub icons_per_page_compact: u32,
    /// Public URL of the website
    pub domain: &'static str,
}

/// Development config
#[cfg(debug_assertions)]
pub const CONFIG: Config = Config {
    max_icons: None,
    // WARNING: If you put a great number here, the search functionality
    // will be very slow
    icons_per_page_comfortable: 30,
    icons_per_page_compact: 60,
    domain: "127.0.0.1:8080",
};

/// Production config
#[cfg(not(debug_assertions))]
pub const CONFIG: Config = Config {
    max_icons: None,
    icons_per_page_comfortable: 30,
    icons_per_page_compact: 60,
    domain: "wasm.simpleicons.org",
};
