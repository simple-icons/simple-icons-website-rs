# Contributing guide

## Setup

- Install Rust with [rustup](https://rustup.rs/).
- Install [cargo-make] with `cargo install --force cargo-make`.
- Install [fnm](https://github.com/Schniz/fnm) with `cargo install fnm` and [setup your Shell](https://github.com/Schniz/fnm#shell-setup).
- Install NodeJS with npm and run `npm install`.
- Install Playwright browsers and dependencies with `npx playwright install --with-deps`.
- Create an _.env_ file at the root including a Github personal token with the variable `GITHUB_TOKEN` like `GITHUB_TOKEN=...`.

## Commands

- `cargo make`: Build WASM and serve. With `watch-css`, recommended for development. After it, you can use `cd app && trunk serve` to serve.
- `cargo make watch-css`: Watch the CSS files with [TailwindCSS](https://tailwindcss.com/).
- `cargo make test`: Build app for production and run tests with [Playwright](https://playwright.dev/).
- `cargo make format`: Format files.
- `cargo make lint`: Check formatting of files.
- `cargo make build`: Build the website for production.
- `cargo make serve`: Build the website for production and serve it with [serve](https://www.npmjs.com/package/serve).

## Testing

Is useful to run only certain tests in a browser. For example: `cargo make test --project=chrome-desktop --grep=header`

## Add translation

- Copy the _app/i18n/locales/en-US/_ folder into _app/locales/{id}_ changing `{id}` with the identifier of the translation. See [leptos-fluent languages documentation].
- Translate the messages in the new file.

[leptos-fluent languages documentation]: https://mondeja.github.io/leptos-fluent/languages.html

## Arquitecture

### Technologies

- [Leptos](https://docs.rs/leptos) as the components library with a client side rendering approach.
- [TailwindCSS](https://tailwindcss.com/) as the CSS framework.
- [Trunk](https://trunkrs.dev/) as the web server (on development) and application builder (on production).
- [Playwright](https://playwright.dev/) for end to end testing.

### Rust crates

- **_app/_**: Main package with the app entrypoint. It provides the logic where the top level components are composed and handles global states.
- **_components/_**: Components libraries. Provide the components used in the website.
- **_libs/_**: Libraries used in the website, like simple-icons NPM package bindings or macros for compile time code generation.

### Where to look

- End to end tests are located in _end2end/tests/_. They are written with [Playwright](https://playwright.dev/). Configuration is located at _end2end/playwright.config.ts_.
- The main stylesheet is located at _app/stylesheet.css_ other assets are located at _app/assets/_. Hopefully you don't need to change this style due to the class-based approach of TailwindCSS framework. Configuration is located at _app/tailwind.config.ts_.
- The initial HTML is located at _app/index.html_. It is used by Trunk to generate the distributed HTML. Most frontend assets are located at _app/public/_.
- The file _Makefile.toml_ contains the commands to build, test, format, lint and serve the website. It is used by [cargo-make].

[cargo-make]: https://sagiegurari.github.io/cargo-make/
