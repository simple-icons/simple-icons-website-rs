# Contributing guide

## Setup

- Install Rust with [rustup](https://rustup.rs/) and setup nightly as default.
- Add `wasm32-unknown-unknown` target with `rustup target add wasm32-unknown-unknown`.
- Install [cargo-make](https://sagiegurari.github.io/cargo-make/) with `cargo install --force cargo-make`.
- Install NodeJS with npm and dependencies with `npm install`.
- Install Playwright browsers and dependencies with `npx playwright install --with-deps`.
- Create an _.env_ file at the root including a Github personal token with the variable `GITHUB_TOKEN`.

## Commands

- `cargo make serve`: Serve the website with [Trunk](https://trunkrs.dev/).
- `cargo make watch-css`: Watch the CSS files with [TailwindCSS](https://tailwindcss.com/).
- `cargo make`: Run `serve` and `watch-css` in parallel. Recommended for development.
- `cargo make tests`: Run the tests with [Playwright](https://playwright.dev/) (you need to build the app before).
- `cargo make formats`: Format files. If you are using VSCode they should be formatted at save.
- `cargo make lint`: Check formatting of files. If you are using VSCode they should be formatted at save.
- `cargo make builds`: Build the website for production.
- `cargo make docs`: Build and open documentation.
- `cargo make locales`: Extract new translations. See [Localization](#localization) section.

## Testing

Before testing you must build the website for development or production. You can run `cargo make` to build for development or `cargo make builds` to build for production. The distributed folder will be located at `app/dist/`.

You'll find useful to only run certain tests for development displaying the GUI, you can change to `end2end` directory and run `npx playwright test --headed --project={browser} --grep={regex}`.

## Arquitecture

### Technologies

- [Leptos](https://docs.rs/leptos) as the components library with a client side rendering approach.
- [TailwindCSS](https://tailwindcss.com/) as the CSS framework.
- [Trunk](https://trunkrs.dev/) as the web server (on development) and application builder (on production).

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

## Localization

- Extract translations with `cargo make locales`.
- Translate po files located at _i18n/locales_ with your favorite editor.
- The translations are included in the website at compile time.

### How to add a new locale

- Add the locale to the `LANGUAGES` array in `i18n/src/lib.rs`.
- Copy the file `i18n/messages.pot` to the new locale po file at `i18n/locales/{code}.po`.

## Recommended VSCode extensions

- [Tailwind CSS IntelliSense](https://marketplace.visualstudio.com/items?itemName=bradlc.vscode-tailwindcss)
- [Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer)
- [Run On Save](https://marketplace.visualstudio.com/items?itemName=emeraldwalk.RunOnSave)
- [Prettier Code formatter](https://marketplace.visualstudio.com/items?itemName=esbenp.prettier-vscode)
