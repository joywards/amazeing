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

pub fn erose(a: &HashSet<(i32, i32)>, b: &HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    let mut result = HashSet::new();

    'a:
    for &cell in a {
        for shift in b {
            let shifted = (cell.0 + shift.0, cell.1 + shift.1);
            if !a.contains(&shifted) {
                continue 'a;
            }
        }
        result.insert(cell);
    }
    result
}

fn get_border(a: &HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    let erosed = erose(&a, &(-1..=1).cartesian_product(-1..=1).collect());
    a.difference(&erosed).copied().collect()
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

pub fn make_lemniscate(size: f32, breadth: i32) -> impl Iterator<Item=(i32, i32)> {
    let lemniscate = itertools_num::linspace(
        0.0, std::f32::consts::PI / 2.0, size as usize
    ).flat_map(move |t| {
        let d = 1. + t.sin().powi(2);
        let x = (size * t.cos() / d).round() as i32;
        let y = (size * t.sin() * t.cos() / d).round() as i32;
        vec![(x, y), (-x, y), (x, -y), (-x, -y)]
    }).collect();
    dilate(&lemniscate, &make_circle(breadth).collect()).into_iter()
}

#[allow(clippy::collapsible_if)]
pub fn make_hourglass(radius: i32) -> impl Iterator<Item=(i32, i32)> {
    let circle_center_y = (2f32.sqrt() * radius as f32) as i32;
    let half_height = circle_center_y + radius;

    let outer_shape: HashSet<(i32, i32)> = (-radius..=radius).cartesian_product(-half_height..=half_height)
        .filter(move |(x, y)| {
            let y = y.abs();
            let x = x.abs();
            if y >= x && x + y <= circle_center_y {
                return true;
            }
            (y - circle_center_y).pow(2) + x.pow(2) < radius.pow(2)
        }).collect();
    let mut result = get_border(&outer_shape);

    for (x, y) in outer_shape {
        if y < 0 {
            if y > -radius {
                result.insert((x, y));
            }
        } else {
            if x == 0 || y * 5 > x.abs() * 2 + radius * 5 {
                result.insert((x, y));
            }
        }
    }
    result.into_iter()
}
