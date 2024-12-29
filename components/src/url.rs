//! URL utilities working with Leptos
//!
//! Currently, there is not a way to reactively maintain the state
//! of the URL of the page, so we need to hand craft some convenient
//! utilities

/// Single source of thruth for the URL params state
pub mod params {
    use leptos::prelude::window;

    /// Enum to ensure that the params names are unique
    pub enum Names {
        Query,
        Language,
        DownloadType,
        Layout,
        ColorScheme,
        Modal,
    }

    impl Names {
        pub fn as_str(&self) -> &'static str {
            match self {
                Self::Query => "q",
                Self::Language => "lang",
                Self::DownloadType => "download-type",
                Self::Layout => "layout",
                Self::ColorScheme => "color-scheme",
                Self::Modal => "modal",
            }
        }
    }

    fn current_url() -> web_sys::Url {
        web_sys::Url::new(&window().location().href().unwrap()).unwrap()
    }

    /// Update a parameter value in the URL query using window history
    pub fn update(k: &Names, v: &str) {
        let url = current_url();
        let params = url.search_params();
        // Remove empty values from the URL
        if v.is_empty() {
            params.delete(k.as_str())
        } else {
            params.set(k.as_str(), v)
        }
        url.set_search(&params.to_string().as_string().unwrap());
        window()
            .history()
            .unwrap()
            .replace_state_with_url(
                &wasm_bindgen::JsValue::NULL,
                "",
                Some(&url.to_string().as_string().unwrap()),
            )
            .map_err(|e| {
                leptos::logging::error!("Failed to update the URL: {:?}", e)
            })
            .ok();
    }

    /// Get a URL param value from the URL of the browser
    pub fn get(k: &Names) -> Option<String> {
        current_url().search_params().get(k.as_str())
    }
}
