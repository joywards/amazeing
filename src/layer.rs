use disjoint_sets::UnionFind;
use crate::geometry::Dir;


#[derive(Default, Clone, Copy)]
struct Cell<Info> {
    has_passage_right: bool,
    has_passage_down: bool,
    info: Info,
}

impl<Info> Cell<Info> {
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
pub struct Layer<CellInfo: Default> {
    cells: Vec<Option<Cell<CellInfo>>>,
    min_i: i32,
    min_j: i32,
    stride: i32,
    height: i32,
    dsu: UnionFind<usize>,
}

impl<CellInfo: Default> Layer<CellInfo> {
    pub fn from_shape(coords: &[(i32, i32)]) -> Self {
        assert!(!coords.is_empty());
        let &(min_i, _j) = coords.iter().min_by_key(|(i, _j)| i).unwrap();
        let &(_i, min_j) = coords.iter().min_by_key(|(_i, j)| j).unwrap();
        let &(max_i, _j) = coords.iter().max_by_key(|(i, _j)| i).unwrap();
        let &(_i, max_j) = coords.iter().max_by_key(|(_i, j)| j).unwrap();
        let stride = max_j - min_j + 1;
        let height = max_i - min_i + 1;
        let len = (height * stride) as usize;

        let mut result = Self {
            cells: std::iter::repeat_with(|| None).take(len).collect(),
            min_i, min_j, stride, height,
            dsu: UnionFind::new(len),
        };
        for &coord in coords {
            let index = result.index(coord).unwrap();
            result.cells[index] = Some(Default::default());
        }
        result
    }

    fn index(&self, (i, j): (i32, i32)) -> Option<usize> {
        let i = i - self.min_i;
        let j = j - self.min_j;
        if i < 0 || j < 0 || j >= self.stride || i >= self.height {
            None
        } else {
            Some((i * self.stride + j) as usize)
        }
    }

    fn pos_from_index(&self, index: usize) -> (i32, i32) {
        (
            index as i32 / self.stride + self.min_i,
            index as i32 % self.stride + self.min_j
        )
    }

    fn get(&self, coord: (i32, i32)) -> Option<&Cell<CellInfo>> {
        let index = self.index(coord)?;
        self.cells.get(index).and_then(|x| x.as_ref())
    }

    fn get_mut(&mut self, coord: (i32, i32)) -> Option<&mut Cell<CellInfo>> {
        let index = self.index(coord)?;
        self.cells.get_mut(index).and_then(|x| x.as_mut())
    }

    pub fn get_info(&self, coord: (i32, i32)) -> Option<&CellInfo> {
        self.get(coord).map(|cell| &cell.info)
    }

    pub fn get_info_mut(&mut self, coord: (i32, i32)) -> Option<&mut CellInfo> {
        self.get_mut(coord).map(|cell| &mut cell.info)
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

                let cell = self.get_mut(from).expect(MSG);
                *cell.get_passage_mut(dir) = true;
                self.dsu.union(self.index(from).unwrap(), self.index(to).expect(MSG));
            }
        }
    }

    pub fn reachable(&self, from: (i32, i32), to: (i32, i32)) -> bool {
        if let (Some(from_i), Some(to_i)) = (self.index(from), self.index(to)) {
            self.dsu.equiv(from_i, to_i)
        } else {
            false
        }
    }

    pub fn treat_as_reachable(&mut self, a: (i32, i32), b: (i32, i32)) {
        self.dsu.union(self.index(a).unwrap(), self.index(b).unwrap());
    }

    pub fn map<ResultInfo: Default>(
        &self,
        f: impl Fn(&CellInfo, (i32, i32)) -> ResultInfo
    ) -> Layer<ResultInfo> {
        Layer {
            cells: self.cells.iter().enumerate().map(|(index, opt)| opt.as_ref().map(
                |cell| Cell{
                    has_passage_right: cell.has_passage_right,
                    has_passage_down: cell.has_passage_down,
                    info: f(&cell.info, self.pos_from_index(index))
                }
            )).collect(),
            min_i: self.min_i,
            min_j: self.min_j,
            stride: self.stride,
            height: self.height,
            dsu: self.dsu.clone()
        }
    }
}

impl<CellInfo: Default> Default for Layer<CellInfo> {
    fn default() -> Self {
        Self {
            cells: Default::default(),
            min_i: 0,
            min_j: 0,
            stride: 0,
            height: 0,
            dsu: Default::default()
        }
    }
}

#[test]
fn test_layer() {
    use itertools::Itertools;

    let shape = [
        (0, 0), (0, 1), (1, 0), (-1, -2), (-1, 0)
    ];
    let mut layer = Layer::<()>::from_shape(&shape);
    for &coord in &shape {
        assert!(layer.has(coord));
    }
    let count: usize = (-10..10).cartesian_product(-10..10).map(
        |coord| if layer.has(coord) { 1 } else { 0 }
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

    layer.treat_as_reachable((-1, -2), (0, 0));
    assert!(layer.reachable((-1, -2), (0, 1)));
    assert!(!layer.reachable((-1, -2), (-1, 0)));
}

#[test]
fn test_indexing() {
    use itertools::Itertools;

    let shape = (-3..=3).cartesian_product(-3..=3).collect::<Vec<_>>();
    let layer = Layer::<()>::from_shape(&shape);

    for coord in (-10..10).cartesian_product(-10..10) {
        if let Some(index) = layer.index(coord) {
            assert_eq!(coord, layer.pos_from_index(index))
        }
    }
}
