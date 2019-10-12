use std::sync::mpsc::{channel, Receiver, TryRecvError};

use crate::screens::{
    *,
    scene::SceneScreen,
    menu::MenuScreen,
    fading::FadingScreen,
};
use crate::ui::text_view::TextView;
use crate::maze::Maze;
use crate::levels::LevelGenerator;

use sdl2::pixels::Color;
use sdl2::rect::Rect;


enum State {
    Waiting,
    WaitingForGeneration,
    WaitingForKeyPress(Box<dyn Screen>),
}

pub struct LoadingScreen {
    state: State,
    receiver: Receiver<Maze>,
    level_id: &'static str,
    stage: u32,
    autocontinue: bool,
    main_text: String,

    main_text_view: Option<TextView>,
    press_any_key_text: Option<TextView>,
}

impl LoadingScreen {
    pub fn new(
        generator: &'static dyn LevelGenerator,
        stage: u32,
        autocontinue: bool,
    ) -> FadingScreen<Self> {
        let (sender, receiver) = channel();
        std::thread::spawn(move|| {
            if sender.send(generator.generate(stage)).is_err() {
                /* The receiving end is disconnected. Drop the result. */
            };
        });

        let state;
        let text;
        if stage == 0 && autocontinue {
            state = State::Waiting;
            text = generator.intro_text();
        } else {
            state = State::WaitingForGeneration;
            text = "Generating...";
        };
        let level_id = generator.id();
        FadingScreen::new(
            Self {
                state,
                receiver,
                level_id,
                stage,
                autocontinue,
                main_text: text.to_string(),
                main_text_view: None,
                press_any_key_text: None,
            },
            Duration::from_millis(400),
            Duration::from_millis(400),
        )
    }
}

impl Screen for LoadingScreen {
    fn update(&mut self, elapsed: Duration) -> Transition {
        self.main_text_view.as_mut().unwrap().update(elapsed);
        self.press_any_key_text.as_mut().unwrap().update(elapsed);

        if let State::WaitingForKeyPress(_) = self.state {
            return Transition::Stay;
        }
        match self.receiver.try_recv() {
            Ok(maze) => {
                if let State::Waiting = self.state {
                    self.state = State::WaitingForKeyPress(
                        Box::new(SceneScreen::from_maze(
                            maze, self.level_id, self.stage, self.autocontinue
                        ))
                    );
                    self.press_any_key_text.as_mut().unwrap().show_pulsating(
                        Duration::from_millis(800),
                        128, 255
                    );
                    Transition::Stay
                } else {
                    Transition::Goto(Box::new(SceneScreen::from_maze(
                        maze, self.level_id, self.stage, self.autocontinue
                    )))
                }
            },
            Err(TryRecvError::Empty) => Transition::Stay,
            Err(TryRecvError::Disconnected) => Transition::GotoNow(
                MenuScreen::create()
            ),
        }
    }

    fn handle_event(&mut self, event: &Event) -> Transition {
        match event {
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } =>
                return Transition::GotoNow(MenuScreen::create()),
            Event::KeyDown {..} => {},
            _ => return Transition::Stay,
        };
        match self.state {
            State::Waiting => {
                self.state = State::WaitingForGeneration;
                Transition::Stay
            },
            State::WaitingForGeneration => Transition::Stay,
            State::WaitingForKeyPress(_) => {
                if let State::WaitingForKeyPress(new_screen)
                    = std::mem::replace(&mut self.state, State::Waiting)
                {
                    Transition::Goto(new_screen)
                } else { unreachable!() }
            }
        }
    }

    fn initialize(&mut self, canvas: &mut Canvas, fonts: &Fonts) {
        let center = canvas.viewport().center();

        let mut main_text_view = TextView::new(
            canvas,
            &self.main_text,
            &fonts.default,
            Color::RGB(192, 192, 192),
            975
        );
        let main_text_rect = Rect::from_center(
            center,
            main_text_view.width(),
            main_text_view.height()
        );
        main_text_view.set_dst_rect(main_text_rect);

        main_text_view.show();
        self.main_text_view = Some(main_text_view);


        let mut press_any_key_text = TextView::new(
            canvas,
            "Press any key to continue",
            &fonts.small,
            Color::RGB(128, 128, 128),
            700
        );
        let mut press_any_key_text_rect = Rect::from_center(
            center,
            press_any_key_text.width(),
            press_any_key_text.height()
        );
        press_any_key_text_rect.set_y(main_text_rect.bottom() + 18);
        press_any_key_text.set_dst_rect(press_any_key_text_rect);

        self.press_any_key_text = Some(press_any_key_text);
    }

    fn render(&self, canvas: &mut Canvas) {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        let main_text_view = self.main_text_view.as_ref().unwrap();
        main_text_view.render(canvas);

        let press_any_key_text = self.press_any_key_text.as_ref().unwrap();
        press_any_key_text.render(canvas);
    }
}
