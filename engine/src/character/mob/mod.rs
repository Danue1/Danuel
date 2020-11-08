pub mod mob_builder;

use crate::Named;
pub use mob_builder::*;

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
