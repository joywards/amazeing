use crate::build::{MazeBuilder, make_circle, GenerationError};
use crate::maze::Maze;


pub trait LevelGenerator {
    fn generate(&self, stage: u64) -> Result<Maze, GenerationError>;
    fn id(&self) -> &'static str;
}


pub struct Plain();

impl LevelGenerator for Plain {
    fn generate(&self, stage: u64) -> Result<Maze, GenerationError> {
        let radius = 12 + stage as i32;
        let shape = make_circle(radius).collect();
        let mut builder = MazeBuilder::new(stage, shape);
        builder.generate_first_layer((0, 0));
        Ok(builder.into_maze())
    }

    fn id(&self) -> &'static str { "plain" }
}


pub struct Debug();

impl LevelGenerator for Debug {
    fn generate(&self, stage: u64) -> Result<Maze, GenerationError> {
        let radius = 17 + stage as i32;
        let shape = make_circle(radius).collect();
        let mut builder = MazeBuilder::new(stage, shape);

        let first = builder.generate_first_layer((0, 0));
        let (_, mut last, _) = builder.fork_to_three_layers(first)?;
        for _ in 0..6 {
            last = builder.add_layer_from_deepest_point(last)?;
        }
        Ok(builder.into_maze())
    }

    fn id(&self) -> &'static str { "debug" }
}
