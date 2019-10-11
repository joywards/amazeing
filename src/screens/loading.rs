use std::sync::mpsc::{channel, Receiver, TryRecvError};

use crate::screens::*;
use crate::screens::scene::SceneScreen;
use crate::screens::menu::MenuScreen;
use crate::maze::Maze;
use crate::levels::LevelGenerator;

use sdl2::render::Texture;
use sdl2::pixels::Color;

pub struct LoadingScreen {
    receiver: Receiver<Maze>,
    level_id: &'static str,
    stage: u32,

    text_texture: Option<Texture>,
}

impl LoadingScreen {
    pub fn new(generator: &'static dyn LevelGenerator, stage: u32) -> Self {
        let level_id = generator.id();

        let (sender, receiver) = channel();
        std::thread::spawn(move|| {
            sender.send(generator.generate(stage)).unwrap();
        });

        Self {
            receiver,
            level_id,
            stage,
            text_texture: None,
        }
    }

    fn render_text(canvas: &mut Canvas, texture: &Texture) {
        let query = texture.query();
        canvas.copy(
            &texture,
            None,
            sdl2::rect::Rect::from_center(
                canvas.viewport().center(),
                query.width,
                query.height
            )
        ).unwrap();
    }
}

impl Screen for LoadingScreen {
    fn update(&mut self, _elapsed: Duration) -> Transition {
        match self.receiver.try_recv() {
            Ok(maze) => {
                Transition::Goto(Box::new(SceneScreen::from_maze(
                    maze, self.level_id, self.stage
                )))
            },
            Err(TryRecvError::Empty) => Transition::Stay,
            Err(TryRecvError::Disconnected) => Transition::Goto(
                Box::new(MenuScreen::new())
            ),
        }
    }

    fn initialize(&mut self, canvas: &mut Canvas, fonts: &Fonts) {
        let text_color = Color::RGB(192, 192, 192);
        let text_surface = fonts.default.render("Generating...")
            .blended_wrapped(text_color, 700).unwrap();
        let text_texture = canvas.texture_creator()
            .create_texture_from_surface(&text_surface).unwrap();
        self.text_texture = Some(text_texture);
    }

    fn render(&self, canvas: &mut Canvas) {
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear();

        Self::render_text(canvas, &self.text_texture.as_ref().unwrap());
        // TODO: fade in and fade out effects
        // TODO: draw a spinner
    }
}
