use crate::{Build, Context, Error, LifeCycle, Render, Version, World};

#[derive(Debug)]
pub enum CoreEngineError {
    UnspecifiedWorld,
    UnspecifiedVersion,
}

#[derive(Serialize, Deserialize)]
pub struct CoreEngine<Image: Default> {
    version: Version,
    world: World<Image>,
    context: Context,
}

impl<Image: Default> CoreEngine<Image> {
    pub fn world(&self) -> &World<Image> {
        &self.world
    }

    pub fn version(&self) -> &Version {
        &self.version
    }

    pub fn run(&mut self) -> Result<LifeCycle, Error> {
        while let LifeCycle::Continue = self.world.on_update()? {
            if let LifeCycle::Exit = self.world.on_render(&mut self.context)? {
                break;
            }
        }

        Ok(LifeCycle::Exit)
    }
}

#[derive(Default)]
pub struct CoreEngineBuilder<Image: Default> {
    world: Option<World<Image>>,
    version: Option<Version>,
    context: Option<Context>,
}

impl<Image: Default> Build for CoreEngineBuilder<Image> {
    type Target = CoreEngine<Image>;
    type Error = CoreEngineError;

    fn new() -> Self {
        Default::default()
    }

    fn build(self) -> Result<Self::Target, Self::Error> {
        Ok(CoreEngine {
            world: self.world.ok_or(CoreEngineError::UnspecifiedWorld)?,
            version: self.version.ok_or(CoreEngineError::UnspecifiedVersion)?,
            context: self.context.unwrap_or_default(),
        })
    }
}

impl<Image: Default> CoreEngineBuilder<Image> {
    pub fn world(mut self, world: World<Image>) -> Self {
        self.world = Some(world);
        self
    }

    pub fn version(mut self, version: Version) -> Self {
        self.version = Some(version);
        self
    }
}
