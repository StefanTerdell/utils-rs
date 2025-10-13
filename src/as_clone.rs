pub trait AsClone<T: Clone> {
    fn as_clone(&self) -> T;
}

impl<'a, T: AsClone<C>, C: Clone> AsClone<C> for &'a T {
    fn as_clone(&self) -> C {
        (*self).as_clone()
    }
}
