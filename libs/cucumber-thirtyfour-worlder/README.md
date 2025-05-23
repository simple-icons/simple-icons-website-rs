# cucumber-thirtyfour-worlder

Do you need to reuse a bunch of logic between different projects testing apps with [Cucumber] and [Thirtyfour]? This crate is for you.

Provides a `cucumber::World` builder that can be used to create a `World` for thirtyfour tests, allowing to inject environment variables to parametrize them.

- `BROWSER`: the browser to use. Supported are `firefox`, `chrome`, and `edge`.
- `HEADLESS`: by default, tests are executed in headless mode. Set this to `false` to run them in a visible browser.
- `WINDOW_SIZE`: the size of the browser window. The default is `1920x1080`.
- `HOST_URL`: the base URL of the application under test. The default is `http://localhost:8080`.
- `DRIVER_URL`: the URL of the WebDriver server. The default is `http://localhost:4444`.

## Usage

Create a crate and add the following dependencies to your `Cargo.toml`.

```toml
[dependencies]
cucumber = "0.15"
thirtyfour = "0.34"
cucumber-thirtyfour-worlder = "0.1"
```

Inside, create your `AppWorld` struct and pass it the `#[worlder]` attribute.

```rust
use cucumber_thirtyfour_worlder::worlder;

#[worlder]
pub struct AppWorld;
```

Then, create a crate for the tests and run the world as you would do with `cucumber::World` directly.

```rust
use your_crate::AppWorld;
use cucumber::World;

#[tokio::main]
async fn main() {
    AppWorld::cucumber()
        .fail_on_skipped()
        .run_and_exit("./features/desktop")
        .await
}
```

## Known issues

### `cargo-machete` additional configuration

The [`cargo-machete`] tool don't know that you're not using neither `cucumber` nor `thirtyfour` directly, so it could complain about missing dependencies. To fix this, add the following to your _Cargo.toml_.

```toml
[package.metadata.cargo-machete]
ignored = ["thirtyfour", "cucumber"]
```

[`cargo-machete`]: https://github.com/bnjbvr/cargo-machete
