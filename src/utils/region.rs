use std::collections::HashSet;
use itertools::Itertools;
use crate::geometry_sets::dilate;

#[derive(Debug, Clone)]
pub struct Region {
    cells: HashSet<(i32, i32)>,
    boundary: HashSet<(i32, i32)>,
}

impl From<HashSet<(i32, i32)>> for Region {
    fn from(cells: HashSet<(i32, i32)>) -> Self {
        let mut boundary = dilate(&cells, &(-1..=1).cartesian_product(-1..=1).collect());
        for cell in &cells {
            boundary.remove(cell);
        }
        boundary.shrink_to_fit();

        Region{cells, boundary}
    }
}

impl From<&HashSet<(i32, i32)>> for Region {
    fn from(cells: &HashSet<(i32, i32)>) -> Self {
        cells.clone().into()
    }
}

impl Region {
    pub fn cells(&self) -> &HashSet<(i32, i32)> {
        &self.cells
    }

    pub fn boundary(&self) -> &HashSet<(i32, i32)> {
        &self.boundary
    }

    pub fn shifted_by(&self, origin: (i32, i32)) -> Region {
        let mapper = |&c: &(i32, i32)| (c.0 + origin.0, c.1 + origin.1);
        Region{
            cells: self.cells.iter().map(mapper).collect(),
            boundary: self.boundary.iter().map(mapper).collect(),
        }
    }
}
