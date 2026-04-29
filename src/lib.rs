//! A collection of useful Rust utility functions, types, and traits.

// Traits:
pub mod as_arc;
pub mod as_bool;
pub mod as_clone;
pub mod as_copy;
pub mod as_str;
pub mod expect_comparison;
pub mod expect_length;
pub mod find_upwards;
pub mod from_str;
pub mod map;
pub mod map_into;
pub mod try_map_into;
pub mod unwrap_into;
pub mod with_len;

// Trait implementations:
pub mod bool;
pub mod hash_map;
pub mod hash_set;
pub mod option;
pub mod path;
pub mod str;
pub mod string;
pub mod vec;

// Structs:
pub mod comparator;
pub mod secret;

// Serde-helpers
#[cfg(feature = "serde")]
pub mod php_safe_hashmap;
#[cfg(feature = "serde")]
pub mod single_kv_map;

// Exports:
pub mod dependencies;
pub mod prelude;
