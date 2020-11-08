pub mod item_builder;
pub mod item_id;
pub mod item_kind;

use crate::{ClassId, Count, Named};
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
    UnspecifiedClassAllowList,
}

#[derive(Serialize, Deserialize)]
pub struct Item {
    name: String,
    kind: ItemKind,
    class_allow_list: Vec<ClassId>,
}

impl Item {
    pub fn kind(&self) -> &ItemKind {
        &self.kind
    }

    pub fn class_allow_list(&self) -> &[ClassId] {
        self.class_allow_list.as_ref()
    }
}

impl Named for Item {
    fn name(&self) -> &str {
        self.name.as_ref()
    }
}
