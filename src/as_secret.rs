use crate::secret::Secret;

pub trait AsSecret<T> {
    #[allow(clippy::wrong_self_convention)]
    fn as_secret(self) -> Secret<T>;
}

impl<T> AsSecret<T> for Secret<T> {
    fn as_secret(self) -> Secret<T> {
        self
    }
}

impl<T> AsSecret<T> for T {
    fn as_secret(self) -> Secret<T> {
        Secret::new(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::as_secret::AsSecret;
    use crate::secret::Secret;

    #[test]
    fn hmm() {
        fn assert_eq_as_secret(value: impl AsSecret<&'static str>, expected: &str) {
            assert_eq!(
                value.as_secret().expose_ref(),
                Secret::new(expected).expose_ref()
            );
        }

        assert_eq_as_secret("hello", "hello");
        assert_eq_as_secret(Secret::new("hello"), "hello");
    }
}
