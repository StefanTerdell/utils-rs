pub trait AsClone<T: Clone> {
    fn as_clone(&self) -> T;
}

impl<T: AsClone<C>, C: Clone> AsClone<C> for &T {
    fn as_clone(&self) -> C {
        (*self).as_clone()
    }
}
