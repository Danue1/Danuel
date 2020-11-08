use crate::Id;

#[derive(Eq, PartialEq, Hash, Clone, Serialize, Deserialize)]
pub struct CharacterId(Id);

impl From<Id> for CharacterId {
    fn from(id: Id) -> Self {
        CharacterId(id)
    }
}

impl From<CharacterId> for Id {
    fn from(id: CharacterId) -> Self {
        id.0
    }
}
