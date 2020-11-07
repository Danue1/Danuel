use crate::{Builder, Named};

#[derive(Debug)]
pub enum PlayerError {
    UnspecifiedName,
    UnspecifiedLevel,
}

#[derive(Serialize, Deserialize)]
pub struct Player {
    name: String,
    level: u8,
}

impl Player {
    pub fn level(&self) -> u8 {
        self.level
    }
}

impl Named for Player {
    fn name(&self) -> &str {
        self.name.as_ref()
    }
}

#[derive(Default)]
pub struct PlayerBuilder {
    name: Option<String>,
    level: Option<u8>,
}

impl Builder for PlayerBuilder {
    type Target = Player;
    type Error = PlayerError;

    fn new() -> Self {
        Default::default()
    }

    fn build(self) -> Result<Self::Target, Self::Error> {
        Ok(Player {
            name: self.name.ok_or(PlayerError::UnspecifiedName)?,
            level: self.level.ok_or(PlayerError::UnspecifiedLevel)?,
        })
    }
}

impl PlayerBuilder {
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn level(mut self, level: u8) -> Self {
        self.level = Some(level);
        self
    }
}
