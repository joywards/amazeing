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
            y: self.x + rhs.y,
        }
    }
}

impl std::ops::Sub for Coord {
    type Output = Coord;
    fn sub(self, rhs: Coord) -> Coord {
        Coord{
            x: self.x - rhs.x,
            y: self.x - rhs.y,
        }
    }
}

impl Ordinal for Coord {
    fn ordinal(coord: Self) -> usize {
        Ordinal::ordinal(&(coord.x, coord.y))
    }
}
