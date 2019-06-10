use itertools::Itertools;
use rand::Rng;

use crate::layer::Layer;
use crate::region::Region;
use crate::maze::Maze;
use crate::geometry::direction::{DIRECTIONS, Dir};
use crate::geometry::coord::Coord;
use crate::generation::generate;

pub fn make_circle(radius: i32) -> impl Iterator<Item=Coord> {
    (-radius..=radius).cartesian_product(-radius..=radius)
        .filter_map(move |(x, y)| {
            if x.pow(2) + y.pow(2) < radius.pow(2) {
                Some(Coord::new(x, y))
            } else {
                None
            }
        })
}

pub fn generate_layer<'l>(
    shape: impl IntoIterator<Item=&'l Coord>,
    spawn_point: Coord,
    rng: &mut impl Rng
) -> Layer {
    use std::iter::once;

    let mut layer = Layer::default();
    for &coord in shape {
        layer.add(coord);
    }
    generate(&mut layer, once(spawn_point), &Default::default(), rng);

    layer
}

fn find_cell_at_boundary(
    layer: &Layer,
    coord: Coord, back: Dir,
    region: &Region
) -> Option<(Coord, Dir)> {
    let mut dir = back.rotate_clockwise();
    while dir != back {
        if layer.passable(coord, dir) {
            let to = coord.advance(dir);

            if region.boundary().contains(&to) {
                return Some((to, dir));
            }

            if let Some((escape, _)) = find_cell_at_boundary(
                layer, to, dir.opposite(), region
            ) {
                return Some((escape, dir));
            }
        }
        dir = dir.rotate_clockwise();
    }
    None
}

fn with_copied_region(source: &Layer, region: &Region) -> Layer {
    let mut result = Layer::default();
    for &cell in region.cells().iter()
        .chain(region.boundary())
    {
        if source.has(cell) {
            result.add(cell);
        }
    }
    for &cell in region.cells() {
        for &dir in &DIRECTIONS {
            if source.passable(cell, dir) {
                result.join(cell, dir);
            }
        }
    }
    result
}

pub fn add_layer_seamlessly(
    maze: &mut Maze,
    source_layer_index: usize,
    source_coord: Coord,
    back: Dir,
    shape: impl Iterator<Item=Coord>,
    visible_area: &Region,
    rng: &mut impl Rng
) -> usize {
    let source_layer = maze.clone_layer(source_layer_index);

    let (escape, escape_dir) = find_cell_at_boundary(
        &source_layer,
        source_coord, back,
        &visible_area.shifted_by(source_coord)
    ).expect("Trying to add transition at non-escapable cell.");

    let copy_region = visible_area.shifted_by(source_coord);
    let mut new_layer = with_copied_region(&source_layer, &copy_region);
    for coord in shape {
        new_layer.add(coord);
    }
    generate(&mut new_layer, std::iter::once(escape), copy_region.cells(), rng);

    let new_layer_index = maze.add_layer(new_layer);
    maze.add_transition(source_coord, escape_dir, source_layer_index, new_layer_index);

    new_layer_index
}
