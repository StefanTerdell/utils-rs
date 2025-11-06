use crate::prelude::AsClone;

impl AsClone<String> for str {
    fn as_clone(&self) -> String {
        self.to_string()
    }
}
