use crate::screens::*;

pub struct ScreenManager {
    current_screen: Box<dyn Screen>,
    is_running: bool,
}

impl ScreenManager {
    pub fn new(screen: Box<dyn Screen>) -> Self {
        Self {
            current_screen: screen,
            is_running: true,
        }
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
            Transition::Goto(screen) => {
                self.current_screen.on_leave();
                self.current_screen = screen;
                self.current_screen.on_enter();
            }
        }
    }
}

impl Screen for ScreenManager {
    fn handle_event(&mut self, event: &Event) -> Transition {
        if let Event::Quit {..} = event {
            self.is_running = false;
            Transition::Exit
        } else {
            let transition = self.current_screen.handle_event(event);
            self.respond(transition);
            Transition::Stay
        }
    }

    fn update(&mut self, elapsed: Duration) -> Transition {
        let transition = self.current_screen.update(elapsed);
        self.respond(transition);
        Transition::Stay
    }

    fn render(&self, target: &mut Target) {
        self.current_screen.render(target);
    }
}
