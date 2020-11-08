use crate::{Build, Count, Id, Named};

#[derive(Eq, PartialEq, Hash, Clone, Serialize, Deserialize)]
pub struct ItemId(Id);

impl From<Id> for ItemId {
    fn from(id: Id) -> Self {
        ItemId(id)
    }
}

impl From<ItemId> for Id {
    fn from(id: ItemId) -> Self {
        id.0
    }
}

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
}

#[derive(Serialize, Deserialize)]
pub struct Item {
    name: String,
}

impl Named for Item {
    fn name(&self) -> &str {
        self.name.as_ref()
    }
}

#[derive(Default)]
pub struct ItemBuilder {
    name: Option<String>,
}

impl Build for ItemBuilder {
    type Target = Item;
    type Error = ItemError;

    fn new() -> Self {
        Default::default()
    }

    fn build(self) -> Result<Self::Target, Self::Error> {
        Ok(Item {
            name: self.name.ok_or(ItemError::UnspecifiedName)?,
        })
    }
}

impl ItemBuilder {
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
}
