fn main() {
    let target = std::env::var("TARGET").unwrap_or_default();

    // Set cfg flag for getrandom wasm_js
    if target == "wasm32-unknown-unknown" {
        // Set a custom cfg flag for wasm builds
        println!("cargo:rustc-cfg=getrandom_backend=\"wasm_js\"");
    }
}
