#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    RIGHT,
    DOWN,
    LEFT,
    UP,
}

pub const DIRECTIONS: [Dir; 4] = [Dir::UP, Dir::RIGHT, Dir::DOWN, Dir::LEFT];

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

/// Uses screen coordinate system - X axis points right and Y axis points down.
impl Into<(i32, i32)> for Dir {
    fn into(self) -> (i32, i32) {
        match self {
            Dir::RIGHT => (1, 0),
            Dir::DOWN => (0, 1),
            Dir::LEFT => (-1, 0),
            Dir::UP => (0, -1)
        }
    }
}

impl std::ops::Add<Dir> for (i32, i32) {
    type Output = (i32, i32);

    fn add(self, dir: Dir) -> (i32, i32) {
        let dir: (i32, i32) = dir.into();
        (self.0 + dir.0, self.1 + dir.1)
    }
}

#[test]
fn test_add_dir_to_tuple() {
    assert_eq!((0, 1) + Dir::UP, (0, 0));
    assert_eq!((2, 1) + Dir::RIGHT, (3, 1));
    assert_eq!((-1, -1) + Dir::DOWN, (-1, 0));
    assert_eq!((-2, 1) + Dir::LEFT, (-3, 1));
}
