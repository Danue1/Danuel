use crate::{Builder, Named};

#[derive(Debug)]
pub enum NpcError {
    UnspecifiedName,
}

#[derive(Serialize, Deserialize)]
pub struct Npc {
    name: String,
}

impl Named for Npc {
    fn name(&self) -> &str {
        self.name.as_ref()
    }
}

#[derive(Default)]
pub struct NpcBuilder {
    name: Option<String>,
}

impl Builder for NpcBuilder {
    type Target = Npc;
    type Error = NpcError;

    fn new() -> Self {
        Default::default()
    }

    fn build(self) -> Result<Self::Target, Self::Error> {
        Ok(Npc {
            name: self.name.ok_or(NpcError::UnspecifiedName)?,
        })
    }
}

impl NpcBuilder {
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
}
