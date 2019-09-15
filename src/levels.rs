use crate::build::{MazeBuilder, make_circle, GenerationError};
use crate::maze::Maze;


pub trait LevelGenerator: Send + Sync {
    fn generate(&self, stage: u32) -> Result<Maze, GenerationError>;
    fn id(&self) -> &'static str;
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
    fn generate(&self, stage: u32) -> Result<Maze, GenerationError> {
        let radius = 8 + stage as i32;
        let shape = make_circle(radius).collect();
        let mut builder = MazeBuilder::new(stage, shape);
        builder.generate_first_layer((0, 0));
        builder.set_finish_at_deepest_point(0);
        Ok(builder.into_maze())
    }

    fn id(&self) -> &'static str { "plain" }
}


pub struct Double();

impl LevelGenerator for Double {
    fn generate(&self, stage: u32) -> Result<Maze, GenerationError> {
        let radius = 11 + stage as i32;
        let shape = make_circle(radius).collect();
        let mut builder = MazeBuilder::new(stage, shape);
        let first = builder.generate_first_layer((0, 0));
        let last = builder.add_layer_from_deepest_point(first)?;
        builder.set_finish_at_deepest_point(last);
        Ok(builder.into_maze())
    }

    fn id(&self) -> &'static str { "double" }
}


pub struct Debug();

impl LevelGenerator for Debug {
    fn generate(&self, stage: u32) -> Result<Maze, GenerationError> {
        let radius = 12 + stage as i32;
        let shape = make_circle(radius).collect();
        let mut builder = MazeBuilder::new(stage, shape);

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
