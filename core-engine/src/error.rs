use crate::{CoreEngineError, ItemError, MobError, NpcError, PlayerError, StatError, WorldError};

#[derive(Debug)]
pub enum Error {
    CoreEngine(CoreEngineError),
    Item(ItemError),
    Mob(MobError),
    Npc(NpcError),
    Player(PlayerError),
    Stat(StatError),
    World(WorldError),
}

impl From<CoreEngineError> for Error {
    fn from(error: CoreEngineError) -> Self {
        Error::CoreEngine(error)
    }
}

impl From<ItemError> for Error {
    fn from(error: ItemError) -> Self {
        Error::Item(error)
    }
}

impl From<MobError> for Error {
    fn from(error: MobError) -> Self {
        Error::Mob(error)
    }
}

impl From<NpcError> for Error {
    fn from(error: NpcError) -> Self {
        Error::Npc(error)
    }
}

impl From<PlayerError> for Error {
    fn from(error: PlayerError) -> Self {
        Error::Player(error)
    }
}

impl From<StatError> for Error {
    fn from(error: StatError) -> Self {
        Error::Stat(error)
    }
}

impl From<WorldError> for Error {
    fn from(error: WorldError) -> Self {
        Error::World(error)
    }
}
