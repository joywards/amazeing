pub mod scene;
mod manager;

use std::time::Duration;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use crate::render::Target;

pub use manager::ScreenManager;


pub enum Transition {
    Stay,
    Goto(Box<dyn Screen>),
    Exit,
}


pub trait Screen {
    fn on_enter(&mut self) {}
    fn on_leave(&mut self) {}

    fn handle_event(&mut self, _event: &Event) -> Transition {
        Transition::Stay
    }
    fn update(&mut self, _elapsed: Duration) -> Transition {
        Transition::Stay
    }
    fn render(&self, _target: &mut Target) {}
}
