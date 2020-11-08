use crate::{Build, Npc, NpcError};

#[derive(Default)]
pub struct NpcBuilder {
    name: Option<String>,
}

impl Build for NpcBuilder {
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
