/// Contains extension traits and generic implementations. Specific implementations by type are done in the respective type module in the root of this crate (such as $crate::option)
pub trait AsBoolExt {
    /// Converts a reference of the current value into a bool
    ///
    /// Example:
    /// ```rust
    /// use utils_rs::traits::AsBoolExt;
    ///
    /// assert_eq!(Some(true).as_bool(), true);
    /// assert_eq!(Some(false).as_bool(), false);
    /// assert_eq!(None.as_bool(), false);
    /// ```
    fn as_bool(&self) -> bool;
}
