use crate::Named;

pub struct Reward {
    name: String,
}

impl Reward {
    pub fn new(name: String) -> Self {
        Reward { name }
    }
}

impl Named for Reward {
    fn name(&self) -> &str {
        self.name.as_ref()
    }
}
