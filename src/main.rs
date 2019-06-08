extern crate sdl2;
extern crate itertools;

mod dsu;
mod layer;
mod geometry;
mod generation;
mod maze;
mod region;
mod build;

use std::collections::HashSet;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render;
use sdl2::rect::Rect;
use std::time::Duration;

use layer::Layer;
use geometry::coord::Coord;
use geometry::direction::Dir;
use itertools::Itertools;

use region::Region;
use build::{make_circle, generate_with_copied_region, generate_layer};
use maze::Maze;

use rand::rngs::SmallRng;
use rand::SeedableRng;

type Canvas = render::Canvas<sdl2::video::Window>;

const CELL_SIZE: u32 = 17;
const WINDOW_WIDTH: u32 = 1400;
const WINDOW_HEIGHT: u32 = 900;
const SIZE: i32 = 20;

fn to_view(scene_coord: Coord) -> Coord {
    let scene_camera = Coord::new(0, 0);
    let view_camera = Coord::new(WINDOW_WIDTH as i32 / 2, WINDOW_HEIGHT as i32 / 2);
    let x = (scene_coord.x - scene_camera.x) * CELL_SIZE as i32 + view_camera.x;
    let y = (scene_coord.y - scene_camera.y) * CELL_SIZE as i32 + view_camera.y;
    (x, y).into()
}

fn fill_rect<X, Y, W, H>(
    canvas: &mut Canvas,
    x: X, y: Y, w: W, h: H
) where
    X: Into<i32>, Y: Into<i32>,
    W: Into<u32>, H: Into<u32>,
{
    canvas.fill_rect(Some(Rect::new(
        x.into(), y.into(),
        w.into(), h.into()
    ))).unwrap();
}

fn render_layer(canvas: &mut Canvas, layer: &Layer) {
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    for (x, y) in (-SIZE..=SIZE).cartesian_product(-SIZE..=SIZE) {
        let coord = Coord::new(x, y);
        if layer.has(coord) {
            let view_coord = to_view(coord);

            fill_rect(canvas, view_coord.x, view_coord.y, CELL_SIZE - 1, CELL_SIZE - 1);
            if layer.passable(coord, Dir::DOWN) {
                fill_rect(canvas, view_coord.x, view_coord.y, CELL_SIZE - 1, CELL_SIZE);
            }
            if layer.passable(coord, Dir::RIGHT) {
                fill_rect(canvas, view_coord.x, view_coord.y, CELL_SIZE, CELL_SIZE - 1);
            }
        }
    }
}

fn render_square(canvas: &mut Canvas, coord: Coord) {
    canvas.set_draw_color(Color::RGBA(192, 192, 192, 32));
    let view_coord = to_view(coord);
    fill_rect(canvas, view_coord.x, view_coord.y, CELL_SIZE - 1, CELL_SIZE - 1);
}

fn main() {
    let shape: Vec<_> = make_circle(SIZE).collect();
    let visible_area: Region = make_circle(12).collect::<HashSet<_>>().into();
    let cloned_area = visible_area.shifted_by(Coord::new(5, 5));

    let mut rng = SmallRng::seed_from_u64(2);
    let first = generate_layer(&shape, (0, 0).into(), &mut rng);
    let second = generate_with_copied_region(
        shape.iter().cloned(),
        &first,
        &cloned_area,
        &mut rng
    );
    let layers = [&first, &second];
    let mut level = 0;
    let maze = Maze::new(first.clone(), (0, 0).into());

    let sdl_context: sdl2::Sdl = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Amazeing", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    level = 0;
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    level = 1;
                },
                _ => {}
            }
        }

        render_layer(&mut canvas, layers[level]);
        render_square(&mut canvas, (0, 0).into());
        for &cell in cloned_area.boundary() {
            if first.has(cell) {
                render_square(&mut canvas, cell);
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
