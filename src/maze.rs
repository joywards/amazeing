use std::collections::HashMap;

use crate::layer::Layer;
use crate::geometry::Dir;
use crate::traversal;


#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CellInfo {
    Untouched,
    Visited,
    Finish,
}

#[derive(Copy, Clone)]
pub enum LazyCellInfo {
    Some(CellInfo),
    Ref(usize),
}

impl Default for CellInfo {
    fn default() -> Self {
        CellInfo::Untouched
    }
}

impl Default for LazyCellInfo {
    fn default() -> Self {
        LazyCellInfo::Some(Default::default())
    }
}


#[derive(Clone, Copy)]
struct Transition {
    dest_layer: usize,
}

pub struct MazeLayer {
    pub layer: Layer<LazyCellInfo>,
    transitions: HashMap<(i32, i32), Transition>,
    pub info: traversal::Info,
}

pub struct Maze {
    layers: Vec<MazeLayer>,
    position: (i32, i32),
    current_layer_index: usize,
    // A copy of the current layer is made for speeding up rendering.
    current_layer: Layer<CellInfo>,
}

#[derive(Debug, PartialEq)]
pub enum MoveResult{
    SUCCESS,
    OBSTACLE,
    FINISH,
}

impl Maze {
    fn resolve_references(&self, layer: &Layer<LazyCellInfo>) -> Layer<CellInfo> {
        layer.map(|&info, coord| {
            match info {
                LazyCellInfo::Some(info) => info,
                LazyCellInfo::Ref(to_layer) => {
                    match *self.layers[to_layer].layer.get_info(coord).unwrap() {
                        LazyCellInfo::Some(info) => info,
                        LazyCellInfo::Ref(_) => panic!("LazyCellInfo::Ref leads to another Ref")
                    }
                }
            }
        })
    }

    pub fn new(layer: Layer<LazyCellInfo>, spawn_point: (i32, i32)) -> Maze {
        let mut result = Maze {
            layers: vec![MazeLayer{
                layer: layer.clone(),
                transitions: HashMap::new(),
                info: traversal::dfs(&layer, spawn_point, None),
            }],
            position: spawn_point,
            current_layer_index: 0,
            current_layer: Default::default()
        };
        result.current_layer = result.resolve_references(&result.layers[0].layer);
        result.on_position_updated();
        result
    }

    fn on_position_updated(&mut self) {
        let current_layer = &self.layers[self.current_layer_index];
        if let Some(transition) = current_layer.transitions.get(&self.position) {
            self.current_layer_index = transition.dest_layer;
            self.update_current_level();
        }

        self.modify_cell_info(
            (self.position.0, self.position.1, self.current_layer_index),
            |info| if *info == CellInfo::Untouched { *info = CellInfo::Visited }
        );
    }

    fn update_current_level(&mut self) {
        self.current_layer = self.resolve_references(
            &self.layers[self.current_layer_index].layer
        );
    }

    pub fn try_move(&mut self, dir: Dir) -> MoveResult {
        if self.current_layer.passable(self.position, dir) {
            self.position = self.position + dir;
            self.on_position_updated();
            if self.is_at_finish() {
                MoveResult::FINISH
            } else {
                MoveResult::SUCCESS
            }
        } else {
            MoveResult::OBSTACLE
        }
    }

    pub fn current_layer(&self) -> &Layer<CellInfo> {
        &self.current_layer
    }

    pub fn position(&self) -> (i32, i32) {
        self.position
    }

    pub fn current_layer_info(&self) -> &traversal::Info {
        &self.layers[self.current_layer_index].info
    }

    pub fn maze_layer(&self, i: usize) -> &MazeLayer {
        &self.layers[i]
    }

    pub fn set_finish(&mut self, pos: (i32, i32, usize)) {
        self.modify_cell_info(pos, |info| *info = CellInfo::Finish)
    }

    fn is_at_finish(&self) -> bool {
        *self.current_layer().get_info(self.position).unwrap() == CellInfo::Finish
    }

    fn mut_lazy_cell_info(&mut self, (x, y, z): (i32, i32, usize)) -> Option<&mut LazyCellInfo> {
        self.layers.get_mut(z)?.layer.get_info_mut((x, y))
    }

    pub fn modify_cell_info(
        &mut self, (x, y, z): (i32, i32, usize),
        modify: impl FnOnce(&mut CellInfo)
    ) {
        match *self.mut_lazy_cell_info((x, y, z)).unwrap() {
            LazyCellInfo::Some(ref mut info) => modify(info),
            LazyCellInfo::Ref(to) => {
                match self.mut_lazy_cell_info((x, y, to)).unwrap() {
                    LazyCellInfo::Some(ref mut info) => modify(info),
                    LazyCellInfo::Ref(_) => panic!("LazyCellInfo::Ref leads to another Ref")
                }
            }
        }
        self.update_current_level();
    }

    pub fn add_layer(&mut self, layer: Layer<LazyCellInfo>, info: traversal::Info) -> usize {
        self.layers.push(MazeLayer{
            layer,
            transitions: HashMap::new(),
            info
        });
        self.layers.len() - 1
    }

    pub fn add_transition(&mut self, coord: (i32, i32), dir: Dir, from_index: usize, to_index: usize) {
        let from = &mut self.layers[from_index];
        assert!(from.layer.passable(coord, dir));
        from.transitions.insert(coord + dir, Transition{dest_layer: to_index});

        let to = &mut self.layers[to_index];
        assert!(to.layer.passable(coord, dir));
        to.transitions.insert(coord, Transition{dest_layer: from_index});
    }
}

#[test]
fn test_maze() {
    let mut first = Layer::from_shape(
        &(0..=3).map(|i| (0, i)).collect::<Vec<_>>()
    );
    for i in 1..=3 {
        first.join((0, i), Dir::UP);
    }
    let second = first.clone();

    let mut maze = Maze::new(first, (0, 0));
    let second_layer = maze.add_layer(second, traversal::Info::default());
    maze.add_transition((0, 1), Dir::DOWN, 0, 1);
    maze.set_finish((0, 3, second_layer));

    assert_eq!(maze.try_move(Dir::RIGHT), MoveResult::OBSTACLE);
    assert_eq!(maze.try_move(Dir::DOWN), MoveResult::SUCCESS);
    assert_eq!(maze.position, (0, 1));
    assert_eq!(maze.current_layer_index, 0);
    assert!(!maze.current_layer.has((2, 2)));

    assert_eq!(maze.try_move(Dir::DOWN), MoveResult::SUCCESS);
    assert_eq!(maze.position, (0, 2));
    assert_eq!(maze.current_layer_index, 1);

    assert_eq!(maze.try_move(Dir::DOWN), MoveResult::FINISH);
    assert_eq!(maze.position, (0, 3));
    assert_eq!(maze.current_layer_index, 1);

    assert_eq!(maze.try_move(Dir::UP), MoveResult::SUCCESS);
    assert_eq!(maze.position, (0, 2));
    assert_eq!(maze.current_layer_index, 1);

    assert_eq!(maze.try_move(Dir::UP), MoveResult::SUCCESS);
    assert_eq!(maze.position, (0, 1));
    assert_eq!(maze.current_layer_index, 0);

    assert_eq!(maze.try_move(Dir::UP), MoveResult::SUCCESS);
    assert_eq!(maze.position, (0, 0));
    assert_eq!(maze.current_layer_index, 0);
    assert_eq!(maze.try_move(Dir::UP), MoveResult::OBSTACLE);
}
