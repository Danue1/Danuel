use crate::{Build, ClassId, Item, ItemError, ItemKind};

#[derive(Default)]
pub struct ItemBuilder {
    name: Option<String>,
    kind: Option<ItemKind>,
    class_allow_list: Option<Vec<ClassId>>,
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
            class_allow_list: self
                .class_allow_list
                .ok_or(ItemError::UnspecifiedClassAllowList)?,
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

    pub fn class_allow_list(mut self, class_allow_list: Vec<ClassId>) -> Self {
        self.class_allow_list = Some(class_allow_list);
        self
    }
}
