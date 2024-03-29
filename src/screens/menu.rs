use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use crate::screens::*;
use crate::screens::loading::LoadingScreen;
use crate::screens::fading::FadingScreen;
use crate::levels::*;
use crate::levels;
use crate::geometry::Dir;
use crate::utils::persistent_state::get_persistent_state;

pub struct MenuScreen {
    levels: Vec<(&'static dyn LevelGenerator, u32)>,
    cursor: (u32, u32),
    recommended_level: (u32, u32),
}

impl MenuScreen {
    fn new() -> Self {
        let levels = Self::get_unlocked_levels();
        let recommended_level = Self::find_recommended_level(&levels);
        Self {
            levels,
            cursor: recommended_level,
            recommended_level
        }
    }

    pub fn create() -> Box<dyn Screen> {
        Self::new().with_effects()
    }

    pub fn create_and_autostart() -> Box<dyn Screen> {
        let menu_screen = Self::new();
        menu_screen.start_level(menu_screen.recommended_level)
    }

    pub fn create_initial() -> Box<dyn Screen> {
        let menu_screen = Self::new();
        if menu_screen.recommended_level == (0, 0) {
            menu_screen.start_level(menu_screen.recommended_level)
        } else {
            menu_screen.with_effects()
        }
    }

    fn with_effects(self) -> Box<dyn Screen> {
        Box::new(FadingScreen::new(
            self,
            Duration::from_millis(100),
            Duration::from_millis(100)
        ))
    }

    fn get_unlocked_levels() -> Vec<(&'static dyn LevelGenerator, u32)> {
        let persistent_state = get_persistent_state().lock().unwrap();
        let mut found_uncompleted_level = false;
        levels::GENERATORS.iter().map(|&generator| {
            let completed_stages = persistent_state
                .progress.completed_stages(generator.id());
            (generator, completed_stages)
        }).take_while(|(_generator, completed_stages)| {
            if found_uncompleted_level {
                false
            } else {
                if *completed_stages == 0 {
                    found_uncompleted_level = true;
                }
                true
            }
        }).collect()
    }

    fn find_recommended_level(
        levels: &[(&'static dyn LevelGenerator, u32)]
    ) -> (u32, u32) {
        for (i, &(generator, completed_stages)) in levels.iter().enumerate() {
            if completed_stages < generator.recommended_length()
                || i + 1 == levels.len()
            {
                return (i as u32, completed_stages);
            }
        }
        unreachable!();
    }

    fn start_level(&self, level: (u32, u32)) -> Box<dyn Screen> {
        let (generator, completed_stages) = self.levels[level.0 as usize];
        let stage = level.1;
        assert!(stage <= completed_stages);
        let autocontinue = self.recommended_level <= level;
        Box::new(LoadingScreen::new(generator, stage, autocontinue))
    }
}

enum Action {
    Exit,
    StartLevel,
    MoveCursor(Dir),
    Nothing,
}

impl Screen for MenuScreen {
    fn handle_event(&mut self, event: &sdl2::event::Event) -> Transition {
        let action = match event {
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => Action::Exit,
            Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                Action::MoveCursor(Dir::DOWN)
            },
            Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                Action::MoveCursor(Dir::RIGHT)
            },
            Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                Action::MoveCursor(Dir::UP)
            },
            Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                Action::MoveCursor(Dir::LEFT)
            },
            Event::KeyDown { keycode: Some(Keycode::Return), .. } => {
                Action::StartLevel
            },
            _ => Action::Nothing
        };

        match action {
            Action::Exit => Transition::Exit,
            Action::Nothing => Transition::Stay,
            Action::StartLevel => Transition::Goto(self.start_level(self.cursor)),
            Action::MoveCursor(dir) => {
                match dir {
                    Dir::UP => {
                        let i = self.cursor.0;
                        if i > 0 {
                            self.cursor.0 = i - 1;
                            self.cursor.1 = self.levels[i as usize - 1].1;
                        }
                    },
                    Dir::DOWN => {
                        let i = self.cursor.0;
                        if i + 1 < self.levels.len() as u32 {
                            self.cursor.0 = i + 1;
                            self.cursor.1 = self.levels[i as usize + 1].1;
                        }
                    },
                    Dir::LEFT => {
                        if self.cursor.1 > 0 {
                            self.cursor.1 -= 1;
                        }
                    },
                    Dir::RIGHT => {
                        let i = self.cursor.0 as usize;
                        if self.cursor.1 < self.levels[i].1 {
                            self.cursor.1 += 1;
                        }
                    },
                }
                Transition::Stay
            },
        }
    }

    fn render(&self, canvas: &mut Canvas) {
        canvas.set_draw_color(Color::RGB(32, 32, 32));
        canvas.clear();

        render_button_in_grid(canvas, self.cursor.0, self.cursor.1, Color::RGB(192, 192, 192), true);
        for (i, &(_, completed)) in self.levels.iter().enumerate() {
            for j in 0..completed {
                render_button_in_grid(canvas, i as u32, j, Color::RGB(0, 128, 0), false);
            }
            render_button_in_grid(canvas, i as u32, completed, Color::RGB(64, 64, 64), false);
        }
    }
}

fn render_button_in_grid(canvas: &mut Canvas, i: u32, j: u32, color: Color, enlarged: bool) {
    const SIZE: u32 = 20;
    const GAP: u32 = 6;
    const MARGIN: u32 = 52;

    let x = MARGIN + (SIZE + GAP) * j;
    let y = MARGIN + (SIZE + GAP) * i;
    let rect = if enlarged {
        Rect::new(
            x as i32 - 1, y as i32 - 1, SIZE + 2, SIZE + 2
        )
    } else {
        Rect::new(
            x as i32, y as i32, SIZE, SIZE
        )
    };
    canvas.set_draw_color(color);
    canvas.fill_rect(Some(rect)).unwrap();
}
