extern crate sdl2;
extern crate itertools;

mod dsu;
mod layer;
mod geometry;
mod generation;
mod maze;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render;
use sdl2::rect::Rect;
use std::time::Duration;

use layer::Layer;
use generation::generate;
use geometry::coord::Coord;
use geometry::direction::Dir;
use rand::SeedableRng;
use rand::rngs::SmallRng;
use itertools::Itertools;

type Canvas = render::Canvas<sdl2::video::Window>;

const CELL_SIZE: u32 = 17;
const WINDOW_WIDTH: u32 = 1400;
const WINDOW_HEIGHT: u32 = 900;
const SIZE: i32 = 25;

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

fn render_center_square(canvas: &mut Canvas) {
    canvas.set_draw_color(Color::RGB(128, 128, 128));
    let view_coord = to_view((0, 0).into());
    fill_rect(canvas, view_coord.x, view_coord.y, CELL_SIZE - 1, CELL_SIZE - 1);
}

fn generate_layer(seed: u64) -> Layer {
    let mut layer = Layer::default();
    for x in -SIZE..=SIZE {
        for y in -SIZE..=SIZE {
            if x.pow(2) + y.pow(2) < SIZE.pow(2) {
                layer.add((x, y));
            }
        }
    }
    let mut rng = SmallRng::seed_from_u64(seed);
    generate(&mut layer, Coord::new(0, 0), &mut rng);
    layer
}

fn main() {
    let layer = generate_layer(0);

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
                _ => {}
            }
        }

        render_layer(&mut canvas, &layer);
        render_center_square(&mut canvas);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
