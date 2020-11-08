use crate::Id;

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
