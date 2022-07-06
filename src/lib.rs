#![deny(warnings)]
#![feature(slice_from_ptr_range)]
#![feature(strict_provenance)]

pub use error::OpenError;
pub use library::Library;

pub(crate) use cache::Cache;

mod cache;
mod error;
mod library;

pub mod ffi;
