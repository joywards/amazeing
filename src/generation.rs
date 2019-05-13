extern crate rand;

use crate::layer::Layer;
use crate::geometry::coord::Coord;
use crate::geometry::direction::{Dir, DIRECTIONS};
use rand::Rng;
use rand::seq::SliceRandom;

const CHANCE_TO_BE_NEXT: f64 = 0.07;

fn possible_moves(layer: &Layer, from: Coord) -> Vec<Dir> {
    let mut result= vec![];
    for &dir in &DIRECTIONS {
        let to = from.advance(dir);
        if layer.has(to) && !layer.reachable(from, to) {
            result.push(dir);
        }
    }
    result
}

fn expand_randomly<R>(layer: &mut Layer, from: Coord, rng: &mut R)
    -> Option<Dir>
    where R: Rng + ?Sized
{
    let moves = possible_moves(&layer, from);
    moves.as_slice().choose(rng).map(|&dir| {
        layer.join(from, dir);
        dir
    })
}

// There is probably space for optimization here.
pub fn generate<R: Rng + ?Sized>(layer: &mut Layer, origin: Coord, rng: &mut R) {
    assert!(layer.has(origin));
    let mut queue: Vec<Coord> = vec![origin];

    while !queue.is_empty() {
        while queue.iter().last().map_or(false,
            |cell| possible_moves(&layer, *cell).is_empty()
        ) {
            queue.pop();
        }
        let mut new_cell: Option<Coord> = None;
        for cell in queue.iter().rev() {
            if rng.gen_bool(CHANCE_TO_BE_NEXT) {
                if let Some(dir) = expand_randomly(layer, *cell, rng) {
                    new_cell = Some(cell.advance(dir));
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

    let mut rng = SmallRng::seed_from_u64(0);
    let mut layer = Layer::default();
    for x in -100..100 {
        for y in -100..100 {
            layer.add((x, y));
        }
    }
    generate(&mut layer, Coord::new(0, 0), &mut rng);
    for x in -100..100 {
        for y in -100..100 {
            assert!(layer.reachable((0, 0).into(), (x, y).into()));
        }
    }
}

