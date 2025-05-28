# leptos-unique-ids

[Leptos] library to ensure unique identifiers in components. Uses an attribute macro + Dylint to generate unique identifiers for components, preventing conflicts and ensuring consistency across your application.

## Installation

```toml
[dependencies]
leptos-unique-ids = "0.1"
```

## Usage

Create a module in your Leptos application to manage unique identifiers. It must expose an enum with the name `Ids` and the `#[leptos_unique_ids]` attribute macro applied to it.

```rust
// ids.rs
use leptos_unique_ids::leptos_unique_ids;

#[leptos_unique_ids(
    "language-selector",
    "preview-download-svg-button",
    "preview-upload-svg-button",
)]
pub enum Ids {};
```

Then, in your components, you can use the `Ids` enum to generate unique identifiers:

```rust
use your_crate::Ids;

Ids::LanguageSelector.as_str();
```

[Leptos]: https://leptos.dev
