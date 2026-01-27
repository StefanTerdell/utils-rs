use std::sync::Arc;

pub trait AsArc<T> {
    #[allow(clippy::wrong_self_convention)]
    fn as_arc(self) -> Arc<T>;
}

impl<T> AsArc<T> for Arc<T> {
    fn as_arc(self) -> Arc<T> {
        self
    }
}

impl<T> AsArc<T> for &Arc<T> {
    fn as_arc(self) -> Arc<T> {
        self.clone()
    }
}

impl<T> AsArc<T> for T {
    fn as_arc(self) -> Arc<T> {
        Arc::new(self)
    }
}

impl<T: Clone> AsArc<T> for &T {
    fn as_arc(self) -> Arc<T> {
        Arc::new(self.clone())
    }
}

impl<T> AsArc<T> for Box<T> {
    fn as_arc(self) -> Arc<T> {
        Arc::new(*self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Default)]
    struct MyStruct(usize);

    impl Clone for MyStruct {
        fn clone(&self) -> Self {
            Self(self.0 + 1)
        }
    }

    #[test]
    fn as_arc_should_only_clone_a_ref_of_arc() {
        let before = Arc::new(MyStruct::default());

        fn takes_impl(arg: impl AsArc<MyStruct>) -> Arc<MyStruct> {
            arg.as_arc()
        }

        assert_eq!(
            Arc::strong_count(&before),
            1,
            "Arc ref count should be 1 before passing value to takes_impl"
        );
        assert_eq!(
            before.0, 0,
            "Clone count should be 0 before passing value to takes_impl"
        );

        let referenced = takes_impl(&before);

        assert_eq!(
            Arc::strong_count(&referenced),
            2,
            "Arc ref count should be 2 after passing reference to takes_impl"
        );
        assert_eq!(
            Arc::strong_count(&before),
            2,
            "Arc ref count should be 2 after passing reference to takes_impl"
        );
        assert_eq!(
            referenced.0, 0,
            "Clone count should be 0 after passing reference to takes_impl"
        );
        assert_eq!(
            before.0, 0,
            "Clone count should be 0 after passing reference to takes_impl"
        );

        let moved = takes_impl(before);

        assert_eq!(
            Arc::strong_count(&referenced),
            2,
            "Arc ref count should be 2 after moving value to takes_impl"
        );
        assert_eq!(
            Arc::strong_count(&moved),
            2,
            "Arc ref count should be 2 after moving value to takes_impl"
        );
        assert_eq!(
            referenced.0, 0,
            "Clone count should be 0 after moving value to takes_impl"
        );
        assert_eq!(
            moved.0, 0,
            "Clone count should be 0 after moving value to takes_impl"
        );
    }

    #[test]
    fn as_arc_should_only_clone_a_ref_of_inner() {
        let before = MyStruct::default();

        fn takes_impl(arg: impl AsArc<MyStruct>) -> Arc<MyStruct> {
            arg.as_arc()
        }

        assert_eq!(
            before.0, 0,
            "Clone count should be 0 before passing value to takes_impl"
        );

        let arc_from_ref = takes_impl(&before);

        assert_eq!(
            arc_from_ref.0, 1,
            "Inner clone count of arc_from_ref should be 1 after passing reference to takes_impl"
        );
        assert_eq!(
            before.0, 0,
            "Clone count of before should still be 0 after passing reference to takes_impl"
        );

        let arc_from_moved = takes_impl(before);

        assert_eq!(
            arc_from_ref.0, 1,
            "Inner clone count of arc_from_ref should still be 1 after moving original to takes_impl"
        );
        assert_eq!(
            arc_from_moved.0, 0,
            "Inner clone count of arc_from_moved should still be 0 after moving original to takes_impl"
        );
    }
}
