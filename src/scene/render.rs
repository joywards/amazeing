use itertools::Itertools;
use std::cell::Cell;

use sdl2::pixels::Color;
use sdl2::surface::Surface;
use sdl2::render::{Texture, TextureCreator};
use sdl2::rect::Rect;
use sdl2::video::WindowContext;

use crate::layer::Layer;
use crate::geometry::Dir;

use crate::visible_area::{visibility_radius, visible_area};
use crate::scene::{Scene, Camera};

pub type Canvas = sdl2::render::Canvas<sdl2::video::Window>;

const CELL_SIZE: u32 = 17;
const DEBUG: bool = true;

pub struct Renderer<'t> {
    light_texture: Texture<'t>,
    window_size: Cell<(u32, u32)>,
}

impl<'t> Renderer<'t> {
    // TextureCreator can not be stored inside Renderer because it has to
    // outlive every created texture and borrow checker doesn't allow to store
    // such objects in a single structure.
    pub fn new(texture_creator: &'t TextureCreator<WindowContext>) -> Self {
        let light_radius = ((visibility_radius() as f32 - 1. / 2_f32.sqrt()) * CELL_SIZE as f32) as u32;
        let texture_size = visibility_radius() as u32 * 2 * CELL_SIZE;
        let light_surface = create_light_surface(light_radius, texture_size).unwrap();
        Renderer {
            light_texture: texture_creator.create_texture_from_surface(
                light_surface
            ).unwrap(),
            window_size: Cell::new((0, 0))
        }
    }

    pub fn render(&self, scene: &Scene, canvas: &mut Canvas) {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        self.window_size.set(canvas.output_size().unwrap());

        self.render_current_layer(canvas, &scene);

        for &cell in visible_area().shifted_by(scene.maze.position()).boundary() {
            if scene.maze.current_layer().has(cell) {
                self.render_square(
                    canvas,
                    cell, Color::RGB(240, 240, 240), scene.camera
                );
            }
        }

        if DEBUG {
            let layer_info = scene.maze.clone_current_layer_info();

            for (&coord, coord_info) in layer_info.coords.iter() {
                if coord_info.escapable.is_some() {
                    self.render_square(
                        canvas,
                        coord, Color::RGB(192, 192, 192), scene.camera
                    );
                }
            }

            for &coord in &layer_info.leaf_escapables {
                self.render_square(
                    canvas,
                    coord, Color::RGB(220, 192, 192), scene.camera
                );
            }
        }

        self.render_square(
            canvas,
            scene.maze.position(), Color::RGBA(0, 192, 0, 255), scene.camera
        );

        let mut light_center = self.to_view(scene.maze.position(), scene.camera);
        light_center.0 += CELL_SIZE as i32 / 2;
        light_center.1 += CELL_SIZE as i32 / 2;

        let query = self.light_texture.query();
        canvas.copy(
            &self.light_texture,
            None,
            Some(Rect::from_center(
                light_center,
                query.width, query.height
            ))
        ).unwrap();
    }

    fn render_current_layer(&self, canvas: &mut Canvas, scene: &Scene) {
        let layer: &Layer = scene.maze.current_layer();

        let cells_iter: Vec<_> = if DEBUG {
            const RENDER_SIZE: i32 = 20;
            let range = -RENDER_SIZE..=RENDER_SIZE;
            range.clone().cartesian_product(range).collect()
        } else {
            visible_area().shifted_by(scene.maze.position()).cells().iter().cloned().collect()
        };

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        for cell in cells_iter {
            if layer.has(cell) {
                let view_coord = self.to_view(cell, scene.camera);

                self.fill_rect(canvas, view_coord.0, view_coord.1, CELL_SIZE - 1, CELL_SIZE - 1);
                if layer.passable(cell, Dir::DOWN) {
                    self.fill_rect(canvas, view_coord.0, view_coord.1, CELL_SIZE - 1, CELL_SIZE);
                }
                if layer.passable(cell, Dir::RIGHT) {
                    self.fill_rect(canvas, view_coord.0, view_coord.1, CELL_SIZE, CELL_SIZE - 1);
                }
            }
        }
    }

    fn render_square(&self, canvas: &mut Canvas, coord: (i32, i32), color: Color, camera: Camera) {
        canvas.set_draw_color(color);
        let view_coord = self.to_view(coord, camera);
        self.fill_rect(canvas, view_coord.0, view_coord.1, CELL_SIZE - 1, CELL_SIZE - 1);
    }

    fn to_view(&self, scene_coord: (i32, i32), scene_camera: Camera) -> (i32, i32) {
        let window_size = self.window_size.get();
        let view_camera = (window_size.0 as f32 / 2.0, window_size.1 as f32 / 2.0);
        let x = (scene_coord.0 as f32 - scene_camera.0) * CELL_SIZE as f32 + view_camera.0;
        let y = (scene_coord.1 as f32 - scene_camera.1) * CELL_SIZE as f32 + view_camera.1;
        (x as i32, y as i32)
    }

    fn fill_rect<X, Y, W, H>(
        &self,
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
}

fn create_light_surface(radius: u32, size: u32) -> Result<Surface<'static>, String> {
    let max_opacity = if DEBUG { 192 } else { 255 };
    let center = size / 2;
    let surface = Surface::new(size, size, sdl2::pixels::PixelFormatEnum::RGBA32)?;
    let mut canvas = surface.into_canvas()?;
    for i in 0..size {
        for j in 0..size {
            let x = i as i32 - center as i32;
            let y = j as i32 - center as i32;
            let d_squared = x.pow(2) + y.pow(2);
            if d_squared < (radius as i32).pow(2) {
                let k = d_squared as f32 / (radius as f32).powi(2);
                let opacity = (k.powi(3) * f32::from(max_opacity)) as u8;
                canvas.set_draw_color(Color::RGBA(0, 0, 0, opacity));
            } else {
                canvas.set_draw_color(Color::RGBA(0, 0, 0, max_opacity));
            }
            canvas.draw_point((i as i32, j as i32)).unwrap();
        }
    }
    Ok(canvas.into_surface())
}
