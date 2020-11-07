#[derive(Eq, PartialEq, Hash, Clone, Serialize, Deserialize)]
pub struct TileId(u32);

impl From<u32> for TileId {
    fn from(id: u32) -> Self {
        TileId(id)
    }
}

impl From<TileId> for u32 {
    fn from(id: TileId) -> Self {
        id.0
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Tile<Image> {
    image: Image,
}
