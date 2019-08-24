use std::time::Duration;
use crate::geometry::Dir;
use crate::scene;
use crate::screens::*;

pub struct SceneScreen {
    scene: scene::Scene,
    renderer: scene::Renderer,
}

impl SceneScreen {
    pub fn new(scene: scene::Scene) -> Self {
        Self {
            scene,
            renderer: scene::Renderer::new(),
        }
    }
}

impl Screen for SceneScreen {
    fn handle_event(&mut self, event: &sdl2::event::Event) -> Transition {
        match event {
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                return Transition::Exit;
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
