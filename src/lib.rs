#![feature(strict_provenance)]

pub use error::OpenError;
pub use library::Library;

pub(crate) use modules::Modules;

mod error;
mod library;
mod modules;
