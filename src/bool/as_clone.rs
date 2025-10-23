use crate::prelude::AsClone;

impl AsClone<bool> for bool {
    fn as_clone(&self) -> bool {
        *self
    }
}
