use crate::as_copy::AsCopy;

impl AsCopy<bool> for bool {
    fn as_copy(&self) -> bool {
        *self
    }
}
