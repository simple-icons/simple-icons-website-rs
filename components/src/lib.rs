#![feature(trait_alias)]
#![feature(const_option_ext)]
#![feature(stmt_expr_attributes)]

// TODO: when leptos allows imports of components without needing to
// import the Props struct, refactor imports with `use my_crate::MyComponent;`
// instead of the current `use my_crate::*;`

pub mod controls;
mod copy;
mod footer;
pub mod grid;
pub mod header;
mod ids;
mod modal;
mod scroll;
pub(crate) mod storage;
mod svg_defs;
pub(crate) mod url;

pub use controls::*;
pub use footer::*;
pub use grid::*;
pub use header::*;
pub(crate) use ids::Ids;
pub use modal::*;
pub use scroll::*;
pub use svg_defs::*;
pub(crate) use url as Url;
