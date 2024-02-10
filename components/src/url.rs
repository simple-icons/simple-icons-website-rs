//! URL utilities working with Leptos
//!
//! Currently, there is not a way to reactively maintain the state
//! of the URL of the page, so we need to hand craft some convenient
//! utilities

/// Single source of thruth for the URL params state
pub mod params {
    use leptos::window;

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

        let query = params.to_string().as_string().unwrap();
        let query = query.trim_matches('?');
        let pathname = url.pathname();
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
        let url = current_url();
        let params = url.search_params();
        let iterator = js_sys::try_iter(&params).unwrap().unwrap();

        for pair in iterator {
            let pair = pair.unwrap();
            let key = js_sys::Reflect::get(&pair, &0.into())
                .unwrap()
                .as_string()
                .unwrap();
            if key.as_str() != k.as_str() {
                continue;
            }
            let value = js_sys::Reflect::get(&pair, &1.into())
                .unwrap()
                .as_string()
                .unwrap();
            if value.is_empty() {
                return None;
            } else {
                return Some(value);
            }
        }
        None
    }
}
