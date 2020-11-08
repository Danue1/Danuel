pub mod builder;

use crate::{ClassId, ItemCount, ItemId, Named, Stat};
pub use builder::*;

#[derive(Debug)]
pub enum PlayerError {
    UnspecifiedName,
    UnspecifiedClass,
    UnspecifiedLevel,
    UnspecifiedHealth,
    UnspecifiedMana,
    UnspecifiedStamina,
    UnspecifiedStat,
    UnspecifiedItemList,
}

#[derive(Serialize, Deserialize)]
pub struct Player {
    name: String,
    class: ClassId,
    level: u8,
    health: u32,
    mana: u32,
    stamina: u32,
    stat: Stat,
    item_list: Vec<(ItemId, ItemCount)>,
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
