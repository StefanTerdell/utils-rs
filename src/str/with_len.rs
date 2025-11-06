use crate::with_len::WithLen;

impl WithLen for str {
    fn len(&self) -> usize {
        str::len(&self)
    }
}
