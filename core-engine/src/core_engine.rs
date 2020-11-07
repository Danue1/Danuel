use crate::{Builder, Error, Version, World};

#[derive(Debug)]
pub enum CoreEngineError {
    UnspecifiedWorld,
    UnspecifiedVersion,
}

#[derive(Serialize, Deserialize)]
pub struct CoreEngine<Image> {
    version: Version,
    world: World<Image>,
}

impl<Image> CoreEngine<Image> {
    pub fn world(&self) -> &World<Image> {
        &self.world
    }

    pub fn version(&self) -> &Version {
        &self.version
    }

    pub fn run(&self) -> Result<(), Error> {
        println!("Hello, World!");

        Ok(())
    }
}

#[derive(Default)]
pub struct CoreEngineBuilder<Image> {
    version: Option<Version>,
    world: Option<World<Image>>,
}

impl<Image: Default> Builder for CoreEngineBuilder<Image> {
    type Target = CoreEngine<Image>;
    type Error = CoreEngineError;

    fn new() -> Self {
        Default::default()
    }

    fn build(self) -> Result<Self::Target, Self::Error> {
        Ok(CoreEngine {
            world: self.world.ok_or(CoreEngineError::UnspecifiedWorld)?,
            version: self.version.ok_or(CoreEngineError::UnspecifiedVersion)?,
        })
    }
}

impl<Image> CoreEngineBuilder<Image> {
    pub fn world(mut self, world: World<Image>) -> Self {
        self.world = Some(world);
        self
    }

    pub fn version(mut self, version: Version) -> Self {
        self.version = Some(version);
        self
    }
}
