use itertools::Itertools;
use std::cell::Cell;
use std::cmp;

use sdl2::pixels::Color;
use sdl2::surface::Surface;
use sdl2::rect::Rect;

use crate::geometry::Dir;
use crate::visible_area::{visibility_radius, visible_area};
use crate::scene::{Scene, Camera};
use crate::render::{Canvas, Target};
use crate::maze::{CellInfo};

const CELL_SIZE: u32 = 17;
const DEBUG: bool = false;
const INVISIBLE_CELLS_BRIGHTNESS: u8 = 96;

pub struct Renderer {
    window_size: Cell<(u32, u32)>,
}

impl Renderer {
    pub fn new() -> Self {
        Renderer {
            window_size: Cell::new((0, 0))
        }
    }
    pub fn initialize(&self, renderer: &mut Target) {
        if renderer.textures.light.is_none() {
            let light_surface = create_light_surface().unwrap();
            renderer.textures.light = Some(
                renderer.texture_creator.create_texture_from_surface(light_surface).unwrap()
            );
        }
    }

    pub fn render(&self, scene: &Scene, renderer: &mut Target) {
        self.initialize(renderer);
        let canvas = &mut renderer.canvas;

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        self.window_size.set(canvas.output_size().unwrap());

        self.render_current_layer(canvas, &scene);

        if DEBUG {
            for &cell in visible_area().shifted_by(scene.maze.position()).boundary() {
                if scene.maze.current_layer().has(cell) {
                    self.render_square(
                        canvas,
                        cell, Color::RGB(240, 240, 240), scene
                    );
                }
            }

            let layer_info = scene.maze.current_layer_info();

            for (&coord, coord_info) in layer_info.coords.iter() {
                if coord_info.escapable.is_some() {
                    self.render_square(
                        canvas,
                        coord, Color::RGB(192, 192, 192), scene
                    );
                }
            }

            for &coord in &layer_info.leaf_escapables {
                self.render_square(
                    canvas,
                    coord, Color::RGB(220, 192, 192), scene
                );
            }
        }

        // Current position
        self.render_square(
            canvas,
            scene.maze.position(), Color::RGBA(128, 128, 255, 255), scene
        );

        if !DEBUG {
            let mut light_center = self.to_view(scene.maze.position(), scene.camera);
            light_center.0 += CELL_SIZE as i32 / 2;
            light_center.1 += CELL_SIZE as i32 / 2;

            let light_texture: &_ = renderer.textures.light.as_ref().unwrap();
            let query = light_texture.query();
            canvas.copy(
                light_texture,
                None,
                Some(Rect::from_center(
                    light_center,
                    query.width, query.height
                ))
            ).unwrap();
        }
    }

    fn render_current_layer(&self, canvas: &mut Canvas, scene: &Scene) {
        let layer = scene.maze.current_layer();

        let cells_iter: Vec<_> = if DEBUG {
            const RENDER_SIZE: i32 = 20;
            let range = -RENDER_SIZE..=RENDER_SIZE;
            range.clone().cartesian_product(range).collect()
        } else {
            visible_area().shifted_by(scene.maze.position()).cells().iter().cloned().collect()
        };

        for cell in cells_iter {
            if layer.has(cell) {
                let visual_info = scene.visual_info.get(&cell);
                let br = visual_info.map(|info| info.brightness)
                    .unwrap_or(INVISIBLE_CELLS_BRIGHTNESS);
                let visible = visual_info.map(|info| info.directly_reachable)
                    .unwrap_or(false);
                if visible {
                    canvas.set_draw_color(match layer.get_info(cell).unwrap() {
                        CellInfo::Untouched => Color::RGB(br, br, br),
                        CellInfo::Visited => Color::RGB(cmp::min(208, br), cmp::min(208, br), br),
                        CellInfo::Finish => Color::RGB(0, br / 4 * 3, 0),
                    });
                } else {
                    canvas.set_draw_color(Color::RGB(br, br, br));
                }

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

    fn render_square(&self, canvas: &mut Canvas, coord: (i32, i32), color: Color, scene: &Scene) {
        let camera = scene.camera;
        let brightness = f32::from(
            scene.visual_info.get(&coord).map(|info| info.brightness)
                .unwrap_or(INVISIBLE_CELLS_BRIGHTNESS)
        ) / 255.0;
        let color = Color::RGBA(
            (f32::from(color.r) * brightness) as u8,
            (f32::from(color.g) * brightness) as u8,
            (f32::from(color.b) * brightness) as u8,
            255
        );
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

fn create_light_surface() -> Result<Surface<'static>, String> {
    let radius = ((visibility_radius() as f32 - 1. / 2_f32.sqrt()) * CELL_SIZE as f32) as u32;
    let size = visibility_radius() as u32 * 2 * CELL_SIZE;
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
                let opacity = (k.powi(3) * 255.0) as u8;
                canvas.set_draw_color(Color::RGBA(0, 0, 0, opacity));
            } else {
                canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
            }
            canvas.draw_point((i as i32, j as i32)).unwrap();
        }
    }
    Ok(canvas.into_surface())
}
