use crate::geometry::Dir;
use crate::maze::{Maze, MoveResult};
use crate::scene;
use crate::screens::{
    *,
    menu::MenuScreen,
    fading::FadingScreen,
};
use crate::observers::{level_completion_observer, LevelCompleted};
use crate::cli::Args;

pub struct SceneScreen {
    scene: scene::Scene,
    renderer: scene::Renderer,
    args: Args,
}

impl SceneScreen {
    pub fn from_maze(maze: Maze, level_id: &'static str, stage: u32, args: Args) -> FadingScreen<Self> {
        FadingScreen::new(
            Self {
                scene: scene::Scene::new(maze, level_id, stage),
                renderer: scene::Renderer::new(),
                args,
            },
            Duration::from_millis(0), // Maze is initially shadowed anyways
            Duration::from_millis(700),
        )
    }

    fn notify_about_level_completion(&self) {
        level_completion_observer().lock().unwrap()
            .notify(LevelCompleted{
                level: self.scene.level_id,
                stage: self.scene.stage,
            });
    }
}

enum Action {
    Exit,
    Move(Dir),
    MoveBackwards,
    UseHint,
    Nothing,
}

impl Screen for SceneScreen {
    fn handle_event(&mut self, event: &sdl2::event::Event) -> Transition {
        let action = match event {
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                Action::Exit
            },
            Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                Action::MoveBackwards
            },
            Event::KeyDown { keycode: Some(Keycode::Backquote), .. } => {
                Action::UseHint
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

        let move_result = match action {
            Action::Exit => return Transition::GotoNow(Box::new(MenuScreen::new(self.args.clone()))),
            Action::Move(dir) => {
                self.scene.try_move(dir)
            },
            Action::MoveBackwards => {
                self.scene.try_move_towards_start()
            },
            Action::UseHint => {
                self.scene.use_hint();
                return Transition::Stay;
            },
            Action::Nothing => return Transition::Stay,
        };

        if move_result == MoveResult::Finish {
            self.notify_about_level_completion();
            Transition::Goto(Box::new(MenuScreen::new(self.args.clone())))
        } else {
            Transition::Stay
        }
    }

    fn update(&mut self, elapsed: Duration) -> Transition {
        self.scene.update(elapsed);
        Transition::Stay
    }

    fn initialize(&mut self, canvas: &mut Canvas, _fonts: &Fonts) {
        self.renderer.initialize(canvas);
    }

    fn render(&self, canvas: &mut Canvas) {
        self.renderer.render(&self.scene, canvas);
    }
}
