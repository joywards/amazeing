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
    pub parent_layer_index: usize,
}

pub struct Maze {
    layers: Vec<MazeLayer>,
    position: (i32, i32),
    current_layer_index: usize,
    // A copy of the current layer is made for speeding up rendering.
    current_layer: Layer<CellInfo>,
    path_from_start: Vec<Dir>,
    path_from_finish: Vec<Dir>,
}

#[derive(Debug, PartialEq)]
pub enum MoveResult{
    MovedToVisited,
    MovedToUntouched,
    Obstacle,
    Finish,
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
                parent_layer_index: 0,
            }],
            position: spawn_point,
            current_layer_index: 0,
            current_layer: Default::default(),
            path_from_start: Vec::new(),
            path_from_finish: Vec::new(),
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

    fn update_path(path: &mut Vec<Dir>, dir: Dir) {
        if let Some(&last) = path.last() {
            if last == dir.opposite() {
                path.pop();
                return;
            }
        }
        path.push(dir);
    }

    fn do_move(&mut self, dir: Dir) {
        self.position = self.position + dir;
        Self::update_path(&mut self.path_from_start, dir);
        Self::update_path(&mut self.path_from_finish, dir);
        self.on_position_updated();
    }

    pub fn try_move(&mut self, dir: Dir) -> MoveResult {
        if self.current_layer.passable(self.position, dir) {
            let new_position = self.position + dir;
            let cell_state = *self.current_layer.get_info(new_position).unwrap();
            self.do_move(dir);
            match cell_state {
                CellInfo::Finish => MoveResult::Finish,
                CellInfo::Untouched => MoveResult::MovedToUntouched,
                CellInfo::Visited => MoveResult::MovedToVisited,
            }
        } else {
            MoveResult::Obstacle
        }
    }

    pub fn try_move_towards_start(&mut self) -> MoveResult {
        if !self.path_from_start.is_empty() {
            self.try_move(self.path_from_start.last().unwrap().opposite())
        } else {
            MoveResult::Obstacle
        }
    }

    pub fn try_move_towards_finish(&mut self) -> MoveResult {
        if !self.path_from_finish.is_empty() {
            self.try_move(self.path_from_finish.last().unwrap().opposite())
        } else {
            MoveResult::Obstacle
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
        self.modify_cell_info(pos, |info| *info = CellInfo::Finish);
        assert!(self.path_from_finish.is_empty(), "Finish is already set");
        self.update_path_from_finish(pos);
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

    pub fn add_layer(
        &mut self,
        layer: Layer<LazyCellInfo>, info: traversal::Info,
        parent_layer_index: usize
    ) -> usize {
        self.layers.push(MazeLayer{
            layer,
            transitions: HashMap::new(),
            info,
            parent_layer_index
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

    fn update_path_from_finish(&mut self, finish: (i32, i32, usize)) {
        self.path_from_finish.clear();

        let mut pos = (finish.0, finish.1);
        let mut layer_index = finish.2;
        while layer_index != self.current_layer_index || pos != self.position {
            let layer = &self.maze_layer(layer_index);
            match layer.info.coords[&pos].came_from {
                Some(dir) => {
                    self.path_from_finish.push(dir);
                    pos = pos + dir;
                },
                None => {
                    layer_index = layer.parent_layer_index;
                },
            };
        }
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
    let mut second = first.clone();
    for i in 1..=3 {
        *second.get_info_mut((0, i)).unwrap() = LazyCellInfo::Ref(0);
    }

    let mut maze = Maze::new(first, (0, 0));
    let info = traversal::dfs(&second, (0, 2), Some(Dir::UP));
    let second_layer = maze.add_layer(second, info, 0);
    maze.add_transition((0, 1), Dir::DOWN, 0, 1);
    maze.set_finish((0, 3, second_layer));

    assert_eq!(maze.try_move(Dir::RIGHT), MoveResult::Obstacle);
    assert_eq!(maze.try_move(Dir::DOWN), MoveResult::MovedToUntouched);
    assert_eq!(maze.position, (0, 1));
    assert_eq!(maze.current_layer_index, 0);
    assert!(!maze.current_layer.has((2, 2)));

    assert_eq!(maze.try_move(Dir::DOWN), MoveResult::MovedToUntouched);
    assert_eq!(maze.position, (0, 2));
    assert_eq!(maze.current_layer_index, 1);

    assert_eq!(maze.try_move(Dir::DOWN), MoveResult::Finish);
    assert_eq!(maze.position, (0, 3));
    assert_eq!(maze.current_layer_index, 1);

    assert_eq!(maze.try_move(Dir::UP), MoveResult::MovedToVisited);
    assert_eq!(maze.position, (0, 2));
    assert_eq!(maze.current_layer_index, 1);

    assert_eq!(maze.try_move(Dir::UP), MoveResult::MovedToVisited);
    assert_eq!(maze.position, (0, 1));
    assert_eq!(maze.current_layer_index, 0);

    assert_eq!(maze.try_move(Dir::UP), MoveResult::MovedToVisited);
    assert_eq!(maze.position, (0, 0));
    assert_eq!(maze.current_layer_index, 0);
    assert_eq!(maze.try_move(Dir::UP), MoveResult::Obstacle);
}
