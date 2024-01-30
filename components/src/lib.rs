#![feature(trait_alias)]
#![feature(stmt_expr_attributes)]
#![feature(async_closure)]

pub mod button;
pub mod controls;
pub mod copy;
pub mod event;
pub(crate) mod fetch;
pub mod footer;
pub mod grid;
pub mod header;
mod ids;
pub mod js_libs;
pub mod keyboard;
pub mod menu;
pub mod modal;
pub mod preview_generator;
pub mod storage;
pub mod svg;
pub(crate) mod url;

pub(crate) use ids::Ids;
pub(crate) use url as Url;
