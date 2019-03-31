use super::direction::{Dir, DIRECTIONS};
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
        *self + DIRECTIONS[dir]
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

