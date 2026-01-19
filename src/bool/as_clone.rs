use crate::as_clone::AsClone;

impl AsClone<bool> for bool {
    fn as_clone(&self) -> bool {
        *self
    }
}
