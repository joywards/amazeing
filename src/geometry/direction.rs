#[derive(Debug, Clone, Copy)]
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

    pub fn rotate_anticlockwise(self) -> Dir {
        match self {
            Dir::RIGHT => Dir::UP,
            Dir::DOWN => Dir::RIGHT,
            Dir::LEFT => Dir::DOWN,
            Dir::UP => Dir::LEFT,
        }
    }
}
