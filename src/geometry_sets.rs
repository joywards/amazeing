use std::collections::HashSet;
use itertools::Itertools;

pub fn dilate(a: &HashSet<(i32, i32)>, b: &HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    let mut result = HashSet::new();
    for cell in a {
        for shift in b {
            result.insert((cell.0 + shift.0, cell.1 + shift.1));
        }
    }
    result
}

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

pub fn make_ring(inner_radius: i32, outer_radius: i32) -> impl Iterator<Item=(i32, i32)> {
    (-outer_radius..=outer_radius).cartesian_product(-outer_radius..=outer_radius)
        .filter_map(move |(x, y)| {
            let r_sqr = x.pow(2) + y.pow(2);
            if r_sqr < outer_radius.pow(2) && r_sqr >= inner_radius.pow(2) {
                Some((x, y))
            } else {
                None
            }
        })
}
