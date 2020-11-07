use crate::{Mob, Named, Npc, Player};

#[derive(Eq, PartialEq, Hash, Clone, Serialize, Deserialize)]
pub struct CharacterId(u32);

impl From<u32> for CharacterId {
    fn from(id: u32) -> Self {
        CharacterId(id)
    }
}

impl From<CharacterId> for u32 {
    fn from(id: CharacterId) -> Self {
        id.0
    }
}

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
