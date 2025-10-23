pub mod as_bool;
pub mod as_clone {
    use crate::prelude::AsClone;

    impl AsClone<String> for str {
        fn as_clone(&self) -> String {
            self.to_string()
        }
    }
}
pub mod as_copy {
    use crate::prelude::AsCopy;

    impl<'a> AsCopy<&'a str> for &'a str {
        fn as_copy(&self) -> &'a str {
            self
        }
    }
}
pub mod as_str {
    use crate::as_str::AsStr;

    impl AsStr for str {
        fn as_str(&self) -> &str {
            self
        }
    }
}
