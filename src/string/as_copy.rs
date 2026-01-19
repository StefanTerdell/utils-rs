use crate::as_copy::AsCopy;

impl<'a> AsCopy<&'a String> for &'a String {
    fn as_copy(&self) -> &'a String {
        self
    }
}
