# Contributing guide

## Setup

- Install Rust with [rustup](https://rustup.rs/).
- Add `wasm32-unknown-unknown` target with `rustup target add wasm32-unknown-unknown`.
- Install [cargo-make](https://sagiegurari.github.io/cargo-make/) with `cargo install --force cargo-make`.
- Install NodeJS with npm and dependencies with `npm install`.
- Install Playwright browsers and dependencies with `npx playwright install --with-deps`.
- Create an _.env_ file at the root including a Github personal token with the variable `GITHUB_TOKEN`.

## Commands

- `cargo make`: Run `serve` and `watch-css` in parallel. Recommended for development.
- `cargo make tests`: Run the tests with [Playwright](https://playwright.dev/) (you need to build the app for production before).
- `cargo make formats`: Format files. If you are using VSCode they should be formatted at save.
- `cargo make lint`: Check formatting of files. If you are using VSCode they should be formatted at save.
- `cargo make builds`: Build the website for production.
- `cargo make serve`: Build the website for production and serve it with [anywhere](https://www.npmjs.com/package/anywhere).
- `cargo make docs`: Build and open documentation.
- `cargo make watch-css`: Watch the CSS files with [TailwindCSS](https://tailwindcss.com/).

## Testing

Is useful to run only certain tests in a browser. For example: `cargo make tests --project=chrome-desktop --grep=header`

## Arquitecture

### Technologies

- [Leptos](https://docs.rs/leptos) as the components library with a client side rendering approach.
- [TailwindCSS](https://tailwindcss.com/) as the CSS framework.
- [Trunk](https://trunkrs.dev/) as the web server (on development) and application builder (on production).
- [Playwright](https://playwright.dev/) for end to end testing.

### Rust crates

- `app/`: Main package with the app entrypoint. It provides the logic where the top level components are composed.
- `components/`: Components library. It provides the components used in the website.
- `config/`: Application configuration.
- `i18n/`: It provides the translations and i18n utilities used in the website.
- `macros/`: It provides compile time macros used in the website. Used to generate the data provided by the simple-icons npm package.
- `simple-icons/`: Simple Icons Rust library. It provides the data provided by the simple-icons npm package.
- `scripts/`: Scripts used to generate data needed by the website.

### Where to look

- End to end tests are located in `end2end/tests/`. They are written with [Playwright](https://playwright.dev/). Configuration is located at `end2end/playwright.config.ts`.
- The main stylesheet is located at `app/stylesheet.css` other assets are located at `app/assets/`. Hopefully you don't need to change this style due to the class-based approach of TailwindCSS framework. Configuration is located at `app/tailwind.config.ts`.
- The initial HTML is located at `app/index.html`. It is used by Trunk to generate the final HTML.

### How to add a new locale

- Add the locale to the `LANGUAGES` array in `i18n/src/lib.rs`.
- Copy the `en-US` locale directory at `i18n/locales` and replace the translations.

### Compatibility

- We are currently tied to Node.js >= 18 to ensure that the `fetch` API is included in the standard library.
