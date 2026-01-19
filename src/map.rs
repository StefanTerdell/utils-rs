//! Contains the Map trait which collects the `ok`, `ok_or`, `map`, `unwrap` and `expect` functions as seen on TV

use core::fmt;

pub trait Map: Sized {
    fn ok(self) -> Result<Self, Self>;
    fn ok_or<E>(self, err: E) -> Result<Self, E>;
    fn map<T>(self, f: impl FnOnce() -> T) -> Option<T>;
    fn and_then<T>(self, f: impl FnOnce() -> Option<T>) -> Option<T>;
    fn unwrap(self) -> Self;
    fn expect(self, message: impl fmt::Display) -> Self;
}
