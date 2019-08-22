use itertools::Itertools;

use sdl2::pixels::Color;
use sdl2::surface::Surface;
use sdl2::render::{Texture, TextureCreator};
use sdl2::rect::Rect;
use sdl2::video::WindowContext;

use crate::layer::Layer;
use crate::geometry::Dir;

use crate::region::Region;
use crate::scene::{Scene, Camera};

pub type Canvas = sdl2::render::Canvas<sdl2::video::Window>;

const CELL_SIZE: u32 = 17;
pub const VISIBILITY_RADIUS: i32 = 12;
pub const WINDOW_WIDTH: u32 = 1400;
pub const WINDOW_HEIGHT: u32 = 900;

pub struct Renderer<'c, 't> {
    canvas: &'c mut Canvas,
    light_texture: Texture<'t>,
}

impl<'c, 't> Renderer<'c, 't> {
    // TextureCreator can not be stored inside Renderer because it has to
    // outlive every created texture and borrow checker doesn't allow to store
    // such objects in a single structure.
    pub fn new(canvas: &'c mut Canvas, texture_creator: &'t TextureCreator<WindowContext>) -> Self {
        let light_radius = ((VISIBILITY_RADIUS as f32 - 1. / 2_f32.sqrt()) * CELL_SIZE as f32) as u32;
        let texture_size = 1024; // TODO: calculate accurately
        let light_surface = create_light_surface(light_radius, texture_size).unwrap();
        Renderer {
            canvas,
            light_texture: texture_creator.create_texture_from_surface(
                light_surface
            ).unwrap()
        }
    }

    pub fn render(&mut self, scene: &Scene, visible_area: &Region) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        render_layer(&mut self.canvas, scene.maze.current_layer(), scene.camera);
        for &cell in visible_area.shifted_by(scene.maze.position()).boundary() {
            if scene.maze.current_layer().has(cell) {
                render_square(
                    &mut self.canvas,
                    cell, Color::RGB(240, 240, 240), scene.camera
                );
            }
        }

        for (&coord, coord_info) in scene.layer_info[scene.maze.current_layer_index()].coords.iter() {
            if coord_info.escapable.is_some() {
                render_square(
                    &mut self.canvas,
                    coord, Color::RGB(192, 192, 192), scene.camera
                );
            }
        }

        for &coord in &scene.layer_info[scene.maze.current_layer_index()].leaf_escapables {
            render_square(
                &mut self.canvas,
                coord, Color::RGB(220, 192, 192), scene.camera
            );
        }

        render_square(
            &mut self.canvas,
            scene.maze.position(), Color::RGBA(0, 192, 0, 255), scene.camera
        );

        let mut light_center = to_view(scene.maze.position(), scene.camera);
        light_center.0 += CELL_SIZE as i32 / 2;
        light_center.1 += CELL_SIZE as i32 / 2;

        let query = self.light_texture.query();
        self.canvas.copy(
            &self.light_texture,
            None,
            Some(Rect::from_center(
                light_center,
                query.width, query.height
            ))
        ).unwrap();

        self.canvas.present();
    }
}


fn to_view(scene_coord: (i32, i32), scene_camera: Camera) -> (i32, i32) {
    let view_camera = (WINDOW_WIDTH as f32 / 2.0, WINDOW_HEIGHT as f32 / 2.0);
    let x = (scene_coord.0 as f32 - scene_camera.0) * CELL_SIZE as f32 + view_camera.0;
    let y = (scene_coord.1 as f32 - scene_camera.1) * CELL_SIZE as f32 + view_camera.1;
    (x as i32, y as i32)
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

fn render_layer(canvas: &mut Canvas, layer: &Layer, camera: Camera) {
    const RENDER_SIZE: i32 = 20;
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    for (x, y) in (-RENDER_SIZE..=RENDER_SIZE).cartesian_product(-RENDER_SIZE..=RENDER_SIZE) {
        let coord = (x, y);
        if layer.has(coord) {
            let view_coord = to_view(coord, camera);

            fill_rect(canvas, view_coord.0, view_coord.1, CELL_SIZE - 1, CELL_SIZE - 1);
            if layer.passable(coord, Dir::DOWN) {
                fill_rect(canvas, view_coord.0, view_coord.1, CELL_SIZE - 1, CELL_SIZE);
            }
            if layer.passable(coord, Dir::RIGHT) {
                fill_rect(canvas, view_coord.0, view_coord.1, CELL_SIZE, CELL_SIZE - 1);
            }
        }
    }
}

fn render_square(canvas: &mut Canvas, coord: (i32, i32), color: Color, camera: Camera) {
    canvas.set_draw_color(color);
    let view_coord = to_view(coord, camera);
    fill_rect(canvas, view_coord.0, view_coord.1, CELL_SIZE - 1, CELL_SIZE - 1);
}

fn create_light_surface(radius: u32, size: u32) -> Result<Surface<'static>, String> {
    let center = size / 2;
    let surface = Surface::new(size, size, sdl2::pixels::PixelFormatEnum::RGBA32)?;
    let mut canvas = surface.into_canvas()?;
    for i in 0..size {
        for j in 0..size {
            let x = i as i32 - center as i32;
            let y = j as i32 - center as i32;
            if x.pow(2) + y.pow(2) < (radius as i32).pow(2) {
                canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
            } else {
                canvas.set_draw_color(Color::RGBA(0, 0, 0, 192));
            }
            canvas.draw_point((i as i32, j as i32)).unwrap();
        }
    }
    Ok(canvas.into_surface())
}

