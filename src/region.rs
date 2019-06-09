use std::collections::HashSet;
use itertools::Itertools;
use crate::geometry::coord::Coord;

#[derive(Debug, Clone)]
pub struct Region {
    cells: HashSet<Coord>,
    boundary: HashSet<Coord>,
}

impl From<HashSet<Coord>> for Region {
    fn from(cells: HashSet<Coord>) -> Self {
        let mut boundary = HashSet::new();
        for cell in &cells {
            for (dx, dy) in (-1..=1).cartesian_product(-1..=1) {
                boundary.insert((cell.x + dx, cell.y + dy).into());
            }
        }
        for cell in &cells {
            boundary.remove(cell);
        }
        boundary.shrink_to_fit();

        Region{cells, boundary}
    }
}

impl From<&HashSet<Coord>> for Region {
    fn from(cells: &HashSet<Coord>) -> Self {
        cells.clone().into()
    }
}

impl Region {
    pub fn cells(&self) -> &HashSet<Coord> {
        &self.cells
    }

    pub fn boundary(&self) -> &HashSet<Coord> {
        &self.boundary
    }

    pub fn shifted_by(&self, origin: Coord) -> Region {
        let mapper = |&c| c + origin;
        Region{
            cells: self.cells.iter().map(mapper).collect(),
            boundary: self.boundary.iter().map(mapper).collect(),
        }
    }
}