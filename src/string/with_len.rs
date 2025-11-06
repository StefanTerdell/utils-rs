use crate::with_len::WithLen;

impl WithLen for String {
    fn len(&self) -> usize {
        String::len(&self)
    }
}
