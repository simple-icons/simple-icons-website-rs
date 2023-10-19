use leptos::window;
use wasm_bindgen::JsCast;

/// Fetch a resource forcing the retrieval from the cache
///
/// See https://developer.mozilla.org/en-US/docs/Web/API/Request/cache
///
/// The SVG values of the icons are cached in the browser because are retrieved
/// when are loaded through the `img` tag.
pub(crate) async fn fetch_text(url: &str) -> Option<String> {
    let mut req_opts = web_sys::RequestInit::new();
    req_opts.cache(web_sys::RequestCache::ForceCache);
    req_opts.method("GET");

    let request =
        web_sys::Request::new_with_str_and_init(url, &req_opts).unwrap();

    let response = wasm_bindgen_futures::JsFuture::from(
        window().fetch_with_request(&request),
    )
    .await
    .unwrap()
    .dyn_into::<web_sys::Response>()
    .unwrap();

    match response.ok() {
        true => {
            let value: String = js_sys::JsString::from(
                wasm_bindgen_futures::JsFuture::from(response.text().unwrap())
                    .await
                    .unwrap(),
            )
            .into();
            Some(value)
        }
        false => {
            log::error!(
                "Failed to fetch {}. Check your network connection",
                url
            );
            None
        }
    }
}
