use itertools::Itertools;
use rand::Rng;

use crate::layer::Layer;
use crate::region::Region;
use crate::geometry::direction::DIRECTIONS;
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

pub fn generate_with_copied_region<'l>(
    shape: impl Iterator<Item=Coord>,
    source: &Layer,
    region: impl Into<&'l Region>,
    rng: &mut impl Rng
) -> Layer {
    let region = region.into();
    let mut layer = with_copied_region(source, &region);
    for coord in shape {
        layer.add(coord);
    }
    generate(&mut layer, region.boundary().iter().cloned(), region.cells(), rng);
    layer
}
