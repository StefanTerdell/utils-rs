use std::collections::HashMap;

use crate::with_len::WithLen;

impl<K, V> WithLen for HashMap<K, V> {
    fn len(&self) -> usize {
        HashMap::len(&self)
    }
}
