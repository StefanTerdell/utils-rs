pub trait AsCopy<T> {
    #[allow(clippy::wrong_self_convention)]
    fn as_copy(self) -> T;
}

impl<T: Copy> AsCopy<T> for T {
    fn as_copy(self) -> T {
        self
    }
}

impl<T: Copy + AsCopy<T>> AsCopy<T> for &T {
    fn as_copy(self) -> T {
        (*self).as_copy()
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fmt::Debug,
        net::{IpAddr, Ipv4Addr},
    };

    use super::*;

    #[derive(Debug, PartialEq, Default)]
    struct MyStruct(usize);

    #[allow(clippy::non_canonical_clone_impl)]
    impl Clone for MyStruct {
        fn clone(&self) -> Self {
            Self(self.0 + 1)
        }
    }

    impl Copy for MyStruct {}

    #[test]
    fn as_copy_should_call_copy_on_a_reference() {
        let source = MyStruct::default();
        let copied = source;

        fn takes_impl<T>(arg: impl AsCopy<T>) -> T {
            arg.as_copy()
        }

        assert_eq!(
            copied,
            takes_impl(
                #[allow(clippy::needless_borrows_for_generic_args)]
                &source
            )
        );
    }

    #[test]
    fn as_copy_should_return_an_owned_value_unchanged() {
        let source = MyStruct::default();
        let equal = MyStruct::default();

        fn takes_impl<T>(arg: impl AsCopy<T>) -> T {
            arg.as_copy()
        }

        assert_eq!(equal, takes_impl(source));
    }

    #[test]
    fn as_copy_should_work_with_any_copyable_type() {
        fn case(value: impl Copy + Debug + PartialEq) {
            assert_eq!(value, value.as_copy());
        }

        case("Hello, world!");
        case(123);
        case(IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)));
    }
}
