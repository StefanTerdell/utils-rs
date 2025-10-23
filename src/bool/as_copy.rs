use crate::prelude::AsCopy;

impl AsCopy<bool> for bool {
    fn as_copy(&self) -> bool {
        *self
    }
}
