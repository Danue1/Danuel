pub mod character_id;
pub mod class;
pub mod mob;
pub mod npc;
pub mod player;
pub mod stat;

use crate::Named;
pub use character_id::*;
pub use class::*;
pub use mob::*;
pub use npc::*;
pub use player::*;
pub use stat::*;

#[derive(Serialize, Deserialize)]
pub enum Character {
    Npc(Npc),
    Mob(Mob),
    Player(Player),
}

impl Named for Character {
    fn name(&self) -> &str {
        match self {
            Character::Npc(npc) => npc.name(),
            Character::Mob(mob) => mob.name(),
            Character::Player(player) => player.name(),
        }
    }
}
