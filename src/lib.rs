//! This crate provides Sitemap parser and writer.
//! # Examples
//!
//! [Reading sitemap](reader/index.html#examples).
//! [Writing sitemap](writer/index.html#examples).
mod errors;
pub mod reader;
pub mod structs;
pub mod writer;
pub use errors::Error;
