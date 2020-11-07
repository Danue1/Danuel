#[derive(Eq, PartialEq, Hash, Clone, Serialize, Deserialize)]
pub struct FixtureId(u32);

impl From<u32> for FixtureId {
    fn from(id: u32) -> Self {
        FixtureId(id)
    }
}

impl From<FixtureId> for u32 {
    fn from(id: FixtureId) -> Self {
        id.0
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Fixture<Image> {
    image: Image,
}
