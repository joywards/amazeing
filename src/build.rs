use std::collections::{HashSet, VecDeque};
use rand::rngs::SmallRng;

use crate::layer::Layer;
use crate::utils::region::Region;
use crate::visible_area::visible_area;
use crate::maze::{Maze, LazyCellInfo};
use crate::geometry::{Dir, DIRECTIONS};
use crate::generation::generate;
use crate::traversal;

fn reachable_cells(
    layer: &Layer<LazyCellInfo>,
    from: (i32, i32),
    region: &Region
) -> HashSet<(i32, i32)> {
    assert!(layer.has(from));

    let mut reached = HashSet::new();
    reached.insert(from);

    let mut queue = VecDeque::new();
    queue.push_back(from);
    while let Some(c) = queue.pop_front() {
        for &dir in &DIRECTIONS {
            let to = c + dir;
            if region.cells().contains(&to)
                && layer.passable(c, dir)
                && !reached.contains(&to)
            {
                queue.push_back(to);
                reached.insert(to);
            }
        }
    }
    reached
}

fn copy_region(
    src: &Layer<LazyCellInfo>,
    pos: (i32, i32),
    src_index: usize,
    dst: &mut Layer<LazyCellInfo>,
    region: &Region
) {
    for &cell in region.cells() {
        if src.has(cell) {
            assert!(dst.has(cell));
            for &dir in &DIRECTIONS {
                if src.passable(cell, dir) {
                    dst.join(cell, dir);
                }
            }
        }
    }
    for cell in reachable_cells(src, pos, region) {
        *dst.get_info_mut(cell).unwrap() = match *src.get_info(cell).unwrap() {
            LazyCellInfo::Some(_) => LazyCellInfo::Ref(src_index),
            LazyCellInfo::Ref(to) => LazyCellInfo::Ref(to),
        }
    }
}

fn carve_path<I: Default>(layer: &mut Layer<I>, start: (i32, i32), path: &[Dir]) {
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


pub struct MazeBuilder<'r> {
    maze: Option<Maze>,

    shape: Vec<(i32, i32)>,
    rng: &'r mut SmallRng,
}

impl<'r> MazeBuilder<'r> {
    pub fn new(shape: Vec<(i32, i32)>, rng: &mut SmallRng) -> MazeBuilder {
        MazeBuilder{
            maze: None, shape,
            rng
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
        copy_region(&maze_layer.layer, source_coord, source_layer_index, &mut new_layer, &region_to_copy);

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

        let new_layer_index = maze.add_layer(new_layer, info, source_layer_index);
        maze.add_transition(source_coord, escape_dir, source_layer_index, new_layer_index);

        new_layer_index
    }

    pub fn generate_first_layer(
        &mut self,
        spawn_point: (i32, i32)
    ) -> usize {
        self.generate_first_layer_from_multiple(&[spawn_point])
    }

    pub fn generate_first_layer_from_multiple(
        &mut self,
        spawn_points: &[(i32, i32)]
    ) -> usize {
        assert!(!spawn_points.is_empty());
        let mut layer = Layer::from_shape(&self.shape);
        for &spawn_point in &spawn_points[1..] {
            layer.treat_as_reachable(spawn_points[0], spawn_point);
        }
        generate(&mut layer, spawn_points.iter().copied(), &Default::default(), &mut self.rng);

        self.maze = Some(Maze::new(layer, spawn_points[0]));

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
        layer_index: usize,
    ) {
        let info = self.traversal_info(layer_index);
        let (&deepest, _) = info.coords.iter().max_by_key(
            |(_coord, info)| info.depth
        ).expect("layer has no reachable cells");

        let maze = self.maze.as_mut().unwrap();
        maze.set_finish((deepest.0, deepest.1, layer_index));
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
