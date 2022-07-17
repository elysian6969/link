#![deny(warnings)]
#![feature(ptr_const_cast)]
#![feature(strict_provenance)]

pub use error::OpenError;
pub use library::Library;

pub(crate) use cache::Cache;

mod cache;
mod error;
mod library;

pub(crate) mod imp;

pub mod ffi;
