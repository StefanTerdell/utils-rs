use std::fmt::Debug;

pub trait UnwrapInto<T> {
    fn unwrap_into(self) -> T;
    fn expect_into(self, message: &str) -> T;
}

impl<T, E: Debug, S: TryInto<T, Error = E>> UnwrapInto<T> for S {
    fn unwrap_into(self) -> T {
        self.try_into().unwrap()
    }

    fn expect_into(self, message: &str) -> T {
        self.try_into().expect(message)
    }
}
