use sdl2::video::WindowContext;
use sdl2::render::{Texture, TextureCreator};


pub type Canvas = sdl2::render::Canvas<sdl2::video::Window>;


#[derive(Default)]
pub struct Textures<'t> {
    pub light: Option<Texture<'t>>,
}

pub struct Target<'t> {
    pub canvas: Canvas,
    pub textures: Textures<'t>,
    pub texture_creator: &'t TextureCreator<WindowContext>,
}

impl<'t> Target<'t> {
    pub fn new(canvas: Canvas, texture_creator: &'t TextureCreator<WindowContext>) -> Self {
        Target {
            canvas,
            textures: Textures::default(),
            texture_creator,
        }
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }
}
