pub trait AsClone<T> {
    #[allow(clippy::wrong_self_convention)]
    fn as_clone(self) -> T;
}

impl<T: Clone> AsClone<T> for T {
    fn as_clone(self) -> T {
        self
    }
}

impl<T: Clone + AsClone<T>> AsClone<T> for &T {
    fn as_clone(self) -> T {
        (*self).clone()
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

    impl Clone for MyStruct {
        fn clone(&self) -> Self {
            Self(self.0 + 1)
        }
    }

    #[test]
    fn as_clone_should_call_clone_on_a_reference() {
        let source = MyStruct::default();
        let cloned = source.clone();

        fn takes_impl<T>(arg: impl AsClone<T>) -> T {
            arg.as_clone()
        }

        assert_eq!(cloned, takes_impl(&source));
    }

    #[test]
    fn as_clone_should_return_an_owned_value_unchanged() {
        let source = MyStruct::default();
        let equal = MyStruct::default();

        fn takes_impl<T>(arg: impl AsClone<T>) -> T {
            arg.as_clone()
        }

        assert_eq!(equal, takes_impl(source));
    }

    #[test]
    fn as_clone_should_work_with_any_clonable_type() {
        fn case(value: impl Clone + Debug + PartialEq) {
            assert_eq!(value.clone(), value.as_clone());
        }

        case(String::from("Hello, world!"));
        case("Hello, world!");
        case(123);
        case(IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)));
    }
}
