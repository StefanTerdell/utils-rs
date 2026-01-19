use crate::as_clone::AsClone;

impl AsClone<String> for String {
    fn as_clone(&self) -> String {
        self.clone()
    }
}
