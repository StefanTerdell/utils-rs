pub trait AsCopy<T: Copy> {
    fn as_copy(&self) -> T;
}

impl<'a, T: AsCopy<C>, C: Copy> AsCopy<C> for &'a T {
    fn as_copy(&self) -> C {
        (*self).as_copy()
    }
}
