#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Scalar(u32);

impl From<Scalar> for u32 {
    fn from(scalar: Scalar) -> Self {
        scalar.0
    }
}

impl From<u32> for Scalar {
    fn from(primitive: u32) -> Self {
        Scalar(primitive)
    }
}

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

#[derive(Serialize, Deserialize)]
pub struct Position {
    x: Scalar,
    y: Scalar,
}

impl Position {
    pub fn from_u32(x: u32, y: u32) -> Self {
        Position {
            x: Scalar(x),
            y: Scalar(y),
        }
    }

    pub fn from_scalar(x: Scalar, y: Scalar) -> Self {
        Position { x, y }
    }

    pub fn x(&self) -> Scalar {
        self.x
    }

    pub fn y(&self) -> Scalar {
        self.y
    }
}
