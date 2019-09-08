use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use crate::screens::*;
use crate::screens::loading::LoadingScreen;
use crate::levels::*;
use crate::levels;
use crate::geometry::Dir;
use crate::utils::persistent_state::get_persistent_state;
use crate::render::Canvas;

pub struct MenuScreen {
    levels: Vec<(&'static dyn LevelGenerator, u32)>,
    cursor: (u32, u32),
}

impl MenuScreen {
    pub fn new() -> Self {
        let persistent_state = get_persistent_state().lock().unwrap();
        let mut found_uncompleted_level = false;
        let levels: Vec<_> = levels::GENERATORS.iter().map(|&generator| {
            let completed_stages = persistent_state
                .progress.completed_stages(generator.id()) as u32;
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
        }).collect();

        let cursor = (levels.len() as u32 - 1, levels.iter().last().unwrap().1);
        Self {
            levels, cursor
        }
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
            Action::StartLevel => {
                let (generator, completed) = self.levels[self.cursor.0 as usize];
                let stage = self.cursor.1;
                assert!(stage <= completed);
                Transition::Goto(Box::new(LoadingScreen::new(generator, stage)))
            },
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

    fn render(&self, target: &mut Target) {
        let canvas = &mut target.canvas;

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
