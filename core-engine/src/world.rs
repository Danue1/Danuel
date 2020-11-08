use crate::{
    Build, Character, CharacterId, Class, ClassId, Context, Error, Fixture, FixtureId, Item,
    ItemId, LifeCycle, Named, Positioned, Render, Tile, TileId,
};
use std::collections::HashMap;

#[derive(Debug)]
pub enum WorldError {
    UnspecifiedWidth,
    UnspecifiedHeight,
    UnspecifiedName,
}

pub type Floor = (Vec<Vec<TileId>>, Vec<Vec<FixtureId>>);
pub type TileMapping<Image> = HashMap<TileId, Tile<Image>>;
pub type FixtureMapping<Image> = HashMap<FixtureId, Fixture<Image>>;
pub type CharacterMapping = HashMap<CharacterId, Positioned<Character>>;
pub type ClassMapping = HashMap<ClassId, Positioned<Class>>;
pub type ItemMapping = HashMap<ItemId, Positioned<Item>>;

#[derive(Serialize, Deserialize)]
pub struct World<Image: Default> {
    width: u32,
    height: u32,
    name: String,
    floor_list: Vec<Floor>,
    tile_mapping: TileMapping<Image>,
    fixture_mapping: FixtureMapping<Image>,
    character_mapping: CharacterMapping,
    class_mapping: ClassMapping,
    item_mapping: ItemMapping,
}

impl<Image: Default> World<Image> {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn floor_list(&self) -> &[Floor] {
        self.floor_list.as_slice()
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

impl<Image: Default> Named for World<Image> {
    fn name(&self) -> &str {
        self.name.as_ref()
    }
}

impl<Image: Default> Render for World<Image> {
    fn on_update(&self) -> Result<LifeCycle, Error> {
        Ok(LifeCycle::Continue)
    }

    fn on_render(&self, _context: &mut Context) -> Result<LifeCycle, Error> {
        Ok(LifeCycle::Continue)
    }
}

#[derive(Default)]
pub struct WorldBuilder<Image: Default> {
    width: Option<u32>,
    height: Option<u32>,
    name: Option<String>,
    floor_list: Option<Vec<Floor>>,
    tile_mapping: Option<TileMapping<Image>>,
    fixture_mapping: Option<FixtureMapping<Image>>,
    character_mapping: Option<CharacterMapping>,
    class_mapping: Option<ClassMapping>,
    item_mapping: Option<ItemMapping>,
}

impl<Image: Default> Build for WorldBuilder<Image> {
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
            floor_list: self.floor_list.unwrap_or_default(),
            tile_mapping: self.tile_mapping.unwrap_or_default(),
            fixture_mapping: self.fixture_mapping.unwrap_or_default(),
            character_mapping: self.character_mapping.unwrap_or_default(),
            class_mapping: self.class_mapping.unwrap_or_default(),
            item_mapping: self.item_mapping.unwrap_or_default(),
        })
    }
}

impl<Image: Default> WorldBuilder<Image> {
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

    pub fn floor_list(mut self, floor_list: Vec<Floor>) -> Self {
        self.floor_list = Some(floor_list);
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

    pub fn item_mapping(mut self, item_mapping: ItemMapping) -> Self {
        self.item_mapping = Some(item_mapping);
        self
    }
}
