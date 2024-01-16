//! URL utilities working with Leptos
//!
//! Currently, there is not a way to reactively maintain the state
//! of the URL of the page, so we need to hand craft some convenient
//! utilities

/// Single source of thruth for the URL params state
pub mod params {
    use leptos::window;
    use leptos_router::Url;
    use wasm_bindgen;

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

    /// Update a parameter value in the URL query using window history
    pub fn update(k: &Names, v: &str) {
        let current_url =
            Url::try_from(window().location().search().unwrap().as_str())
                .unwrap();
        let mut params = current_url.search_params;
        // Remove empty values from the URL!
        if v.is_empty() {
            params.remove(k.as_str());
        } else {
            params.insert(k.as_str().to_string(), v.to_string());
        }

        let q = params.to_query_string();
        let query = q.trim_matches('?');
        let pathname = window().location().pathname().unwrap();
        let url = match query.is_empty() {
            false => format!("{}?{}", pathname, query),
            true => pathname,
        };
        window()
            .history()
            .unwrap()
            .replace_state_with_url(
                &wasm_bindgen::JsValue::NULL,
                "",
                Some(&url),
            )
            .ok();
    }

    /// Get a URL param value from the URL of the browser
    pub fn get(k: &Names) -> Option<String> {
        let query = window().location().search().unwrap();
        if !query.starts_with('?') {
            return None;
        }
        for (key, value) in
            Url::try_from(query.as_str()).unwrap().search_params.0
        {
            if key != k.as_str() {
                continue;
            }
            if value.is_empty() {
                return None;
            } else {
                return Some(value);
            }
        }
        None
    }
}
