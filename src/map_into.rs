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
