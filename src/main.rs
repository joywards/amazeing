#![cfg_attr(feature = "bench", feature(test))]

extern crate sdl2;
extern crate itertools;
extern crate itertools_num;
extern crate dirs;
extern crate serde;
extern crate ron;
#[macro_use]
extern crate lazy_static;

mod utils;
mod geometry;
mod geometry_sets;
mod layer;
mod maze;
mod generation;
mod build;
mod visible_area;
mod traversal;
mod levels;
mod scene;
mod screens;
mod observers;
mod fonts;

use std::time::{Duration, SystemTime};

use screens::menu::MenuScreen;
use screens::ScreenManager;
use fonts::Fonts;

pub const WINDOW_WIDTH: u32 = 1400;
pub const WINDOW_HEIGHT: u32 = 900;


// We don't really want to mess with lifetimes. Just make it 'static.
lazy_static! {
    static ref TTF: sdl2::ttf::Sdl2TtfContext = {
        sdl2::ttf::init().unwrap()
    };
}

fn main() {
    let sdl_context: sdl2::Sdl = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Amazeing", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let canvas = window.into_canvas().build().unwrap();
    let fonts = Fonts::new(&TTF);

    let mut manager = ScreenManager::new(
        MenuScreen::create_initial(),
        canvas,
        fonts,
    );

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

        manager.render();

        ::std::thread::sleep(Duration::from_micros(1_000_000u64 / 60));
    }
}
