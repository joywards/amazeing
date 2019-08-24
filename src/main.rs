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

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::{Duration, SystemTime};

use geometry::Dir;
use scene::Scene;
use levels::*;

pub const WINDOW_WIDTH: u32 = 1400;
pub const WINDOW_HEIGHT: u32 = 900;


fn main() {
    let maze = levels::Debug::generate(0).unwrap();

    let sdl_context: sdl2::Sdl = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Amazeing", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let mut scene = Scene::new(maze);
    let scene_renderer = scene::Renderer::new(&texture_creator);

    let mut last_time = std::time::SystemTime::now();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    scene.maze.try_move(Dir::DOWN);
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    scene.maze.try_move(Dir::RIGHT);
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    scene.maze.try_move(Dir::UP);
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    scene.maze.try_move(Dir::LEFT);
                },
                _ => {}
            }
        }

        let new_time = SystemTime::now();
        let elapsed = new_time.duration_since(last_time).unwrap();
        scene.update(elapsed);
        last_time = new_time;

        scene_renderer.render(&scene, &mut canvas);
        canvas.present();

        ::std::thread::sleep(Duration::from_micros(1_000_000u64 / 60));
    }
}
