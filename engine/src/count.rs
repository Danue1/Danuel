#[derive(Serialize, Deserialize)]
pub struct Count(u8);

impl From<u8> for Count {
    fn from(count: u8) -> Self {
        Count(count)
    }
}

impl From<Count> for u8 {
    fn from(count: Count) -> Self {
        count.0
    }
}
