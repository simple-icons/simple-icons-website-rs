use wasm_bindgen::JsCast;

/// Fetch a resource forcing the retrieval from the cache
///
/// See https://developer.mozilla.org/en-US/docs/Web/API/Request/cache
///
/// The SVG values of the icons are cached in the browser because are retrieved
/// when are loaded through the `img` tag.
pub async fn fetch_text(url: &str) -> Result<String, String> {
    let req_opts = web_sys::RequestInit::new();
    req_opts.set_cache(web_sys::RequestCache::ForceCache);
    req_opts.set_method("GET");

    let request =
        web_sys::Request::new_with_str_and_init(url, &req_opts).unwrap();

    let response = wasm_bindgen_futures::JsFuture::from(
        web_sys::window().unwrap().fetch_with_request(&request),
    )
    .await
    .unwrap()
    .unchecked_into::<web_sys::Response>();

    match response.ok() {
        true => {
            let value: String = js_sys::JsString::from(
                wasm_bindgen_futures::JsFuture::from(response.text().unwrap())
                    .await
                    .unwrap(),
            )
            .into();
            Ok(value)
        }
        false => Err(format!(
            "Failed to fetch {}: {}",
            url,
            response.status_text()
        )),
    }
}
