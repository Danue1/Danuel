use crate::Id;

#[derive(Eq, PartialEq, Hash, Clone, Serialize, Deserialize)]
pub struct ClassId(Id);

impl From<Id> for ClassId {
    fn from(id: Id) -> Self {
        ClassId(id)
    }
}

impl From<ClassId> for Id {
    fn from(id: ClassId) -> Self {
        id.0
    }
}

#[derive(Serialize, Deserialize)]
pub struct Class {
    name: String,
}
