# Contributing guide

## Setup

- Install Rust with [rustup](https://rustup.rs/).
- Add `wasm32-unknown-unknown` target with `rustup target add wasm32-unknown-unknown`.
- Install [cargo-make](https://sagiegurari.github.io/cargo-make/) with `cargo install --force cargo-make`.
- Install NodeJS with npm and run `npm install`.
- Install [nvm](https://github.com/nvm-sh/nvm).
- Install Playwright browsers and dependencies with `npx playwright install --with-deps`.
- Create an _.env_ file at the root including a Github personal token with the variable `GITHUB_TOKEN`.

## Commands

- `cargo make`: Run `serve` and `watch-css` in parallel. Recommended for development.
- `cargo make test`: Build app for production and run tests with [Playwright](https://playwright.dev/).
- `cargo make format`: Format files. If you are using VSCode they should be formatted on save.
- `cargo make lint`: Check formatting of files.
- `cargo make build`: Build the website for production.
- `cargo make serve`: Build the website for production and serve it with [anywhere](https://www.npmjs.com/package/anywhere).
- `cargo make doc`: Build and open documentation.
- `cargo make watch-css`: Watch the CSS files with [TailwindCSS](https://tailwindcss.com/).

## Testing

Is useful to run only certain tests in a browser. For example: `cargo make test --project=chrome-desktop --grep=header`

## Arquitecture

### Technologies

- [Leptos](https://docs.rs/leptos) as the components library with a client side rendering approach.
- [TailwindCSS](https://tailwindcss.com/) as the CSS framework.
- [Trunk](https://trunkrs.dev/) as the web server (on development) and application builder (on production).
- [Playwright](https://playwright.dev/) for end to end testing.

### Rust crates

- _app/_: Main package with the app entrypoint. It provides the logic where the top level components are composed.
- _components/_: Components library. It provides the components used in the website.
- _config/_: Application configuration.
- _i18n/_: It provides the translations and i18n utilities used in the website.
- _macros/_: It provides compile time macros used in the website. Used to generate the data provided by the simple-icons npm package.
- _simple-icons/_: Simple Icons Rust library. It provides the data provided by the simple-icons npm package.

### Where to look

- End to end tests are located in _end2end/tests/_. They are written with [Playwright](https://playwright.dev/). Configuration is located at _end2end/playwright.config.ts_.
- The main stylesheet is located at _app/stylesheet.css_ other assets are located at _app/assets/_. Hopefully you don't need to change this style due to the class-based approach of TailwindCSS framework. Configuration is located at _app/tailwind.config.ts_.
- The initial HTML is located at _app/index.html_. It is used by Trunk to generate the distributed HTML.

### How to add a new locale

- Add the locale to the `LANGUAGES` array in _i18n/src/lib.rs_.
- Copy the _en-US/_ locale directory at _i18n/locales_ and replace the translations.

### Compatibility

- Currently tied to Node.js >= 18 to ensure that the `fetch` API is included in the standard library.
- Using the nighly Rust toolchain to ensure some nightly Rust features like trait aliases and async closures for the _components/_ crate.
