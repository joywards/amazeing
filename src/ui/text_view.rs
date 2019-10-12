use std::time::Duration;
use std::cell::RefCell;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas as Canvas;
use sdl2::render::Texture;
use sdl2::pixels::Color;
use crate::fonts::Font;


enum TextViewState {
    Hidden,
    Shown,
    // phase, interval, from_opacity, to_opacity
    Pulsating(f32, Duration, u8, u8),
}

pub struct TextView {
    state: TextViewState,
    texture: RefCell<Texture>,
    size: (u32, u32),
    dst_rect: Rect,
}

impl TextView {
    pub fn new(
        canvas: &mut Canvas,
        text: &str,
        font: &Font,
        color: Color,
        wrap_width: u32,
    ) -> Self {
        let surface = font.render(text)
            .blended_wrapped(color, wrap_width).unwrap();
        let size = surface.size();
        let texture = canvas.texture_creator()
            .create_texture_from_surface(&surface).unwrap();
        Self {
            state: TextViewState::Hidden,
            texture: RefCell::new(texture),
            size,
            dst_rect: canvas.viewport(),
        }
    }

    pub fn set_dst_rect(&mut self, dst_rect: Rect) {
        self.dst_rect = dst_rect;
    }

    pub fn width(&self) -> u32 { self.size.0 }
    pub fn height(&self) -> u32 { self.size.1 }

    pub fn show(&mut self) {
        self.state = TextViewState::Shown;
    }

    pub fn show_pulsating(&mut self, interval: Duration, from: u8, to: u8) {
        assert!(from <= to);
        self.state = TextViewState::Pulsating(0., interval, from, to);
    }

    pub fn update(&mut self, elapsed: Duration) {
        if let TextViewState::Pulsating(ref mut phase, interval, _, _) = self.state {
            *phase += elapsed.as_secs_f32() / interval.as_secs_f32();
            *phase = phase.rem_euclid(1.);
        };
    }

    pub fn render(&self, canvas: &mut Canvas) {
        use std::f32::consts::PI;
        match self.state {
            TextViewState::Hidden => {},
            TextViewState::Shown => {
                canvas.copy(&self.texture.borrow(), None, self.dst_rect).unwrap();
            },
            TextViewState::Pulsating(phase, _, from_opacity, to_opacity) => {
                let visibility = (1. - (phase * PI * 2.).cos()) / 2.;
                assert!(0. <= visibility && visibility <= 1.);
                let int_opacity = from_opacity + (
                    f32::from(to_opacity - from_opacity) * visibility
                ) as u8;
                let mut texture = self.texture.borrow_mut();
                texture.set_alpha_mod(int_opacity);
                canvas.copy(&texture, None, self.dst_rect).unwrap();
            },
        }
    }
}
