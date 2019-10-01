use std::sync::mpsc::{channel, Receiver, TryRecvError};

use crate::screens::*;
use crate::screens::scene::SceneScreen;
use crate::maze::Maze;
use crate::levels::LevelGenerator;

pub struct LoadingScreen {
    receiver: Receiver<Maze>,
    level_id: &'static str,
    stage: u32,
}

impl LoadingScreen {
    pub fn new(generator: &'static dyn LevelGenerator, stage: u32) -> Self {
        let level_id = generator.id();

        let (sender, receiver) = channel();
        std::thread::spawn(move|| {
            sender.send(generator.generate(stage)).unwrap();
        });

        Self{receiver, level_id, stage}
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
            Err(TryRecvError::Disconnected) => Transition::Stay,
        }
    }

    fn render(&self, target: &mut Target) {
        let canvas = &mut target.canvas;
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear();

        // TODO: draw a spinner
    }
}
