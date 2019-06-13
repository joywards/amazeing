use itertools::Itertools;
use rand::rngs::SmallRng;
use rand::SeedableRng;

use crate::layer::Layer;
use crate::region::Region;
use crate::maze::Maze;
use crate::geometry::direction::{DIRECTIONS, Dir};
use crate::geometry::coord::Coord;
use crate::generation::generate;
use crate::traversal;

pub fn make_circle(radius: i32) -> impl Iterator<Item=Coord> {
    (-radius..=radius).cartesian_product(-radius..=radius)
        .filter_map(move |(x, y)| {
            if x.pow(2) + y.pow(2) < radius.pow(2) {
                Some(Coord::new(x, y))
            } else {
                None
            }
        })
}

fn find_cell_at_boundary(
    layer: &Layer,
    coord: Coord, back: Dir,
    region: &Region
) -> Option<(Coord, Dir)> {
    let mut dir = back.rotate_clockwise();
    while dir != back {
        if layer.passable(coord, dir) {
            let to = coord.advance(dir);

            if region.boundary().contains(&to) {
                return Some((to, dir));
            }

            if let Some((escape, _)) = find_cell_at_boundary(
                layer, to, dir.opposite(), region
            ) {
                return Some((escape, dir));
            }
        }
        dir = dir.rotate_clockwise();
    }
    None
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


pub struct MazeBuilder {
    maze: Option<Maze>,
    layer_info: Vec<traversal::Info>,

    shape: Vec<Coord>,
    visible_area: Option<Region>,
    // Sometimes "escape cell" gets cornered and no passage from it can be
    // created after copying region from previous layer.
    // To fix this we use a wider visible area when looking for escape cells.
    // This workaround fixes most of the cases, but further work is required to
    // guarantee that generation will always work.
    extended_visible_area: Option<Region>,
    rng: SmallRng,
}

impl MazeBuilder {
    pub fn new(seed: u64, shape: Vec<Coord>) -> MazeBuilder {
        MazeBuilder{
            maze: None, layer_info: Vec::new(), shape,
            visible_area: None, extended_visible_area: None,
            rng: SmallRng::seed_from_u64(seed)
        }
    }

    pub fn set_visible_area(&mut self, visible_area: Region) {
        self.visible_area = Some(visible_area);
        self.extended_visible_area = Some(Region::from(
            self.visible_area.as_ref().unwrap().cells().union(
                self.visible_area.as_ref().unwrap().boundary()
            ).cloned().collect::<std::collections::HashSet<_>>()
        ));
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
        source_coord: Coord,
    ) -> usize {
        let maze = self.maze.as_mut().unwrap();
        let source_layer = maze.clone_layer(source_layer_index);
        let back = self.layer_info[source_layer_index].coords[&source_coord]
            .came_from.unwrap();

        let (escape, escape_dir) = find_cell_at_boundary(
            &source_layer,
            source_coord, back,
            &self.extended_visible_area.as_ref().unwrap().shifted_by(source_coord)
        ).expect("Trying to add transition at non-escapable cell.");

        let region_to_copy = self.visible_area.as_ref().unwrap().shifted_by(source_coord);
        let mut new_layer = Layer::default();
        copy_region(&source_layer, &mut new_layer, &region_to_copy);
        for &coord in &self.shape {
            new_layer.add(coord);
        }
        generate(
            &mut new_layer, std::iter::once(escape), region_to_copy.cells(),
            &mut self.rng
        );

        self.layer_info.push(traversal::dfs(
                &new_layer, source_coord, Some(back),
                self.extended_visible_area.as_ref().unwrap()
            )
        );

        let new_layer_index = maze.add_layer(new_layer);
        maze.add_transition(source_coord, escape_dir, source_layer_index, new_layer_index);

        assert_eq!(new_layer_index, self.layer_info.len() - 1);
        new_layer_index
    }

    pub fn generate_first_layer(
        &mut self,
        spawn_point: Coord
    ) -> usize {
        use std::iter::once;

        let mut layer = Layer::default();
        for &coord in &self.shape {
            layer.add(coord);
        }
        generate(&mut layer, once(spawn_point), &Default::default(), &mut self.rng);

        self.layer_info.push(traversal::dfs(
                &layer, spawn_point, None, self.extended_visible_area.as_ref().unwrap()
            )
        );
        self.maze = Some(Maze::new(layer, spawn_point));
        0
    }

    pub fn add_layer_from_deepest_point(
        &mut self,
        src_layer: usize,
    ) -> usize {
        let info = &self.layer_info[src_layer];
        let deepest = *info.leaf_escapables.iter().max_by_key(
            |coord| info.coords[&coord].depth
        ).unwrap();

        self.add_layer(
            src_layer,
            deepest
        )
    }

    pub fn fork_to_two_layers(
        &mut self,
        src_layer: usize
    ) -> (usize, usize) {
        let leaf_escapables = &self.layer_info[src_layer].leaf_escapables;
        assert!(leaf_escapables.len() >= 2);
        let first = *leaf_escapables.first().unwrap();
        let last = *leaf_escapables.last().unwrap();
        (self.add_layer(src_layer, first), self.add_layer(src_layer, last))
    }

    pub fn fork_to_three_layers(
        &mut self,
        src_layer: usize
    ) -> (usize, usize, usize) {
        let info = &self.layer_info[src_layer];
        let leaf_escapables = &info.leaf_escapables;

        assert!(leaf_escapables.len() >= 3);
        let first = *leaf_escapables.first().unwrap();
        let last = *leaf_escapables.last().unwrap();
        let deepest = *leaf_escapables[1..leaf_escapables.len() - 1].iter().max_by_key(
            |coord| info.coords[&coord].depth
        ).unwrap();
        (
            self.add_layer(src_layer, first),
            self.add_layer(src_layer, deepest),
            self.add_layer(src_layer, last)
        )
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
    builder.generate_first_layer((0, 0).into());

    b.iter(|| {
        builder.add_layer_from_deepest_point(0);
    });
}

}
