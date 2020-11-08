use crate::{Build, Named};

#[derive(Debug)]
pub enum MobError {
    UnspecifiedName,
    UnspecifiedLevel,
    UnspecifiedDescription,
}

#[derive(Serialize, Deserialize)]
pub struct Mob {
    name: String,
    level: u8,
    description: String,
}

impl Mob {
    pub fn level(&self) -> u8 {
        self.level
    }

    pub fn description(&self) -> &str {
        self.description.as_ref()
    }
}

impl Named for Mob {
    fn name(&self) -> &str {
        self.name.as_ref()
    }
}

#[derive(Default)]
pub struct MobBuilder {
    name: Option<String>,
    level: Option<u8>,
    description: Option<String>,
}

impl Build for MobBuilder {
    type Target = Mob;
    type Error = MobError;

    fn new() -> Self {
        Default::default()
    }

    fn build(self) -> Result<Self::Target, Self::Error> {
        Ok(Mob {
            name: self.name.ok_or(MobError::UnspecifiedName)?,
            level: self.level.ok_or(MobError::UnspecifiedLevel)?,
            description: self.description.ok_or(MobError::UnspecifiedDescription)?,
        })
    }
}

impl MobBuilder {
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn level(mut self, level: u8) -> Self {
        self.level = Some(level);
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }
}
