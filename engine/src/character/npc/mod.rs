pub mod builder;

use crate::Named;
pub use builder::*;

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
