use crate::with_len::WithLen;

impl<T> WithLen for Vec<T> {
    fn len(&self) -> usize {
        Vec::len(self)
    }
}
