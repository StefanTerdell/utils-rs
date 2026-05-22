pub trait MapInto<T> {
    fn map_into(self) -> T;
}

impl<A: Into<B>, B> MapInto<Option<B>> for Option<A> {
    fn map_into(self) -> Option<B> {
        self.map(A::into)
    }
}

impl<OkA: Into<OkB>, ErrA: Into<ErrB>, OkB, ErrB> MapInto<Result<OkB, ErrB>> for Result<OkA, ErrA> {
    fn map_into(self) -> Result<OkB, ErrB> {
        match self {
            Ok(a) => Ok(a.into()),
            Err(err_a) => Err(err_a.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct A;
    #[derive(Debug, PartialEq)]
    struct B;
    #[derive(Debug, PartialEq)]
    struct C;
    #[derive(Debug, PartialEq)]
    struct D;

    impl From<A> for B {
        fn from(_: A) -> Self {
            B
        }
    }

    impl From<B> for A {
        fn from(_: B) -> Self {
            A
        }
    }

    impl From<C> for D {
        fn from(_: C) -> Self {
            D
        }
    }

    impl From<D> for C {
        fn from(_: D) -> Self {
            C
        }
    }

    #[test]
    fn map_options() {
        fn check<
            A: Into<B> + PartialEq + std::fmt::Debug,
            B: Into<A> + PartialEq + std::fmt::Debug,
        >(
            a: Option<A>,
            b: Option<B>,
        ) {
            assert_eq!(a.map_into(), b);
        }

        check(Option::<A>::Some(A), Option::<B>::Some(B));
        check(Option::<A>::None, Option::<B>::None);
    }

    #[test]
    fn map_results_into() {
        fn check<
            A: Into<B> + PartialEq + std::fmt::Debug,
            B: Into<A> + PartialEq + std::fmt::Debug,
            C: Into<D> + PartialEq + std::fmt::Debug,
            D: Into<C> + PartialEq + std::fmt::Debug,
        >(
            a: Result<A, C>,
            b: Result<B, D>,
        ) {
            assert_eq!(a.map_into(), b);
        }

        // AC/BD - No shared types
        check(Result::<A, C>::Ok(A), Result::<B, D>::Ok(B));
        check(Result::<B, D>::Ok(B), Result::<A, C>::Ok(A));
        check(Result::<A, C>::Err(C), Result::<B, D>::Err(D));
        check(Result::<B, D>::Err(D), Result::<A, C>::Err(C));

        // AC/AD - Shared Ok type
        check(Result::<A, C>::Ok(A), Result::<A, D>::Ok(A));
        check(Result::<A, D>::Ok(A), Result::<A, C>::Ok(A));
        check(Result::<A, C>::Err(C), Result::<A, D>::Err(D));
        check(Result::<A, D>::Err(D), Result::<A, C>::Err(C));

        // AC/BC - Shared Err type
        check(Result::<A, C>::Ok(A), Result::<B, C>::Ok(B));
        check(Result::<B, C>::Ok(B), Result::<A, C>::Ok(A));
        check(Result::<A, C>::Err(C), Result::<B, C>::Err(C));
        check(Result::<B, C>::Err(C), Result::<A, C>::Err(C));

        // AC/AC - All types shared
        check(Result::<A, A>::Ok(A), Result::<A, A>::Ok(A));
        check(Result::<A, A>::Err(A), Result::<A, A>::Err(A));
    }
}
