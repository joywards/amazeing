use super::direction::Dir;
use crate::dsu::Ordinal;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Coord {
    x: isize,
    y: isize
}

impl Coord {
    pub fn new(x: isize, y: isize) -> Coord {
        Coord{x, y}
    }

    pub fn advance(&self, dir: Dir) -> Coord {
        *self + dir.into()
    }
}

impl std::ops::Add for Coord {
    type Output = Coord;
    fn add(self, rhs: Coord) -> Coord {
        Coord{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for Coord {
    type Output = Coord;
    fn sub(self, rhs: Coord) -> Coord {
        Coord{
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<X: Into<isize>, Y: Into<isize>> From<(X, Y)> for Coord {
    fn from(tuple: (X, Y)) -> Self {
        Coord {
            x: tuple.0.into(),
            y: tuple.1.into(),
        }
    }
}

/// Uses screen coordinate system - X axis points right and Y axis points down.
impl From<Dir> for Coord {
    fn from(dir: Dir) -> Self {
        match dir {
            Dir::RIGHT => Coord::new(1, 0),
            Dir::DOWN => Coord::new(0, 1),
            Dir::LEFT => Coord::new(-1, 0),
            Dir::UP => Coord::new(0, -1)
        }
    }
}

impl Ordinal for Coord {
    fn ordinal(coord: Self) -> usize {
        Ordinal::ordinal(&(coord.x, coord.y))
    }
}

#[test]
fn test_advance() {
    assert_eq!(Coord::new(0, 1).advance(Dir::UP), Coord::new(0, 0));
    assert_eq!(Coord::new(2, 1).advance(Dir::RIGHT), Coord::new(3, 1));
    assert_eq!(Coord::new(-1, -1).advance(Dir::DOWN), Coord::new(-1, 0));
    assert_eq!(Coord::new(-2, 1).advance(Dir::LEFT), Coord::new(-3, 1));
}

