use crate::{Build, Item, ItemError, ItemKind};

#[derive(Default)]
pub struct ItemBuilder {
    name: Option<String>,
    kind: Option<ItemKind>,
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
            kind: self.kind.ok_or(ItemError::UnspecifiedKind)?,
        })
    }
}

impl ItemBuilder {
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn kind(mut self, kind: ItemKind) -> Self {
        self.kind = Some(kind);
        self
    }
}
