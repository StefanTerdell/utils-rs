use crate::as_str::AsStr;

impl AsStr for str {
    fn as_str(&self) -> &str {
        self
    }
}
