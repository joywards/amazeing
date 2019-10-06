use rand::rngs::SmallRng;
use rand::SeedableRng;
use rand::prelude::*;

use crate::build::{MazeBuilder, GenerationError};
use crate::geometry_sets::{make_circle, make_ring, make_lemniscate};
use crate::maze::Maze;
use crate::visible_area::visibility_radius;


pub trait LevelGenerator: Send + Sync {
    fn id(&self) -> &'static str;

    fn try_generate(&self, stage: u32, rng: &mut SmallRng) -> Result<Maze, GenerationError>;

    fn generate(&self, stage: u32) -> Maze {
        let mut rng = SmallRng::seed_from_u64(u64::from(stage));
        loop {
            match self.try_generate(stage, &mut rng) {
                Ok(maze) => return maze,
                Err(error) => eprintln!(
                    "Could not generate level \"{}\" on stage {}: {}",
                    self.id(), stage, error
                ),
            }
        }
    }
}


lazy_static! {
    pub static ref GENERATORS: [&'static dyn LevelGenerator; 5] = {
        [
            &Plain(),
            &Ring(),
            &Lemniscate(),
            &DeceptivelySmall(),
            &Debug(),
        ]
    };
}


pub struct Plain();

impl LevelGenerator for Plain {
    fn try_generate(&self, stage: u32, rng: &mut SmallRng) -> Result<Maze, GenerationError> {
        let radius = 8 + stage as i32;
        let shape = make_circle(radius).collect();
        let mut builder = MazeBuilder::new(shape, rng);
        builder.generate_first_layer((0, 0));
        builder.set_finish_at_deepest_point(0);
        Ok(builder.into_maze())
    }

    fn id(&self) -> &'static str { "plain" }
}


pub struct Ring();

impl LevelGenerator for Ring {
    fn try_generate(&self, stage: u32, rng: &mut SmallRng) -> Result<Maze, GenerationError> {
        let outer_radius = 17 + stage as i32 / 2;
        let inner_radius = outer_radius - 9;
        let depth = 1 + stage / 3;
        let shape: Vec<_> = make_ring(inner_radius, outer_radius).collect();
        let spawn = *shape.choose(rng).unwrap();

        let mut builder = MazeBuilder::new(shape, rng);

        let mut last = builder.generate_first_layer(spawn);
        for _ in 0..depth {
            last = builder.add_layer_from_deepest_point(last)?;
        }
        builder.set_finish_at_deepest_point(last);

        Ok(builder.into_maze())
    }

    fn id(&self) -> &'static str { "ring" }
}


pub struct Lemniscate();

impl LevelGenerator for Lemniscate {
    fn try_generate(&self, stage: u32, rng: &mut SmallRng) -> Result<Maze, GenerationError> {
        let size = 20 + stage;
        let breadth = 3 + stage as i32 / 8;
        let mut shape: Vec<_> = make_lemniscate(size as f32, breadth).collect();
        shape.sort();
        let spawn = *shape.choose(rng).unwrap();

        let mut builder = MazeBuilder::new(shape, rng);

        let first = builder.generate_first_layer(spawn);
        let last = if stage < 2 {
            first
        } else {
            builder.fork_to_two_layers(first)?.0
        };
        builder.set_finish_at_deepest_point(last);

        Ok(builder.into_maze())
    }

    fn id(&self) -> &'static str { "lemniscate" }
}


pub struct DeceptivelySmall();

impl LevelGenerator for DeceptivelySmall {
    fn try_generate(&self, stage: u32, rng: &mut SmallRng) -> Result<Maze, GenerationError> {
        let radius = visibility_radius() - 2;
        let depth = 1 + stage / 2;
        let shape = make_circle(radius).collect();
        let mut builder = MazeBuilder::new(shape, rng);
        let mut last = builder.generate_first_layer((0, 0));
        for _ in 0..depth {
            last = builder.add_layer_from_deepest_point(last)?;
        }
        builder.set_finish_at_deepest_point(last);
        Ok(builder.into_maze())
    }

    fn id(&self) -> &'static str { "deceptively_small" }
}


pub struct Debug();

impl LevelGenerator for Debug {
    fn try_generate(&self, stage: u32, rng: &mut SmallRng) -> Result<Maze, GenerationError> {
        let radius = 12 + stage as i32;
        let depth = std::cmp::max(6, stage / 2);
        let shape = make_circle(radius).collect();
        let mut builder = MazeBuilder::new(shape, rng);

        let first = builder.generate_first_layer((0, 0));
        let (mut left, mut center, mut right) = builder.fork_to_three_layers(first)?;
        for _ in 0..depth {
            left = builder.add_layer_from_deepest_point(left)?;
        }
        for _ in 0..depth {
            right = builder.add_layer_from_deepest_point(right)?;
        }
        center = builder.add_layer_from_deepest_point(center)?;
        builder.set_finish_at_deepest_point(center);
        Ok(builder.into_maze())
    }

    fn id(&self) -> &'static str { "debug" }
}
