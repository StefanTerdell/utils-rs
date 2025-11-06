use crate::prelude::AsCopy;

impl<'a> AsCopy<&'a str> for &'a str {
    fn as_copy(&self) -> &'a str {
        self
    }
}
