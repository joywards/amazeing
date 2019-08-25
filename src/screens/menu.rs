use crate::screens::*;
use crate::screens::scene::SceneScreen;
use crate::levels::*;
use sdl2::keyboard::Keycode;

pub struct MenuScreen {
}

impl MenuScreen {
    pub fn new() -> Self {
        Self {}
    }
}

impl Screen for MenuScreen {
    fn handle_event(&mut self, event: &sdl2::event::Event) -> Transition {
        // TODO: generate levels asynchroniously.
        match event {
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                Transition::Exit
            },
            Event::KeyDown { keycode: Some(Keycode::Num1), .. } => {
                Transition::Goto(Box::new(SceneScreen::from_maze(
                    Plain::generate(0).unwrap()
                )))
            },
            Event::KeyDown { keycode: Some(Keycode::Num2), .. } => {
                Transition::Goto(Box::new(SceneScreen::from_maze(
                    Debug::generate(0).unwrap()
                )))
            },
            _ => Transition::Stay
        }
    }

    fn render(&self, target: &mut Target) {
        let canvas = &mut target.canvas;
        canvas.set_draw_color(sdl2::pixels::Color::RGB(32, 32, 32));
        canvas.clear();
    }
}
