use crate::as_bool::AsBool;

impl AsBool for String {
    fn as_bool(&self) -> bool {
        self.as_str().as_bool()
    }
}
