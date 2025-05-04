pub trait TypeWrapped: Sized {
    type Raw: Clone + ToString;
    type Error;

    fn parse(raw: Self::Raw) -> Result<Self, Self::Error>;
    fn raw(&self) -> Self::Raw;
}
