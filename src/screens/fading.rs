use std::time::Duration;
use crate::screens::*;
use sdl2::pixels::Color;


enum State {
    FadingIn(f32),
    Active,
    FadingOut(Transition, f32),
}

pub struct FadingScreen<S: Screen> {
    screen: S,
    state: State,
    fade_in_time: Duration,
    fade_out_time: Duration,
}

impl<S: Screen> FadingScreen<S> {
    pub fn new(screen: S, fade_in_time: Duration, fade_out_time: Duration) -> Self {
        Self {
            screen,
            state: State::FadingIn(0.),
            fade_in_time, fade_out_time,
        }
    }

    fn handle_transition(&mut self, transition: Transition) {
        match transition {
            Transition::Goto(_) | Transition::Exit => {
                let visibility = if let State::FadingIn(visibility) = self.state {
                    visibility
                } else { 1. };
                self.state = State::FadingOut(
                    transition,
                    visibility
                );
            },
            _ => {},
        };
    }
}

impl<S: Screen> Screen for FadingScreen<S> {
    fn initialize(&mut self, canvas: &mut Canvas, fonts: &Fonts) {
        self.screen.initialize(canvas, fonts)
    }

    fn handle_event(&mut self, event: &Event) -> Transition {
        match self.state {
            State::FadingIn(_) | State::Active => {
                let transition = self.screen.handle_event(event);
                if let Transition::GotoNow(screen) = transition {
                    return Transition::Goto(screen);
                }
                self.handle_transition(transition);
            },
            State::FadingOut(_, _) => {},
        };
        Transition::Stay
    }

    fn update(&mut self, elapsed: Duration) -> Transition {
        match self.state {
            State::FadingIn(_) | State::Active => {
                let transition = self.screen.update(elapsed);
                if let Transition::GotoNow(screen) = transition {
                    return Transition::Goto(screen);
                }
                self.handle_transition(transition);
            },
            State::FadingOut(_, _) => {},
        };

        match &mut self.state {
            State::FadingIn(ref mut visibility) => {
                *visibility += elapsed.as_secs_f32() / self.fade_in_time.as_secs_f32();
                if *visibility > 1. {
                    self.state = State::Active;
                }
            },
            State::FadingOut(_, ref mut visibility) => {
                *visibility -= elapsed.as_secs_f32() / self.fade_out_time.as_secs_f32();
                if *visibility < 0. {
                    // Move that screen of the `State`
                    if let State::FadingOut(transition, _)
                        = std::mem::replace(&mut self.state, State::Active)
                    {
                        return transition;
                    } else {
                        unreachable!();
                    }
                }
            },
            State::Active => {},
        };
        Transition::Stay
    }

    fn render(&self, canvas: &mut Canvas) {
        self.screen.render(canvas);

        let opacity = 1. - match self.state {
            State::FadingIn(visibility) => visibility,
            State::Active => 1.,
            State::FadingOut(_, visibility) => visibility,
        };
        let color = Color::RGBA(0, 0, 0, (opacity * 255.) as u8);
        canvas.set_draw_color(color);
        canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
        canvas.fill_rect(canvas.viewport()).unwrap();
    }
}
