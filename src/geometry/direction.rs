//! Uses screen coordinate system - X axis points right and Y axis points down.

use enum_map::EnumMap;
use super::coord::Coord;

#[derive(Debug, Clone, Copy, Enum)]
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

lazy_static! {
    pub static ref DIRECTIONS: EnumMap<Dir, Coord> = enum_map! {
        Dir::RIGHT => Coord::new(1, 0),
        Dir::DOWN => Coord::new(0, 1),
        Dir::LEFT => Coord::new(-1, 0),
        Dir::UP => Coord::new(0, -1)
    };
}
