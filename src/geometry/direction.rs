#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    RIGHT,
    DOWN,
    LEFT,
    UP,
}

impl Dir {
    pub fn opposite(self) -> Dir {
        match self {
            Dir::RIGHT => Dir::LEFT,
            Dir::DOWN => Dir::UP,
            Dir::LEFT => Dir::RIGHT,
            Dir::UP => Dir::DOWN,
        }
    }

    pub fn rotate_clockwise(self) -> Dir {
        match self {
            Dir::RIGHT => Dir::DOWN,
            Dir::DOWN => Dir::LEFT,
            Dir::LEFT => Dir::UP,
            Dir::UP => Dir::RIGHT,
        }
    }

    #[allow(dead_code)]
    pub fn rotate_anticlockwise(self) -> Dir {
        match self {
            Dir::RIGHT => Dir::UP,
            Dir::DOWN => Dir::RIGHT,
            Dir::LEFT => Dir::DOWN,
            Dir::UP => Dir::LEFT,
        }
    }
}

pub const DIRECTIONS: [Dir; 4] = [Dir::UP, Dir::RIGHT, Dir::DOWN, Dir::LEFT];
