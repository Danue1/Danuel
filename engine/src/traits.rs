use crate::{Context, Error, LifeCycle};

pub trait Named {
    fn name(&self) -> &str;
}

pub trait Build {
    type Target;
    type Error;

    fn new() -> Self;
    fn build(self) -> Result<Self::Target, Self::Error>;
}

pub trait Render {
    fn on_update(&self) -> Result<LifeCycle, Error>;
    fn on_render(&self, context: &mut Context) -> Result<LifeCycle, Error>;
}
