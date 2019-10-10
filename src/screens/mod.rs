pub mod scene;
pub mod menu;
pub mod loading;
mod manager;

use std::time::Duration;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas as Canvas;

pub use manager::ScreenManager;


pub enum Transition {
    Stay,
    Goto(Box<dyn Screen>),
    Exit,
}


pub trait Screen {
    // We want screen objects to be easy to construct passing data from other
    // screens. That's why we provide canvas only after screen construction.
    // `initialize` is guaranteed to be called before the first call to `render`.
    fn initialize(&mut self, _canvas: &mut Canvas) {}
    fn handle_event(&mut self, _event: &Event) -> Transition {
        Transition::Stay
    }
    fn update(&mut self, _elapsed: Duration) -> Transition {
        Transition::Stay
    }
    fn render(&self, _canvas: &mut Canvas) {}
}
