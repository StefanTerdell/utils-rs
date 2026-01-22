//! Contains the implementation of `AsBool` for `Option<bool>` and `Option<&bool>`
//!
//! Example:
//! ```rust
//! use stefans_utils::prelude::*;
//!
//! assert_eq!(Some(true).as_bool(), true);
//! assert_eq!(Some(false).as_bool(), false);
//! assert_eq!(None::<bool>.as_bool(), false);
//! ```

use crate::as_bool::AsBool;

impl AsBool for Option<bool> {
    /// Returns `true` if `self` is `Some(true)`, and `false` if `self` is `None` or `Some(false)`
    fn as_bool(&self) -> bool {
        self.is_some_and(|x| x)
    }
}

impl AsBool for Option<&bool> {
    /// Returns `true` if `self` is `Some(&true)`, and `false` if `self` is `None` or `Some(&false)`
    fn as_bool(&self) -> bool {
        self.is_some_and(|x| *x)
    }
}
