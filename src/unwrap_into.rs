use std::fmt::Debug;

pub trait UnwrapInto<T> {
    fn unwrap_into(self) -> T;
}

impl<T, E: Debug, S: TryInto<T, Error = E>> UnwrapInto<T> for S {
    fn unwrap_into(self) -> T {
        self.try_into().unwrap()
    }
}
