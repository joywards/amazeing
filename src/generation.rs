extern crate rand;

use std::collections::HashSet;
use crate::layer::Layer;
use crate::geometry::{Dir, DIRECTIONS};
use rand::Rng;
use rand::seq::SliceRandom;

const CHANCE_TO_BE_NEXT: f64 = 0.07;

fn possible_moves(layer: &Layer, from: (i32, i32), blocked_cells: &HashSet<(i32, i32)>) -> Vec<Dir> {
    if !layer.has(from) {
        return vec![];
    }

    let mut result= vec![];
    for &dir in &DIRECTIONS {
        let to = from + dir;
        if layer.has(to)
            && !blocked_cells.contains(&to)
            && !layer.reachable(from, to)
        {
            result.push(dir);
        }
    }
    result
}

fn expand_randomly<R>(
    layer: &mut Layer, from: (i32, i32), blocked_cells: &HashSet<(i32, i32)>,
    rng: &mut R
) -> Option<Dir>
    where R: Rng + ?Sized
{
    let moves = possible_moves(&layer, from, blocked_cells);
    moves.as_slice().choose(rng).map(|&dir| {
        layer.join(from, dir);
        dir
    })
}

// There is probably space for optimization here.
pub fn generate<R: Rng + ?Sized>(
    layer: &mut Layer,
    spawn_points: impl Iterator<Item=(i32, i32)>,
    blocked_cells: &HashSet<(i32, i32)>,
    rng: &mut R
) {
    let mut queue: Vec<(i32, i32)> = spawn_points.collect();

    while !queue.is_empty() {
        while queue.iter().last().map_or(false,
            |cell| possible_moves(&layer, *cell, blocked_cells).is_empty()
        ) {
            queue.pop();
        }
        let mut new_cell: Option<(i32, i32)> = None;
        for cell in queue.iter().rev() {
            if rng.gen_bool(CHANCE_TO_BE_NEXT) {
                if let Some(dir) = expand_randomly(layer, *cell, blocked_cells, rng) {
                    new_cell = Some(*cell + dir);
                }
                break;
            }
        }
        if let Some(cell) = new_cell {
            queue.push(cell);
        }
    }
}


#[test]
fn test_generation() {
    use rand::SeedableRng;
    use rand::rngs::SmallRng;
    use itertools::Itertools;

    let mut rng = SmallRng::seed_from_u64(0);
    let shape = (-100..100).cartesian_product(-100..100).collect::<Vec<_>>();
    let mut layer = Layer::from_shape(&shape);
    generate(&mut layer, std::iter::once((0, 0)), &HashSet::new(), &mut rng);
    for &(x, y) in &shape {
        assert!(layer.reachable((0, 0), (x, y)));
    }
}

#[cfg(feature = "bench")]
mod benches {

extern crate test;

use rand::rngs::SmallRng;
use rand::SeedableRng;

use crate::build::make_circle;
use crate::layer::Layer;

#[bench]
fn bench_generation(b: &mut test::Bencher) {
    use crate::generation::generate;

    let mut layer = Layer::from_shape(&make_circle(30).collect::<Vec<_>>());
    let layer = &layer;
    let mut rng = SmallRng::seed_from_u64(0);
    let blocked_cells = std::collections::HashSet::new();

    b.iter(|| {
        generate(
            &mut layer.clone(),
            std::iter::once((0, 0)),
            &blocked_cells,
            &mut rng
        );
    });
}

}
