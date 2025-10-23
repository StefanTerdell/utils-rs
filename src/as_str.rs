pub trait AsStr {
    fn as_str(&self) -> &str;
}

impl<T: AsStr> AsStr for &T {
    fn as_str(&self) -> &str {
        (*self).as_str()
    }
}
