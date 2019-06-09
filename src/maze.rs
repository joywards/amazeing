use std::sync::Mutex;
use std::collections::HashMap;

use crate::layer::Layer;
use crate::geometry::coord::Coord;
use crate::geometry::direction::Dir;

#[derive(Clone, Copy)]
struct Transition {
    dest_layer: usize,
}

#[derive(Clone)]
struct MazeLayer {
    layer: Layer,
    transitions: HashMap<Coord, Transition>,
}

pub struct Maze {
    layers: Mutex<Vec<MazeLayer>>,
    position: Coord,
    current_layer_index: usize,
    // A copy of current layer is made for avoiding
    // thread syncronization during rendering.
    current_layer: Layer,
}

#[derive(Debug, PartialEq)]
pub enum MoveResult{
    SUCCESS,
    OBSTACLE,
}

impl Maze {
    pub fn new(layer: Layer, spawn_point: Coord) -> Maze {
        Maze{
            layers: Mutex::new(vec![MazeLayer{
                layer: layer.clone(),
                transitions: HashMap::new()
            }]),
            position: spawn_point,
            current_layer_index: 0,
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
            self.position = self.position.advance(dir);
            self.change_layer_if_necessary();
            MoveResult::SUCCESS
        } else {
            MoveResult::OBSTACLE
        }
    }

    pub fn current_layer(&self) -> &Layer {
        &self.current_layer
    }

    pub fn position(&self) -> Coord {
        self.position
    }

    pub fn add_layer(&self, layer: Layer) -> usize {
        let mut layers = self.layers.lock().unwrap();
        layers.push(MazeLayer{
            layer,
            transitions: HashMap::new()
        });
        layers.len() - 1
    }

    pub fn add_transition(&self, coord: Coord, dir: Dir, from_index: usize, to_index: usize) {
        let mut layers = self.layers.lock().unwrap();

        let from = &mut layers[from_index];
        assert!(from.layer.passable(coord, dir));
        from.transitions.insert(coord.advance(dir), Transition{dest_layer: to_index});

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
        first.join((0, i).into(), Dir::UP);
    }
    let mut second = first.clone();
    second.add((2, 2));

    let mut maze = Maze::new(first, (0, 0).into());
    maze.add_layer(second);
    maze.add_transition((0, 1).into(), Dir::DOWN, 0, 1);

    assert_eq!(maze.try_move(Dir::RIGHT), MoveResult::OBSTACLE);
    assert_eq!(maze.try_move(Dir::DOWN), MoveResult::SUCCESS);
    assert_eq!(maze.position, (0, 1).into());
    assert_eq!(maze.current_layer_index, 0);
    assert!(!maze.current_layer.has((2, 2)));

    assert_eq!(maze.try_move(Dir::DOWN), MoveResult::SUCCESS);
    assert_eq!(maze.position, (0, 2).into());
    assert_eq!(maze.current_layer_index, 1);
    assert!(maze.current_layer.has((2, 2)));

    assert_eq!(maze.try_move(Dir::DOWN), MoveResult::SUCCESS);
    assert_eq!(maze.position, (0, 3).into());
    assert_eq!(maze.current_layer_index, 1);

    assert_eq!(maze.try_move(Dir::UP), MoveResult::SUCCESS);
    assert_eq!(maze.position, (0, 2).into());
    assert_eq!(maze.current_layer_index, 1);

    assert_eq!(maze.try_move(Dir::UP), MoveResult::SUCCESS);
    assert_eq!(maze.position, (0, 1).into());
    assert_eq!(maze.current_layer_index, 0);

    assert_eq!(maze.try_move(Dir::UP), MoveResult::SUCCESS);
    assert_eq!(maze.position, (0, 0).into());
    assert_eq!(maze.current_layer_index, 0);
    assert_eq!(maze.try_move(Dir::UP), MoveResult::OBSTACLE);
}