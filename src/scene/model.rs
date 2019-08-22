use crate::maze::Maze;
use crate::traversal::Info;
use crate::scene::{Renderer, Canvas};
use crate::region::Region;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;


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

    pub fn update(&mut self) {
        let pos = self.maze.position();
        self.camera = (pos.0 as f32, pos.1 as f32);
    }

    pub fn render(&self, canvas: &mut Canvas, visible_area: &Region) {
        self.renderer.render(self, canvas, visible_area);
    }
}
