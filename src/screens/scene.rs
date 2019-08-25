use crate::geometry::Dir;
use crate::maze::Maze;
use crate::scene;
use crate::screens::*;
use crate::screens::menu::MenuScreen;

pub struct SceneScreen {
    scene: scene::Scene,
    renderer: scene::Renderer,
}

impl SceneScreen {
    pub fn from_maze(maze: Maze) -> Self {
        Self {
            scene: scene::Scene::new(maze),
            renderer: scene::Renderer::new(),
        }
    }
}

impl Screen for SceneScreen {
    fn handle_event(&mut self, event: &sdl2::event::Event) -> Transition {
        match event {
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                return Transition::Goto(Box::new(MenuScreen::new()));
            },
            Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                self.scene.maze.try_move(Dir::DOWN);
            },
            Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                self.scene.maze.try_move(Dir::RIGHT);
            },
            Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                self.scene.maze.try_move(Dir::UP);
            },
            Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                self.scene.maze.try_move(Dir::LEFT);
            },
            _ => {}
        }
        Transition::Stay
    }

    fn update(&mut self, elapsed: Duration) -> Transition {
        self.scene.update(elapsed);
        Transition::Stay
    }

    fn render(&self, target: &mut Target) {
        self.renderer.render(&self.scene, target);
    }
}
