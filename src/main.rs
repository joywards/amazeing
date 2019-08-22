#![cfg_attr(feature = "bench", feature(test))]

extern crate sdl2;
extern crate itertools;
#[macro_use]
extern crate lazy_static;

mod dsu;
mod layer;
mod geometry;
mod generation;
mod maze;
mod scene;
mod region;
mod build;
mod traversal;
mod visible_area;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

use geometry::Dir;
use build::{make_circle, MazeBuilder, GenerationError};
use maze::Maze;
use traversal::Info;
use scene::Scene;

const SIZE: i32 = 17;
pub const WINDOW_WIDTH: u32 = 1400;
pub const WINDOW_HEIGHT: u32 = 900;


fn try_build(builder: &mut MazeBuilder) -> Result<(), GenerationError> {
    let first = builder.generate_first_layer((0, 0));
    let (_, mut last, _) = builder.fork_to_three_layers(first)?;
    for _ in 0..9 {
        last = builder.add_layer_from_deepest_point(last)?;
    }
    Ok(())
}

fn build_maze(seed: u64) -> (Maze, Vec<Info>) {
    let shape: Vec<_> = make_circle(SIZE).collect();

    let mut builder = MazeBuilder::new(seed, shape);
    loop {
        if try_build(&mut builder).is_ok() {
            break;
        } else {
            println!("Generation error");
        }
    }
    builder.into_maze_and_layer_info()
}

fn main() {
    let (maze, layer_info) = build_maze(0);

    let sdl_context: sdl2::Sdl = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Amazeing", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let mut scene = Scene::new(maze, layer_info, &texture_creator);

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

        scene.update();

        scene.render(&mut canvas);
        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
