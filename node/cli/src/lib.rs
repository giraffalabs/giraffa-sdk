//! Substrate Node CLI library.

#![warn(missing_docs)]
#![warn(unused_extern_crates)]

pub mod chain_spec;

#[macro_use]
mod service;

#[cfg(feature = "cli")]
mod cli;

#[cfg(feature = "cli")]
pub use cli::*;