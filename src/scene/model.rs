use std::time::Duration;

use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;

use crate::maze::Maze;
use crate::traversal::Info;
use crate::scene::{Renderer, Canvas};
use crate::tuple_arithmetic::linear_interpolation;


pub type Camera = (f32, f32);

pub struct Scene<'t> {
    pub maze: Maze,
    pub layer_info: Vec<Info>,
    pub camera: Camera,

    renderer: Renderer<'t>,
}


impl<'t> Scene<'t> {
    pub fn new(
        maze: Maze,
        layer_info: Vec<Info>,
        texture_creator: &'t TextureCreator<WindowContext>
    ) -> Scene {
        Scene{
            maze,
            layer_info,
            camera: (0.0, 0.0),
            renderer: Renderer::new(texture_creator),
        }
    }

    pub fn update(&mut self, elapsed: Duration) {
        const ACCELERATION_PER_MS: f32 = 0.997;
        let pos = self.maze.position();
        let ratio = ACCELERATION_PER_MS.powf(elapsed.as_millis() as f32);
        self.camera = linear_interpolation(pos, self.camera, ratio);
    }

    pub fn render(&self, canvas: &mut Canvas) {
        self.renderer.render(self, canvas);
    }
}
