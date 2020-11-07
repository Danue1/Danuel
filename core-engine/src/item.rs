use crate::{Builder, Named};

#[derive(Debug)]
pub enum ItemError {
    UnspecifiedName,
    UnspecifiedCount,
}

pub struct Item {
    name: String,
    count: u8,
}

impl Item {
    pub fn count(&self) -> u8 {
        self.count
    }
}

impl Named for Item {
    fn name(&self) -> &str {
        self.name.as_ref()
    }
}

#[derive(Default)]
pub struct ItemBuilder {
    name: Option<String>,
    count: Option<u8>,
}

impl Builder for ItemBuilder {
    type Target = Item;
    type Error = ItemError;

    fn new() -> Self {
        Default::default()
    }

    fn build(self) -> Result<Self::Target, Self::Error> {
        Ok(Item {
            name: self.name.ok_or(ItemError::UnspecifiedName)?,
            count: self.count.ok_or(ItemError::UnspecifiedCount)?,
        })
    }
}

impl ItemBuilder {
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn count(mut self, count: u8) -> Self {
        self.count = Some(count);
        self
    }
}
