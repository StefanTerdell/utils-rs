/// Contains trait implementations and utility functions for `core::option::Option<T>`
use crate::prefix::*;

impl AsBoolExt for Option<bool> {
    /// Returns `true` if `self` is `Some(true)`, and `false` if `self` is `None` or `Some(false)`
    fn as_bool(&self) -> bool {
        self.is_some_and(|x| x)
    }
}
