use crate::Id;

#[derive(Eq, PartialEq, Hash, Clone, Serialize, Deserialize)]
pub struct FixtureId(Id);

impl From<Id> for FixtureId {
    fn from(id: Id) -> Self {
        FixtureId(id)
    }
}

impl From<FixtureId> for Id {
    fn from(id: FixtureId) -> Self {
        id.0
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Fixture<Image> {
    image: Image,
}
