pub mod item_builder;
pub mod item_id;
pub mod item_kind;

use crate::{Count, Named};
pub use item_builder::*;
pub use item_id::*;
pub use item_kind::*;

#[derive(Serialize, Deserialize)]
pub struct ItemCount(Count);

impl From<Count> for ItemCount {
    fn from(count: Count) -> Self {
        ItemCount(count)
    }
}

impl From<ItemCount> for Count {
    fn from(count: ItemCount) -> Self {
        count.0
    }
}

#[derive(Debug)]
pub enum ItemError {
    UnspecifiedName,
    UnspecifiedKind,
}

#[derive(Serialize, Deserialize)]
pub struct Item {
    name: String,
    kind: ItemKind,
}

impl Item {
    pub fn kind(&self) -> &ItemKind {
        &self.kind
    }
}

impl Named for Item {
    fn name(&self) -> &str {
        self.name.as_ref()
    }
}
