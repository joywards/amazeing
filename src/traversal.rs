use std::collections::{HashSet, HashMap};

use crate::geometry::direction::Dir;
use crate::layer::Layer;
use crate::region::Region;
use crate::geometry::coord::Coord;


#[derive(Debug)]
pub struct CoordInfo {
    /// Tells whether there is a cell in a subtree
    /// which is far enough from this one.
    ///
    /// Being `Some(escape)` means that new layer
    /// can be added with region around this cell copied
    /// and generation started from `escape` is expected to be successful.
    ///
    /// `None` means that there is no such cell in a subtree.
    pub escapable: Option<Coord>,

    /// Tells whether there is an "escapable" (see above) cell in a subtree.
    pub has_escapable_below: bool,

    pub depth: u32,

    pub came_from: Option<Dir>,
}

#[derive(Default, Debug)]
pub struct Info {
    pub coords: HashMap<Coord, CoordInfo>,

    /// Contains locations of cells which are "escapable" (see above) but
    /// don't have "escapable" cells in their subtree.
    pub leaf_escapables: Vec<Coord>,
}

fn escapable(c: Coord, from: Coord, layer: &Layer, visible_area: &Region) -> bool {
    use crate::geometry::direction::DIRECTIONS;

    let visible_area = visible_area.shifted_by(from);
    assert!(layer.has(c));

    if visible_area.cells().contains(&c)
        || visible_area.boundary().contains(&c)
    {
        return false;
    }

    // For now, just check that layer contains 4 closest cells.
    // It works well for simple layer shapes but more complex approach is
    // needed for strong guarantees.
    for &dir in &DIRECTIONS {
        if !layer.has(c.advance(dir)) {
            return false;
        }
    }

    true
}

pub fn dfs(
    layer: &Layer,
    start: Coord, from: Option<Dir>,
    visible_area: &Region
) -> Info {
    let mut info = Info::default();
    let mut visible_trace = HashSet::default();

    if let Some(from) = from {
        dfs_impl(layer, start, from, &mut info, &mut visible_trace, visible_area, 0);
    } else {
        let back = Dir::DOWN;
        dfs_impl(layer, start, back, &mut info, &mut visible_trace, visible_area, 0);
        if layer.passable(start, back) {
            dfs_impl(
                layer, start.advance(back), back.opposite(),
                &mut info, &mut visible_trace, visible_area, 1
            );
        }
        info.coords.get_mut(&start).unwrap().came_from = None;
    };

    info
}

fn dfs_impl(
    layer: &Layer,
    coord: Coord, from: Dir,
    info: &mut Info,
    visible_trace: &mut HashSet<Coord>,
    visible_area: &Region,
    depth: u32
) {
    let prev = info.coords.insert(coord, CoordInfo{
        escapable: None,
        has_escapable_below: false,
        depth,
        came_from: Some(from)
    });
    if prev.is_some() {
        panic!("Layer contains a loop");
    }

    visible_trace.retain(|c| {
        if escapable(coord, /* from= */ *c, &layer, &visible_area) {
            info.coords.get_mut(c).unwrap().escapable = Some(coord);
            false
        } else {
            true
        }
    });
    visible_trace.insert(coord);

    let mut dir = from.rotate_clockwise();
    while dir != from {
        if layer.passable(coord, dir) {
            let to = coord.advance(dir);
            dfs_impl(
                layer,
                to, dir.opposite(),
                info, visible_trace, visible_area, depth + 1
            );
            if info.coords[&to].has_escapable_below
                || info.coords[&to].escapable.is_some()
            {
                info.coords.get_mut(&coord).unwrap().has_escapable_below = true;
            }
        }
        dir = dir.rotate_clockwise();
    }

    let coord_info = &info.coords[&coord];
    if coord_info.escapable.is_some() && !coord_info.has_escapable_below {
        info.leaf_escapables.push(coord);
    }

    visible_trace.remove(&coord);
}

pub fn get_path_to(from: Coord, to: Coord, info: &Info) -> Vec<Dir> {
    let mut c = to;
    let mut result: Vec<Dir> = Vec::new();
    while c != from {
        let back = info.coords[&c].came_from.unwrap();
        result.push(back.opposite());
        c = c.advance(back);
    }
    result.reverse();

    result
}
