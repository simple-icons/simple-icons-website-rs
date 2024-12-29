#[allow(non_snake_case)]
pub mod LocalStorage {
    pub enum Keys {
        DownloadType,
        OrderMode,
        SearchValue,
        ColorScheme,
        Layout,
        Language,
    }

    impl Keys {
        pub fn as_str(&self) -> &'static str {
            match self {
                Keys::DownloadType => "download-type",
                Keys::OrderMode => "order-mode",
                Keys::SearchValue => "search-value",
                Keys::ColorScheme => "color-scheme",
                Keys::Layout => "layout",
                Keys::Language => "language",
            }
        }
    }

    pub fn get(key: Keys) -> Option<String> {
        ::leptos::prelude::window()
            .local_storage()
            .unwrap()
            .unwrap()
            .get_item(key.as_str())
            .unwrap()
    }

    pub fn set(key: Keys, value: &str) {
        ::leptos::prelude::window()
            .local_storage()
            .unwrap()
            .unwrap()
            .set_item(key.as_str(), value)
            .unwrap()
    }
}
