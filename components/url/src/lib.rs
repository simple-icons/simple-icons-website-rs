//! URL utilities

/// Single source of thruth for the URL params state
pub mod params {
    /// Enum to ensure that the params names are unique
    pub enum Names {
        Query,
        Language,
        DownloadType,
        Layout,
        ColorScheme,
        Modal,
        OrderMode,
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
                Self::OrderMode => "order",
            }
        }
    }

    fn current_url() -> web_sys::Url {
        web_sys::Url::new(
            &web_sys::window().unwrap().location().href().unwrap(),
        )
        .unwrap()
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
        web_sys::window()
            .unwrap()
            .history()
            .unwrap()
            .replace_state_with_url(
                &wasm_bindgen::JsValue::NULL,
                "",
                Some(&url.to_string().as_string().unwrap()),
            )
            .map_err(|e| {
                let rs_msg = format!("Failed to update the URL: {e:?}");
                let js_msg = wasm_bindgen::JsValue::from_str(&rs_msg);
                web_sys::console::error_1(&js_msg);
            })
            .ok();
    }

    /// Get a URL param value from the URL of the browser
    pub fn get(k: &Names) -> Option<String> {
        current_url().search_params().get(k.as_str())
    }
}
