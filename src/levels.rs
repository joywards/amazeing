use rand::rngs::SmallRng;
use rand::SeedableRng;

use crate::build::{MazeBuilder, make_circle, GenerationError};
use crate::maze::Maze;


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
    pub static ref GENERATORS: [&'static dyn LevelGenerator; 3] = {
        [
            &Plain(),
            &Double(),
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


pub struct Double();

impl LevelGenerator for Double {
    fn try_generate(&self, stage: u32, rng: &mut SmallRng) -> Result<Maze, GenerationError> {
        let radius = 11 + stage as i32;
        let shape = make_circle(radius).collect();
        let mut builder = MazeBuilder::new(shape, rng);
        let first = builder.generate_first_layer((0, 0));
        let last = builder.add_layer_from_deepest_point(first)?;
        builder.set_finish_at_deepest_point(last);
        Ok(builder.into_maze())
    }

    fn id(&self) -> &'static str { "double" }
}


pub struct Debug();

impl LevelGenerator for Debug {
    fn try_generate(&self, stage: u32, rng: &mut SmallRng) -> Result<Maze, GenerationError> {
        let radius = 12 + stage as i32;
        let shape = make_circle(radius).collect();
        let mut builder = MazeBuilder::new(shape, rng);

        let first = builder.generate_first_layer((0, 0));
        let (mut left, center, mut right) = builder.fork_to_three_layers(first)?;
        for _ in 0..6 {
            left = builder.add_layer_from_deepest_point(left)?;
        }
        for _ in 0..6 {
            right = builder.add_layer_from_deepest_point(right)?;
        }
        builder.add_layer_from_deepest_point(center)?;
        builder.set_finish_at_deepest_point(0);
        Ok(builder.into_maze())
    }

    fn id(&self) -> &'static str { "debug" }
}
