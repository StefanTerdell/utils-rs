pub trait AsCopy<T: Copy> {
    fn as_copy(&self) -> T;
}

impl<T: AsCopy<C>, C: Copy> AsCopy<C> for &T {
    fn as_copy(&self) -> C {
        (*self).as_copy()
    }
}
