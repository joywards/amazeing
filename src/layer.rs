use crate::utils::dsu::DSU;
use crate::geometry::Dir;

#[derive(Default, Clone, Copy)]
struct Cell {
    has_passage_right: bool,
    has_passage_down: bool,
}

impl Cell {
    fn get_passage_mut(&mut self, dir: Dir) -> &mut bool {
        match dir {
            Dir::RIGHT => &mut self.has_passage_right,
            Dir::DOWN => &mut self.has_passage_down,
            Dir::UP => panic!("Cell has no passage up"),
            Dir::LEFT => panic!("Cell has no passage left"),
        }
    }
}

#[derive(Clone)]
pub struct Layer {
    cells: Vec<Option<Cell>>,
    min_i: i32,
    min_j: i32,
    stride: i32,
    dsu: DSU<(i32, i32)>,
}

impl Layer {
    pub fn from_shape(coords: &[(i32, i32)]) -> Self {
        assert!(!coords.is_empty());
        let &(min_i, _j) = coords.iter().min_by_key(|(i, _j)| i).unwrap();
        let &(_i, min_j) = coords.iter().min_by_key(|(_i, j)| j).unwrap();
        let &(max_i, _j) = coords.iter().max_by_key(|(i, _j)| i).unwrap();
        let &(_i, max_j) = coords.iter().max_by_key(|(_i, j)| j).unwrap();
        let stride = max_j - min_j + 1;
        let len = ((max_i - min_i + 1) * stride) as usize;

        let mut result = Self {
            cells: vec![None; len],
            min_i, min_j, stride,
            dsu: DSU::default()
        };
        for &coord in coords {
            let index = result.index(coord).unwrap();
            result.cells[index] = Some(Cell::default());
        }
        result
    }

    fn index(&self, (i, j): (i32, i32)) -> Option<usize> {
        if i < self.min_i || j < self.min_j || j - self.min_j >= self.stride {
            None
        } else {
            Some(((i - self.min_i) * self.stride + (j - self.min_j)) as usize)
        }
    }

    fn get(&self, coord: (i32, i32)) -> Option<&Cell> {
        let index = self.index(coord)?;
        self.cells.get(index).and_then(|x| x.as_ref())
    }

    fn get_mut(&mut self, coord: (i32, i32)) -> Option<&mut Cell> {
        let index = self.index(coord)?;
        self.cells.get_mut(index).and_then(|x| x.as_mut())
    }

    pub fn has(&self, coord: (i32, i32)) -> bool {
        self.get(coord).is_some()
    }

    pub fn passable(&self, from: (i32, i32), dir: Dir) -> bool {
        let cell = match self.get(from) {
            None => return false,
            Some(value) => value,
        };
        match dir {
            Dir::LEFT | Dir::UP =>
                self.passable(
                    from + dir,
                    dir.opposite()
                ),
            Dir::RIGHT => cell.has_passage_right,
            Dir::DOWN => cell.has_passage_down,
        }
    }

    pub fn join(&mut self, from: (i32, i32), dir: Dir) {
        match dir {
            Dir::LEFT | Dir::UP =>
                self.join(
                    from + dir,
                    dir.opposite()
                ),
            Dir::RIGHT | Dir::DOWN => {
                let to = from + dir;
                const MSG: &str = "Trying to join with cell outside the layer";
                assert!(self.has(to), MSG);

                let cell: &mut Cell = self.get_mut(from).expect(MSG);
                *cell.get_passage_mut(dir) = true;
                self.dsu.union(from, to);
            }
        }
    }

    pub fn reachable(&self, from: (i32, i32), to: (i32, i32)) -> bool {
        self.dsu.equiv(from, to)
    }
}

#[test]
fn test_layer() {
    use itertools::Itertools;
    let shape = [
        (0, 0), (0, 1), (1, 0), (-1, -2), (-1, 0)
    ];
    let mut layer = Layer::from_shape(&shape);
    for &cell in &shape {
        assert!(layer.has(cell));
    }
    let count: usize = (-10..10).cartesian_product(-10..10).map(
        |cell| if layer.has(cell) { 1 } else { 0 }
    ).sum();
    assert_eq!(count, shape.len());

    layer.join((0, 0), Dir::RIGHT);
    assert!(layer.passable((1, 0), Dir::LEFT));
    assert!(!layer.passable((0, 0), Dir::DOWN));
    assert!(!layer.passable((0, 1), Dir::DOWN));
    assert!(!layer.reachable((1, 0), (0, 1)));
    layer.join((0, 1), Dir::UP);
    assert!(layer.passable((0, 0), Dir::DOWN));
    assert!(layer.reachable((1, 0), (0, 1)));
}
