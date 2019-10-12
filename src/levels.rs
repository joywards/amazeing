use rand::rngs::SmallRng;
use rand::SeedableRng;
use rand::prelude::*;
use itertools::Itertools;

use crate::build::{MazeBuilder, GenerationError};
use crate::geometry_sets::{
    make_circle,
    make_ring,
    make_lemniscate,
    make_hourglass,
};
use crate::maze::Maze;
use crate::visible_area::visibility_radius;


pub trait LevelGenerator: Send + Sync {
    fn id(&self) -> &'static str;
    /// Recommended number of stages to complete before proceeding to the next level.
    fn recommended_length(&self) -> u32;

    fn intro_text(&self) -> &'static str;

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
    pub static ref GENERATORS: [&'static dyn LevelGenerator; 6] = {
        [
            &Plain(),
            &Ring(),
            &Lemniscate(),
            &Hourglass(),
            &DeceptivelySmall(),
            &TrickySquare(),
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
    fn intro_text(&self) -> &'static str { "Let's start with something simple." }
    fn recommended_length(&self) -> u32 { 3 }
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
    // TODO show help about taking a hint only when the player seems stuck.
    fn intro_text(&self) -> &'static str {
        "So, you get the idea. Lets try something more challenging.\n\n\
        By the way, you can press \"space\" to move backwards and a backtick \
        when you feel desperate (not guaranteed to help though)."
    }
    fn recommended_length(&self) -> u32 { 1 }
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
        let last = builder.fork_to_two_layers(first)?.0;
        builder.set_finish_at_deepest_point(last);

        Ok(builder.into_maze())
    }

    fn id(&self) -> &'static str { "lemniscate" }
    fn intro_text(&self) -> &'static str {
        "If something seems wrong, don't worry - it's just your mind playing tricks on you."
    }
    fn recommended_length(&self) -> u32 { 3 }
}


pub struct Hourglass();

impl LevelGenerator for Hourglass {
    fn try_generate(&self, stage: u32, rng: &mut SmallRng) -> Result<Maze, GenerationError> {
        let radius = 10 + stage as i32;
        let depth = 1 + stage / 2;
        let shape = make_hourglass(radius).collect();

        let mut builder = MazeBuilder::new(shape, rng);

        let mut last = builder.generate_first_layer_from_multiple(
            &[(0, 0), (0, -1)]
        );
        for _ in 0..depth {
            last = builder.fork_to_two_layers(last)?.1
        }
        builder.set_finish_at_deepest_point(last);

        Ok(builder.into_maze())
    }

    fn id(&self) -> &'static str { "hourglass" }
    fn intro_text(&self) -> &'static str {
        "If you start feeling dizzy, nauseous, desperate or miserable, \
        you should probably stop playing."
    }
    fn recommended_length(&self) -> u32 { 4 }
}


pub struct DeceptivelySmall();

impl LevelGenerator for DeceptivelySmall {
    fn try_generate(&self, stage: u32, rng: &mut SmallRng) -> Result<Maze, GenerationError> {
        let radius = visibility_radius() - 2;
        let depth = 1 + stage;
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
    fn intro_text(&self) -> &'static str {
        "Never give up! Even when it seems that there is no way out."
    }
    fn recommended_length(&self) -> u32 { 4 }
}


pub struct TrickySquare();

impl LevelGenerator for TrickySquare {
    fn try_generate(&self, stage: u32, rng: &mut SmallRng) -> Result<Maze, GenerationError> {
        let size = 12 + stage as i32;
        let depth = std::cmp::max(6, stage / 2);
        let shape = (-size..=size).cartesian_product(-size..=size).collect();
        let mut builder = MazeBuilder::new(shape, rng);

        let first = builder.generate_first_layer((0, size));
        let (mut left, mut center, mut right) = builder.fork_to_three_layers(first)?;
        for _ in 0..depth {
            left = builder.add_layer_from_deepest_point(left)?;
            right = builder.add_layer_from_deepest_point(right)?;
        }
        if stage > 0 {
            center = builder.add_layer_from_deepest_point(center)?;
        }
        builder.set_finish_at_deepest_point(center);
        Ok(builder.into_maze())
    }

    fn id(&self) -> &'static str { "tricky_square" }
    fn intro_text(&self) -> &'static str {
        "Well, you still think that you can trick me just by sticking to the wall, huh? \
        Playtime is over. Taste some real stuff!"
    }
    fn recommended_length(&self) -> u32 { 3 }
}
