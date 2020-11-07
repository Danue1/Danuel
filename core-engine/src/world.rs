use crate::{Builder, Character, CharacterId, Fixture, FixtureId, Named, Positioned, Tile, TileId};
use std::collections::HashMap;

#[derive(Debug)]
pub enum WorldError {
    UnspecifiedWidth,
    UnspecifiedHeight,
    UnspecifiedName,
}

pub type FloorMapping = Vec<(Vec<Vec<TileId>>, Vec<Vec<FixtureId>>)>;
pub type TileMapping<Image> = HashMap<TileId, Tile<Image>>;
pub type FixtureMapping<Image> = HashMap<FixtureId, Fixture<Image>>;
pub type CharacterMapping = HashMap<CharacterId, Positioned<Character>>;

#[derive(Serialize, Deserialize)]
pub struct World<Image> {
    width: u32,
    height: u32,
    name: String,
    floor_mapping: FloorMapping,
    tile_mapping: TileMapping<Image>,
    fixture_mapping: FixtureMapping<Image>,
    character_mapping: CharacterMapping,
}

impl<Image> World<Image> {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn floor_mapping(&self) -> &FloorMapping {
        &self.floor_mapping
    }

    pub fn tile_mapping(&self) -> &TileMapping<Image> {
        &self.tile_mapping
    }

    pub fn fixture_mapping(&self) -> &FixtureMapping<Image> {
        &self.fixture_mapping
    }

    pub fn character_mapping(&self) -> &CharacterMapping {
        &self.character_mapping
    }
}

impl<Image> Named for World<Image> {
    fn name(&self) -> &str {
        self.name.as_ref()
    }
}

#[derive(Default)]
pub struct WorldBuilder<Image> {
    width: Option<u32>,
    height: Option<u32>,
    name: Option<String>,
    floor_mapping: Option<FloorMapping>,
    tile_mapping: Option<TileMapping<Image>>,
    fixture_mapping: Option<FixtureMapping<Image>>,
    character_mapping: Option<CharacterMapping>,
}

impl<Image: Default> Builder for WorldBuilder<Image> {
    type Target = World<Image>;
    type Error = WorldError;

    fn new() -> Self {
        Default::default()
    }

    fn build(self) -> Result<Self::Target, Self::Error> {
        Ok(World {
            width: self.width.ok_or(WorldError::UnspecifiedWidth)?,
            height: self.height.ok_or(WorldError::UnspecifiedHeight)?,
            name: self.name.ok_or(WorldError::UnspecifiedName)?,
            floor_mapping: self.floor_mapping.unwrap_or_default(),
            tile_mapping: self.tile_mapping.unwrap_or_default(),
            fixture_mapping: self.fixture_mapping.unwrap_or_default(),
            character_mapping: self.character_mapping.unwrap_or_default(),
        })
    }
}

impl<Image> WorldBuilder<Image> {
    pub fn width(mut self, width: u32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: u32) -> Self {
        self.height = Some(height);
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn floor_mapping(mut self, floor_mapping: FloorMapping) -> Self {
        self.floor_mapping = Some(floor_mapping);
        self
    }

    pub fn tile_mapping(mut self, tile_mapping: TileMapping<Image>) -> Self {
        self.tile_mapping = Some(tile_mapping);
        self
    }

    pub fn fixture_mapping(mut self, fixture_mapping: FixtureMapping<Image>) -> Self {
        self.fixture_mapping = Some(fixture_mapping);
        self
    }

    pub fn character_mapping(mut self, character_mapping: CharacterMapping) -> Self {
        self.character_mapping = Some(character_mapping);
        self
    }
}
