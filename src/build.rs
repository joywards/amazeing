use itertools::Itertools;
use rand::rngs::SmallRng;
use rand::SeedableRng;

use crate::layer::Layer;
use crate::utils::region::Region;
use crate::visible_area::visible_area;
use crate::maze::Maze;
use crate::geometry::{Dir, DIRECTIONS};
use crate::generation::generate;
use crate::traversal;

pub fn make_circle(radius: i32) -> impl Iterator<Item=(i32, i32)> {
    (-radius..=radius).cartesian_product(-radius..=radius)
        .filter_map(move |(x, y)| {
            if x.pow(2) + y.pow(2) < radius.pow(2) {
                Some((x, y))
            } else {
                None
            }
        })
}

fn copy_region(src: &Layer, dst: &mut Layer, region: &Region) {
    for &cell in region.cells().iter()
        .chain(region.boundary())
    {
        if src.has(cell) {
            assert!(dst.has(cell));
        }
    }
    for &cell in region.cells() {
        for &dir in &DIRECTIONS {
            if src.passable(cell, dir) {
                dst.join(cell, dir);
            }
        }
    }
}

fn carve_path(layer: &mut Layer, start: (i32, i32), path: &[Dir]) {
    let mut c = start;
    for &dir in path {
        layer.join(c, dir);
        c = c + dir;
    }
}


#[derive(Debug, Default)]
pub struct GenerationError {}

impl std::fmt::Display for GenerationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Generation error")
    }
}
impl std::error::Error for GenerationError {}


pub struct MazeBuilder {
    maze: Option<Maze>,

    shape: Vec<(i32, i32)>,
    rng: SmallRng,
}

impl MazeBuilder {
    pub fn new(seed: u64, shape: Vec<(i32, i32)>) -> MazeBuilder {
        MazeBuilder{
            maze: None, shape,
            rng: SmallRng::seed_from_u64(seed)
        }
    }

    pub fn into_maze(self) -> Maze {
        self.maze.unwrap()
    }

    fn traversal_info(&mut self, src_layer: usize) -> &traversal::Info {
        &self.maze.as_mut().unwrap().maze_layer(src_layer).info
    }

    fn add_layer(
        &mut self,
        source_layer_index: usize,
        source_coord: (i32, i32),
    ) -> usize {
        let maze = self.maze.as_mut().unwrap();
        let maze_layer = maze.maze_layer(source_layer_index);
        let info = &maze_layer.info;
        let back = info.coords[&source_coord].came_from.unwrap();

        let escape = info.coords[&source_coord].escapable.unwrap();
        let path_to_escape = traversal::get_path_to(source_coord, escape, &info);
        let escape_dir = *path_to_escape.first().unwrap();

        let mut new_layer = Layer::from_shape(&self.shape);
        let region_to_copy = visible_area().shifted_by(source_coord);
        copy_region(&maze_layer.layer, &mut new_layer, &region_to_copy);

        // Sometimes escape cell can be blocked out from the copied area during
        // generation. That's why we make it reachable before running generation.
        carve_path(&mut new_layer, source_coord, &path_to_escape);

        generate(
            &mut new_layer, std::iter::once(escape), region_to_copy.cells(),
            &mut self.rng
        );

        let info = traversal::dfs(
            &new_layer, source_coord, Some(back)
        );

        let new_layer_index = maze.add_layer(new_layer, info);
        maze.add_transition(source_coord, escape_dir, source_layer_index, new_layer_index);

        new_layer_index
    }

    pub fn generate_first_layer(
        &mut self,
        spawn_point: (i32, i32)
    ) -> usize {
        use std::iter::once;

        let mut layer = Layer::from_shape(&self.shape);
        generate(&mut layer, once(spawn_point), &Default::default(), &mut self.rng);

        self.maze = Some(Maze::new(layer, spawn_point));

        self.set_finish_at_deepest_point(0);

        0
    }

    pub fn add_layer_from_deepest_point(
        &mut self,
        src_layer: usize,
    ) -> Result<usize, GenerationError> {
        let info = self.traversal_info(src_layer);
        let deepest = *info.leaf_escapables.iter().max_by_key(
            |coord| info.coords[&coord].depth
        ).ok_or(GenerationError{})?;

        let new_layer_index = self.add_layer(
            src_layer,
            deepest
        );
        self.set_finish_at_deepest_point(new_layer_index);

        Ok(new_layer_index)
    }

    pub fn fork_to_two_layers(
        &mut self,
        src_layer: usize
    ) -> Result<(usize, usize), GenerationError> {
        let leaf_escapables = &self.traversal_info(src_layer).leaf_escapables;
        if leaf_escapables.len() < 2 {
            return Err(GenerationError{});
        }
        let first = *leaf_escapables.first().unwrap();
        let last = *leaf_escapables.last().unwrap();
        Ok((
            self.add_layer(src_layer, first),
            self.add_layer(src_layer, last)
        ))
    }

    pub fn fork_to_three_layers(
        &mut self,
        src_layer: usize
    ) -> Result<(usize, usize, usize), GenerationError> {
        let info = &self.traversal_info(src_layer);
        let leaf_escapables = &info.leaf_escapables;

        if leaf_escapables.len() < 3 {
            return Err(GenerationError{});
        }
        let first = *leaf_escapables.first().unwrap();
        let last = *leaf_escapables.last().unwrap();
        let deepest = *leaf_escapables[1..leaf_escapables.len() - 1].iter().max_by_key(
            |coord| info.coords[&coord].depth
        ).unwrap();
        Ok((
            self.add_layer(src_layer, first),
            self.add_layer(src_layer, deepest),
            self.add_layer(src_layer, last)
        ))
    }

    pub fn set_finish_at_deepest_point(
        &mut self,
        src_layer: usize,
    ) {
        let info = self.traversal_info(src_layer);
        let (&deepest, _) = info.coords.iter().max_by_key(
            |(_coord, info)| info.depth
        ).expect("layer has no reachable cells");

        let maze = self.maze.as_mut().unwrap();
        maze.set_finish(src_layer, deepest);
    }
}

#[cfg(feature = "bench")]
mod benches {

extern crate test;

#[bench]
fn bench_add_layer(b: &mut test::Bencher) {
    use crate::build::{MazeBuilder, make_circle};

    let shape = make_circle(15).collect();
    let mut builder = MazeBuilder::new(0, shape);
    builder.generate_first_layer((0, 0));

    b.iter(|| {
        builder.add_layer_from_deepest_point(0).unwrap();
    });
}

}
