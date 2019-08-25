#![cfg_attr(feature = "bench", feature(test))]

extern crate sdl2;
extern crate itertools;
#[macro_use]
extern crate lazy_static;

mod utils;
mod geometry;
mod layer;
mod maze;
mod generation;
mod build;
mod visible_area;
mod traversal;
mod levels;
mod scene;
mod render;
mod screens;

use std::time::{Duration, SystemTime};

use render::Target;
use screens::menu::MenuScreen;
use screens::{Screen, ScreenManager};

pub const WINDOW_WIDTH: u32 = 1400;
pub const WINDOW_HEIGHT: u32 = 900;


fn main() {
    let sdl_context: sdl2::Sdl = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Amazeing", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut render_target = Target::new(canvas, &texture_creator);

    let mut manager = ScreenManager::new(Box::new(
        MenuScreen::new()
    ));

    let mut last_time = std::time::SystemTime::now();
    let mut event_pump = sdl_context.event_pump().unwrap();
    while manager.keep_running() {
        for event in event_pump.poll_iter() {
            manager.handle_event(&event);
        }

        let new_time = SystemTime::now();
        let elapsed = new_time.duration_since(last_time).unwrap();
        manager.update(elapsed);
        last_time = new_time;

        manager.render(&mut render_target);
        render_target.present();

        ::std::thread::sleep(Duration::from_micros(1_000_000u64 / 60));
    }
}
