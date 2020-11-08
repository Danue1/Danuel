use crate::{Direction, Scalar};

#[derive(Serialize, Deserialize)]
pub struct Position {
    x: Scalar,
    y: Scalar,
}

impl Position {
    pub fn from_u32(x: u32, y: u32) -> Self {
        Position {
            x: x.into(),
            y: y.into(),
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

impl std::ops::Add<Direction> for Position {
    type Output = Self;

    fn add(self, direction: Direction) -> Self::Output {
        let (x, y) = match direction {
            Direction::Top => (self.x, self.y + 1),
            Direction::Right => (self.x + 1, self.y),
            Direction::Down => (self.x, self.y - 1),
            Direction::Left => (self.x - 1, self.y),
            Direction::Neutral => (self.x, self.y),
        };

        Position { x, y }
    }
}

impl std::ops::Sub<Direction> for Position {
    type Output = Self;

    fn sub(self, direction: Direction) -> Self::Output {
        let (x, y) = match direction {
            Direction::Top => (self.x, self.y - 1),
            Direction::Right => (self.x - 1, self.y),
            Direction::Down => (self.x, self.y + 1),
            Direction::Left => (self.x + 1, self.y),
            Direction::Neutral => (self.x, self.y),
        };

        Position { x, y }
    }
}
