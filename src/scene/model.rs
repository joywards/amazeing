use std::time::Duration;

use crate::maze::Maze;
use crate::utils::tuple_arithmetic::linear_interpolation;


pub type Camera = (f32, f32);

pub struct Scene {
    pub maze: Maze,
    pub camera: Camera,
    pub level_id: &'static str,
}


impl Scene {
    pub fn new(
        maze: Maze,
        level_id: &'static str
    ) -> Scene {
        Scene{
            maze,
            camera: (0.0, 0.0),
            level_id,
        }
    }

    pub fn update(&mut self, elapsed: Duration) {
        const ACCELERATION_PER_MS: f32 = 0.997;
        let pos = self.maze.position();
        let ratio = ACCELERATION_PER_MS.powf(elapsed.as_millis() as f32);
        self.camera = linear_interpolation(pos, self.camera, ratio);
    }
}
