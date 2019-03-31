use crate::dsu::DSU;
use crate::geometry::coord::Coord;
use crate::geometry::direction::Dir;
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
struct Layer {
    cells: HashMap<Coord, Cell>,
    dsu: DSU<Coord>,
}

impl Layer {
    pub fn inside(&self, coord: &Coord) -> bool {
        self.cells.contains_key(coord)
    }

    pub fn add(&mut self, coord: &Coord) {
        self.cells.insert(*coord, Default::default());
    }

    pub fn passable(&self, from: &Coord, dir: Dir) -> bool {
        let cell;
        match self.cells.get(from) {
            None => return false,
            Some(value) => cell = value,
        }
        match dir {
            Dir::LEFT | Dir::UP =>
                self.passable(
                    &from.advance(dir),
                    dir.opposite()
                ),
            Dir::RIGHT => cell.has_passage_right,
            Dir::DOWN => cell.has_passage_down,
        }
    }

    pub fn join(&mut self, from: &Coord, dir: Dir) {
        match dir {
            Dir::LEFT | Dir::UP =>
                self.join(
                    &from.advance(dir),
                    dir.opposite()
                ),
            Dir::RIGHT | Dir::DOWN => {
                let to = from.advance(dir);
                const MSG: &str = "Trying to join with cell outside the layer";
                assert!(self.inside(&to), MSG);

                let cell = self.cells.get_mut(from).expect(MSG);
                *cell.get_passage_mut(dir) = true;
                self.dsu.union(*from, to);
            }
        }
    }

    pub fn reachable(&self, from: &Coord, to: &Coord) -> bool {
        self.dsu.equiv(*from, *to)
    }
}

#[test]
fn test_layer() {
    let mut layer: Layer = Default::default();
    assert!(!layer.inside(&Coord::new(0, 0)));
    layer.add(&Coord::new(0, 0));
    layer.add(&Coord::new(0, 1));
    layer.add(&Coord::new(1, 0));
    assert!(layer.inside(&Coord::new(0, 0)));
    assert!(!layer.inside(&Coord::new(1, 1)));

    layer.join(&Coord::new(0, 0), Dir::RIGHT);
    assert!(layer.passable(&Coord::new(1, 0), Dir::LEFT));
    assert!(!layer.passable(&Coord::new(0, 0), Dir::DOWN));
    assert!(!layer.reachable(&Coord::new(1, 0), &Coord::new(0, 1)));
    layer.join(&Coord::new(0, 1), Dir::UP);
    assert!(layer.passable(&Coord::new(0, 0), Dir::DOWN));
    assert!(layer.reachable(&Coord::new(1, 0), &Coord::new(0, 1)));
}
