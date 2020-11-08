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

impl std::ops::Add<u32> for Scalar {
    type Output = Self;

    fn add(self, scalar: u32) -> Self::Output {
        Scalar(self.0 + scalar)
    }
}

impl std::ops::Sub<u32> for Scalar {
    type Output = Self;

    fn sub(self, scalar: u32) -> Self::Output {
        Scalar(self.0 - scalar)
    }
}
