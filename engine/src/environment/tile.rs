use crate::Id;

#[derive(Eq, PartialEq, Hash, Clone, Serialize, Deserialize)]
pub struct TileId(Id);

impl From<Id> for TileId {
    fn from(id: Id) -> Self {
        TileId(id)
    }
}

impl From<TileId> for Id {
    fn from(id: TileId) -> Self {
        id.0
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Tile<Image: Default> {
    image: Image,
}
