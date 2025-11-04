//! A collection of useful Rust utility functions, types, and traits.

pub mod as_bool;
pub mod as_clone;
pub mod as_copy;
pub mod as_str;
pub mod from_str;
pub mod map;

pub mod bool;
pub mod option;
pub mod str;

pub mod path;
pub mod prelude;
pub mod secret;

#[cfg(feature = "serde_formats")]
pub mod serde_formats;

pub mod dependencies;
