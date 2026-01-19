//! A collection of useful Rust utility functions, types, and traits.

// Traits:
pub mod as_bool;
pub mod as_clone;
pub mod as_copy;
pub mod as_str;
pub mod expect_comparison;
pub mod expect_length;
#[cfg(feature = "std")]
pub mod find_upwards;
pub mod from_str;
pub mod map;
pub mod map_into;
pub mod try_map_into;
pub mod unwrap_into;
pub mod with_len;

// Trait implementations:
pub mod bool;
#[cfg(feature = "std")]
pub mod hash_map;
#[cfg(feature = "std")]
pub mod hash_set;
pub mod option;
#[cfg(feature = "std")]
pub mod path;
pub mod str;
pub mod string;
pub mod vec;

// Structs:
pub mod comparator;

// Exports:
pub mod dependencies;
pub mod prelude;
