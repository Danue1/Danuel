use crate::{Id, Mob, Named, Npc, Player};

#[derive(Eq, PartialEq, Hash, Clone, Serialize, Deserialize)]
pub struct CharacterId(Id);

impl From<Id> for CharacterId {
    fn from(id: Id) -> Self {
        CharacterId(id)
    }
}

impl From<CharacterId> for Id {
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
