use crate::{Position, Scalar};

#[derive(Serialize, Deserialize)]
pub struct Positioned<T: Sized> {
    position: Position,
    node: T,
}

impl<T: Sized> std::ops::Deref for Positioned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}

impl<T: Sized> Positioned<T> {
    pub fn x(&self) -> Scalar {
        self.position.x()
    }

    pub fn y(&self) -> Scalar {
        self.position.y()
    }
}
