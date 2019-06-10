use std::collections::{HashSet, HashMap};

use crate::geometry::direction::Dir;
use crate::layer::Layer;
use crate::region::Region;
use crate::geometry::coord::Coord;


#[derive(Debug)]
pub struct CoordInfo {
    /// Tells whether there is a cell in a subtree
    /// which is not seen from this one.
    /// This effectively means that new layer
    /// can be added with region around this cell copied and with transition
    /// at this cell.
    pub escapable: bool,

    /// Tells whether there is an "escapable" (see above) cell in a subtree.
    pub has_escapable_below: bool,

    pub depth: u32,

    pub came_from: Option<Dir>,
}

#[derive(Default, Debug)]
pub struct LayerInfo {
    pub coords: HashMap<Coord, CoordInfo>,

    /// Contains locations of cells which are "escapable" (see above) but
    /// don't have "escapable" cells in their subtree.
    pub leaf_escapables: Vec<Coord>,
}

pub fn dfs(
    layer: &Layer,
    start: Coord, from: Option<Dir>,
    visible_area: &Region
) -> LayerInfo {
    let from = from.expect("Passing dir=None is not implemented yet");
    let mut info = LayerInfo::default();
    let mut visible_trace = HashSet::default();
    dfs_impl(layer, start, from, &mut info, &mut visible_trace, visible_area, 0);
    info
}

fn dfs_impl(
    layer: &Layer,
    coord: Coord, from: Dir,
    info: &mut LayerInfo,
    visible_trace: &mut HashSet<Coord>,
    visible_area: &Region,
    depth: u32
) {
    let prev = info.coords.insert(coord, CoordInfo{
        escapable: false,
        has_escapable_below: false,
        depth,
        came_from: Some(from)
    });
    if prev.is_some() {
        panic!("Layer contains a loop");
    }

    visible_trace.retain(|c| {
        if visible_area.cells().contains(&(coord - *c)) {
            true
        } else {
            info.coords.get_mut(c).unwrap().escapable = true;
            false
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
                || info.coords[&to].escapable
            {
                info.coords.get_mut(&coord).unwrap().has_escapable_below = true;
            }
        }
        dir = dir.rotate_clockwise();
    }

    let coord_info = &info.coords[&coord];
    if coord_info.escapable && !coord_info.has_escapable_below {
        info.leaf_escapables.push(coord);
    }

    visible_trace.remove(&coord);
}
