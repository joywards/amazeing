use crate::maze::Maze;
use crate::traversal::Info;

pub struct Scene {
    pub maze: Maze,
    pub layer_info: Vec<Info>,
}

impl Scene {
    pub fn new(maze: Maze, layer_info: Vec<Info>) -> Scene {
        Scene{ maze, layer_info }
    }
}
