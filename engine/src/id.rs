#[derive(Eq, PartialEq, Hash, Clone, Serialize, Deserialize)]
pub struct Id(u32);

impl From<u32> for Id {
    fn from(id: u32) -> Self {
        Id(id)
    }
}

impl From<Id> for u32 {
    fn from(id: Id) -> Self {
        id.0
    }
}
