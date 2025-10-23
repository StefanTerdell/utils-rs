//! Contains the AsBool trait used for converting a reference of self into a bool.

pub trait AsBool {
    fn as_bool(&self) -> bool;
}

impl<T: AsBool> AsBool for &T {
    fn as_bool(&self) -> bool {
        (*self).as_bool()
    }
}
