use crate::geometry::Dir;
use crate::maze::{Maze, MoveResult};
use crate::scene;
use crate::screens::*;
use crate::screens::menu::MenuScreen;

pub struct SceneScreen {
    scene: scene::Scene,
    renderer: scene::Renderer,
}

impl SceneScreen {
    pub fn from_maze(maze: Maze, level_id: &'static str) -> Self {
        Self {
            scene: scene::Scene::new(maze, level_id),
            renderer: scene::Renderer::new(),
        }
    }
}

enum Action {
    Exit,
    Move(Dir),
    Nothing,
}

impl Screen for SceneScreen {
    fn handle_event(&mut self, event: &sdl2::event::Event) -> Transition {
        let action = match event {
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                Action::Exit
            },
            Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                Action::Move(Dir::DOWN)
            },
            Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                Action::Move(Dir::RIGHT)
            },
            Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                Action::Move(Dir::UP)
            },
            Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                Action::Move(Dir::LEFT)
            },
            _ => Action::Nothing
        };

        match action {
            Action::Exit => Transition::Goto(Box::new(MenuScreen::new())),

            Action::Move(dir) => {
                if self.scene.maze.try_move(dir) == MoveResult::FINISH {
                    Transition::Goto(Box::new(MenuScreen::new()))
                } else {
                    Transition::Stay
                }
            },
            Action::Nothing => Transition::Stay
        }
    }

    fn update(&mut self, elapsed: Duration) -> Transition {
        self.scene.update(elapsed);
        Transition::Stay
    }

    fn render(&self, target: &mut Target) {
        self.renderer.render(&self.scene, target);
    }
}
