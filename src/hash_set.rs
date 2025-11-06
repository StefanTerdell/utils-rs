use std::collections::HashSet;

use crate::with_len::WithLen;

impl<T> WithLen for HashSet<T> {
    fn len(&self) -> usize {
        HashSet::len(&self)
    }
}
