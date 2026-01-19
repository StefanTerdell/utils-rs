pub trait TryMapInto<T> {
    type Error;

    fn try_map_into(self) -> Result<T, Self::Error>;
}

impl<A: TryInto<B>, B> TryMapInto<Option<B>> for Option<A> {
    type Error = A::Error;

    fn try_map_into(self) -> Result<Option<B>, Self::Error> {
        self.map(A::try_into).transpose()
    }
}

impl<OkA: TryInto<OkB>, OkB, ErrA: Into<ErrB>, ErrB> TryMapInto<Result<OkB, ErrB>>
    for Result<OkA, ErrA>
{
    type Error = OkA::Error;

    fn try_map_into(self) -> Result<Result<OkB, ErrB>, Self::Error> {
        match self {
            Ok(a) => a.try_into().map(Ok),
            Err(err_a) => Ok(Err(err_a.into())),
        }
    }
}
