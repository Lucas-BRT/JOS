pub trait Validated: Sized {
    type Raw: Clone;
    type Error;

    fn parse(raw: Self::Raw) -> Result<Self, Self::Error>;
    fn raw(&self) -> Self::Raw;
}
