use std::sync::mpsc::{channel, Receiver, TryRecvError};

use crate::screens::*;
use crate::screens::{scene::SceneScreen, menu::MenuScreen};
use crate::maze::Maze;
use crate::build::GenerationError;
use crate::levels::LevelGenerator;
use crate::utils::persistent_state::get_persistent_state;
use crate::observers::{level_completion_observer, LevelCompleted};

pub struct LoadingScreen {
    receiver: Receiver<Result<Maze, GenerationError>>,
    level_id: &'static str,
    stage: u64,
}

impl LoadingScreen {
    pub fn new(generator: Box<dyn LevelGenerator>) -> Self {
        let stage = get_persistent_state().lock().unwrap()
            .progress.completed_stages(generator.id());
        let level_id = generator.id();

        let (sender, receiver) = channel();
        std::thread::spawn(move|| {
            sender.send(generator.generate(stage)).unwrap();
        });

        Self { receiver, level_id, stage }
    }
}

impl Screen for LoadingScreen {
    fn update(&mut self, _elapsed: Duration) -> Transition {
        match self.receiver.try_recv() {
            Ok(Ok(maze)) => {
                Transition::Goto(Box::new(SceneScreen::from_maze(
                    maze, self.level_id
                )))
            },
            Ok(Err(_)) => {
                eprintln!(
                    "Could not generate level \"{}\" on stage {}.",
                    self.level_id, self.stage
                );

                // Skip this stage in further generation attempts
                level_completion_observer().lock().unwrap()
                    .notify(LevelCompleted {
                        level: self.level_id
                    });

                Transition::Goto(Box::new(MenuScreen::new()))
            }
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
