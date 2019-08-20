use crate::dsu::DSU;
use crate::geometry::Dir;
use std::collections::HashMap;

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

#[derive(Default, Clone)]
pub struct Layer {
    cells: HashMap<(i32, i32), Cell>,
    dsu: DSU<(i32, i32)>,
}

impl Layer {
    pub fn has(&self, coord: (i32, i32)) -> bool {
        self.cells.contains_key(&coord)
    }

    pub fn add(&mut self, coord: (i32, i32)) {
        self.cells.entry(coord).or_insert_with(Default::default);
    }

    pub fn passable(&self, from: (i32, i32), dir: Dir) -> bool {
        let cell;
        match self.cells.get(&from) {
            None => return false,
            Some(value) => cell = value,
        }
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

                let cell = self.cells.get_mut(&from).expect(MSG);
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
    let mut layer: Layer = Default::default();
    assert!(!layer.has((0, 0)));
    layer.add((0, 0));
    layer.add((0, 1));
    layer.add((1, 0));
    assert!(layer.has((0, 0)));
    assert!(!layer.has((1, 1)));

    layer.join((0, 0), Dir::RIGHT);
    assert!(layer.passable((1, 0), Dir::LEFT));
    assert!(!layer.passable((0, 0), Dir::DOWN));
    assert!(!layer.reachable((1, 0), (0, 1)));
    layer.join((0, 1), Dir::UP);
    assert!(layer.passable((0, 0), Dir::DOWN));
    assert!(layer.reachable((1, 0), (0, 1)));
}
