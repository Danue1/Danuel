pub trait Named {
    fn name(&self) -> &str;
}

pub trait Builder {
    type Target;
    type Error;

    fn new() -> Self;
    fn build(self) -> Result<Self::Target, Self::Error>;
}
