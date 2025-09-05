//! Contains the AsBool trait and its implementations for `Option<bool>` and `Option<&bool>`

pub trait AsBool {
    /// Converts a reference of the current value into a bool.
    /// Mainly intended to convert an `Option<bool>` into its inner value (or to `false` if `None`).
    ///
    /// Example:
    /// ```rust
    /// use utils_rs::prefix::*;
    ///
    /// assert_eq!(Some(true).as_bool(), true);
    /// assert_eq!(Some(false).as_bool(), false);
    /// assert_eq!(None::<bool>.as_bool(), false);
    /// ```
    fn as_bool(&self) -> bool;
}

impl AsBool for Option<bool> {
    /// Returns `true` if `self` is `Some(true)`, and `false` if `self` is `None` or `Some(false)`
    fn as_bool(&self) -> bool {
        self.is_some_and(|x| x)
    }
}

impl AsBool for Option<&bool> {
    fn as_bool(&self) -> bool {
        self.is_some_and(|x| *x)
    }
}
