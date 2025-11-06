//! A collection of useful Rust utility functions, types, and traits.

// Traits:
pub mod as_bool;
pub mod as_clone;
pub mod as_copy;
pub mod as_str;
pub mod expect_comparison;
pub mod expect_length;
pub mod from_str;
pub mod map;
pub mod unwrap_into;
pub mod with_len;

// Trait implementations:
pub mod bool;
pub mod hash_map;
pub mod hash_set;
pub mod option;
pub mod str;
pub mod string;
pub mod vec;

// Structs:
pub mod comparator;
pub mod path;
pub mod secret;
#[cfg(feature = "serde_formats")]
pub mod serde_formats;

// Exports:
pub mod dependencies;
pub mod prelude;
