use crate::maze::Maze;
use crate::traversal::Info;

pub type Camera = (f32, f32);

pub struct Scene {
    pub maze: Maze,
    pub layer_info: Vec<Info>,
    pub camera: Camera,
}

impl Scene {
    pub fn new(maze: Maze, layer_info: Vec<Info>) -> Scene {
        Scene{ maze, layer_info, camera: (0.0, 0.0) }
    }

    pub fn update(&mut self) {
        let pos = self.maze.position();
        self.camera = (pos.0 as f32, pos.1 as f32);
    }
}
