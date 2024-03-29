use crate::screens::*;

pub struct ScreenManager {
    current_screen: Box<dyn Screen>,
    canvas: Canvas,
    fonts: Fonts,
    is_running: bool,
}

impl ScreenManager {
    pub fn new(screen: Box<dyn Screen>, canvas: Canvas, fonts: Fonts) -> Self {
        let mut result = Self {
            current_screen: screen,
            canvas,
            fonts,
            is_running: true,
        };
        result.current_screen.initialize(&mut result.canvas, &result.fonts);
        result
    }

    pub fn keep_running(&self) -> bool {
        self.is_running
    }

    fn respond(&mut self, transition: Transition) {
        match transition {
            Transition::Stay => {},
            Transition::Exit => {
                self.is_running = false;
            },
            Transition::Goto(screen) | Transition::GotoNow(screen) => {
                self.current_screen = screen;
                self.current_screen.initialize(&mut self.canvas, &self.fonts);
            }
        }
    }

    pub fn handle_event(&mut self, event: &Event) -> Transition {
        if let Event::Quit {..} = event {
            self.is_running = false;
            Transition::Exit
        } else {
            let transition = self.current_screen.handle_event(event);
            self.respond(transition);
            Transition::Stay
        }
    }

    pub fn update(&mut self, elapsed: Duration) -> Transition {
        let transition = self.current_screen.update(elapsed);
        self.respond(transition);
        Transition::Stay
    }

    pub fn render(&mut self) {
        self.current_screen.render(&mut self.canvas);
        self.canvas.present();
    }
}
