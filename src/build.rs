use itertools::Itertools;
use rand::rngs::SmallRng;
use rand::SeedableRng;

use crate::layer::Layer;
use crate::region::Region;
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
            dst.add(cell);
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
    layer_info: Vec<traversal::Info>,

    shape: Vec<(i32, i32)>,
    rng: SmallRng,
}

impl MazeBuilder {
    pub fn new(seed: u64, shape: Vec<(i32, i32)>) -> MazeBuilder {
        MazeBuilder{
            maze: None, layer_info: Vec::new(), shape,
            rng: SmallRng::seed_from_u64(seed)
        }
    }

    pub fn into_maze(self) -> Maze {
        self.maze.unwrap()
    }

    /// Should be used for debug purposes only.
    pub fn into_maze_and_layer_info(self) -> (Maze, Vec<traversal::Info>) {
        (self.maze.unwrap(), self.layer_info)
    }

    fn add_layer(
        &mut self,
        source_layer_index: usize,
        source_coord: (i32, i32),
    ) -> usize {
        let maze = self.maze.as_mut().unwrap();
        let source_layer = maze.clone_layer(source_layer_index);
        let layer_info = &self.layer_info[source_layer_index];
        let back = layer_info.coords[&source_coord].came_from.unwrap();

        let escape = layer_info.coords[&source_coord].escapable.unwrap();
        let path_to_escape = traversal::get_path_to(source_coord, escape, &layer_info);
        let escape_dir = *path_to_escape.first().unwrap();

        let region_to_copy = visible_area().shifted_by(source_coord);
        let mut new_layer = Layer::default();
        copy_region(&source_layer, &mut new_layer, &region_to_copy);
        for &coord in &self.shape {
            new_layer.add(coord);
        }

        // Sometimes escape cell can be blocked out from the copied area during
        // generation. That's why we make it reachable before running generation.
        carve_path(&mut new_layer, source_coord, &path_to_escape);

        generate(
            &mut new_layer, std::iter::once(escape), region_to_copy.cells(),
            &mut self.rng
        );

        self.layer_info.push(traversal::dfs(
            &new_layer, source_coord, Some(back)
        ));

        let new_layer_index = maze.add_layer(new_layer);
        maze.add_transition(source_coord, escape_dir, source_layer_index, new_layer_index);

        assert_eq!(new_layer_index, self.layer_info.len() - 1);
        new_layer_index
    }

    pub fn generate_first_layer(
        &mut self,
        spawn_point: (i32, i32)
    ) -> usize {
        use std::iter::once;

        let mut layer = Layer::default();
        for &coord in &self.shape {
            layer.add(coord);
        }
        generate(&mut layer, once(spawn_point), &Default::default(), &mut self.rng);

        self.layer_info = vec![traversal::dfs(
            &layer, spawn_point, None
        )];
        self.maze = Some(Maze::new(layer, spawn_point));
        0
    }

    pub fn add_layer_from_deepest_point(
        &mut self,
        src_layer: usize,
    ) -> Result<usize, GenerationError> {
        let info = &self.layer_info[src_layer];
        let deepest = *info.leaf_escapables.iter().max_by_key(
            |coord| info.coords[&coord].depth
        ).ok_or(GenerationError{})?;

        Ok(self.add_layer(
            src_layer,
            deepest
        ))
    }

    pub fn fork_to_two_layers(
        &mut self,
        src_layer: usize
    ) -> Result<(usize, usize), GenerationError> {
        let leaf_escapables = &self.layer_info[src_layer].leaf_escapables;
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
        let info = &self.layer_info[src_layer];
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
}

#[cfg(feature = "bench")]
mod benches {

extern crate test;

#[bench]
fn bench_add_layer(b: &mut test::Bencher) {
    use std::collections::HashSet;
    use crate::build::{MazeBuilder, make_circle};
    use crate::region::Region;

    let shape = make_circle(30).collect();
    let mut builder = MazeBuilder::new(0, shape);
    builder.set_visible_area(Region::from(make_circle(12).collect::<HashSet<_>>()));
    builder.generate_first_layer((0, 0));

    b.iter(|| {
        builder.add_layer_from_deepest_point(0);
    });
}

}
