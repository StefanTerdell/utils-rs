use crate::as_str::AsStr;

impl AsStr for String {
    fn as_str(&self) -> &str {
        self
    }
}
