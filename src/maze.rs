use std::sync::Mutex;
use std::collections::HashMap;

use crate::layer::Layer;
use crate::geometry::Dir;
use crate::traversal;

#[derive(Clone, Copy)]
struct Transition {
    dest_layer: usize,
}

pub struct MazeLayer {
    pub layer: Layer,
    transitions: HashMap<(i32, i32), Transition>,
    pub info: traversal::Info,
}

pub struct Maze {
    layers: Mutex<Vec<MazeLayer>>,
    position: (i32, i32),
    current_layer_index: usize,
    finish: (i32, i32, usize),
    // A copy of current layer is made for avoiding
    // thread synchronization during rendering.
    current_layer: Layer,
}

#[derive(Debug, PartialEq)]
pub enum MoveResult{
    SUCCESS,
    OBSTACLE,
    FINISH,
}

impl Maze {
    pub fn new(layer: Layer, spawn_point: (i32, i32)) -> Maze {
        Maze{
            layers: Mutex::new(vec![MazeLayer{
                layer: layer.clone(),
                transitions: HashMap::new(),
                info: traversal::dfs(&layer, spawn_point, None),
            }]),
            position: spawn_point,
            current_layer_index: 0,
            finish: (0, 0, 0),
            current_layer: layer
        }
    }

    fn change_layer_if_necessary(&mut self) {
        let layers = self.layers.lock().unwrap();
        let current_layer = &layers[self.current_layer_index];
        if let Some(transition) = current_layer.transitions.get(&self.position) {
            self.current_layer = layers[transition.dest_layer].layer.clone();
            self.current_layer_index = transition.dest_layer;
        }
    }

    pub fn try_move(&mut self, dir: Dir) -> MoveResult {
        if self.current_layer.passable(self.position, dir) {
            self.position = self.position + dir;
            self.change_layer_if_necessary();
            if self.is_at_finish() {
                MoveResult::FINISH
            } else {
                MoveResult::SUCCESS
            }
        } else {
            MoveResult::OBSTACLE
        }
    }

    pub fn current_layer(&self) -> &Layer {
        &self.current_layer
    }

    pub fn current_layer_index(&self) -> usize {
        self.current_layer_index
    }

    pub fn position(&self) -> (i32, i32) {
        self.position
    }

    pub fn clone_current_layer_info(&self) -> traversal::Info {
        self.layers.lock().unwrap()[self.current_layer_index].info.clone()
    }

    // Uses mut reference because this way no runtime synchronization is needed.
    pub fn maze_layer(&mut self, i: usize) -> &MazeLayer {
        &self.layers.get_mut().unwrap()[i]
    }

    pub fn set_finish(&mut self, layer_index: usize, pos: (i32, i32)) {
        self.finish = (pos.0, pos.1, layer_index);
    }

    pub fn finish(&self) -> (i32, i32, usize) {
        self.finish
    }

    fn is_at_finish(&self) -> bool {
        (
            self.current_layer_index == self.finish.2
            && self.position == (self.finish.0, self.finish.1)
        )
    }

    pub fn add_layer(&self, layer: Layer, info: traversal::Info) -> usize {
        let mut layers = self.layers.lock().unwrap();
        layers.push(MazeLayer{
            layer,
            transitions: HashMap::new(),
            info
        });
        layers.len() - 1
    }

    pub fn add_transition(&self, coord: (i32, i32), dir: Dir, from_index: usize, to_index: usize) {
        let mut layers = self.layers.lock().unwrap();

        let from = &mut layers[from_index];
        assert!(from.layer.passable(coord, dir));
        from.transitions.insert(coord + dir, Transition{dest_layer: to_index});

        let to = &mut layers[to_index];
        assert!(to.layer.passable(coord, dir));
        to.transitions.insert(coord, Transition{dest_layer: from_index});
    }
}

#[test]
fn test_maze() {
    let mut first: Layer = Layer::default();
    first.add((0, 0));
    for i in 1..=3 {
        first.add((0, i));
        first.join((0, i), Dir::UP);
    }
    let mut second = first.clone();
    second.add((2, 2));

    let mut maze = Maze::new(first, (0, 0));
    let second_layer = maze.add_layer(second, traversal::Info::default());
    maze.add_transition((0, 1), Dir::DOWN, 0, 1);
    maze.set_finish(second_layer, (0, 3));

    assert_eq!(maze.try_move(Dir::RIGHT), MoveResult::OBSTACLE);
    assert_eq!(maze.try_move(Dir::DOWN), MoveResult::SUCCESS);
    assert_eq!(maze.position, (0, 1));
    assert_eq!(maze.current_layer_index, 0);
    assert!(!maze.current_layer.has((2, 2)));

    assert_eq!(maze.try_move(Dir::DOWN), MoveResult::SUCCESS);
    assert_eq!(maze.position, (0, 2));
    assert_eq!(maze.current_layer_index, 1);
    assert!(maze.current_layer.has((2, 2)));

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
