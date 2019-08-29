use crate::screens::*;
use crate::screens::loading::LoadingScreen;
use crate::levels::*;
use sdl2::keyboard::Keycode;

pub struct MenuScreen {
}

impl MenuScreen {
    pub fn new() -> Self {
        Self {}
    }
}

enum Action {
    Exit,
    StartLevel(Box<dyn LevelGenerator>),
    Nothing,
}

impl Screen for MenuScreen {
    fn handle_event(&mut self, event: &sdl2::event::Event) -> Transition {
        let action = match event {
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => Action::Exit,
            Event::KeyDown { keycode: Some(Keycode::Num1), .. } => {
                Action::StartLevel(Box::new(Plain()))
            },
            Event::KeyDown { keycode: Some(Keycode::Num2), .. } => {
                Action::StartLevel(Box::new(Double()))
            },
            Event::KeyDown { keycode: Some(Keycode::Num9), .. } => {
                Action::StartLevel(Box::new(Debug()))
            },
            _ => Action::Nothing
        };

        match action {
            Action::Exit => Transition::Exit,
            Action::Nothing => Transition::Stay,
            Action::StartLevel(generator) => {
                Transition::Goto(Box::new(LoadingScreen::new(generator)))
            },
        }
    }

    fn render(&self, target: &mut Target) {
        let canvas = &mut target.canvas;
        canvas.set_draw_color(sdl2::pixels::Color::RGB(32, 32, 32));
        canvas.clear();
    }
}
