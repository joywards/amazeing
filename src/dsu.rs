extern crate disjoint_sets;

use std::marker::PhantomData;
use disjoint_sets::UnionFind;


/// Represents bijection between elements of type `Self` and nonnegative integers.
pub trait Ordinal {
    fn ordinal(object: Self) -> u32;

    fn from_ordinal(_ordinal: u32) -> Self
    where Self: std::marker::Sized {
        unimplemented!();
    }
}

impl Ordinal for i32 {
    fn ordinal(number: i32) -> u32 {
        if number >= 0 {
            number as u32 * 2
        } else {
            -number as u32 * 2 - 1
        }
    }

    fn from_ordinal(ordinal: u32) -> i32 {
        if ordinal % 2 == 0 {
            (ordinal / 2) as i32
        } else {
            -(((ordinal + 1) / 2) as i32)
        }
    }
}

#[test]
fn test_ordinal_i32() {
    assert_eq!(i32::ordinal(0), 0);
    assert_eq!(i32::ordinal(-1), 1);
    assert_eq!(i32::ordinal(1), 2);
    assert_eq!(i32::ordinal(-2), 3);
    assert_eq!(i32::ordinal(2), 4);
    assert_eq!(i32::ordinal(-3), 5);
    assert_eq!(i32::ordinal(1 << 30), 1 << 31);
    assert_eq!(i32::ordinal(-(1 << 30)), (1 << 31) - 1);
}

#[test]
fn test_from_ordinal_i32() {
    for i in (-16..16).chain([1 << 30, -(1 << 30)].iter().map(|&x| x)) {
        assert_eq!(i, i32::from_ordinal(i32::ordinal(i)));
    }
}


/// Maps pairs of nonnegative numbers to nonnegative numbers as illustrated:
///```text
///  /|\
/// y |
///   | .
///   | :
///   3 6
///   2 3 7
///   1 1 4 8
///   0 0 2 5 9 ...
///   O-0-1-2-3------->
///                  x
///```
impl Ordinal for &(u32, u32) {
    fn ordinal(pair: Self) -> u32 {
        let sum = pair.0 + pair.1;
        sum * (sum + 1) / 2 + pair.0
    }
}

#[test]
fn test_ordinal_pair_u32() {
    assert_eq!(<&(u32, u32)>::ordinal(&(0, 0)), 0);
    assert_eq!(<&(u32, u32)>::ordinal(&(0, 1)), 1);
    assert_eq!(<&(u32, u32)>::ordinal(&(1, 0)), 2);
    assert_eq!(<&(u32, u32)>::ordinal(&(0, 2)), 3);
    assert_eq!(<&(u32, u32)>::ordinal(&(1, 1)), 4);
    assert_eq!(<&(u32, u32)>::ordinal(&(2, 0)), 5);
    assert_eq!(<&(u32, u32)>::ordinal(&(0, 1 << 15)), (1 << 14) * ((1 << 15) + 1));
}


impl Ordinal for &(i32, i32) {
    fn ordinal(pair: Self) -> u32 {
        Ordinal::ordinal(&(i32::ordinal(pair.0), i32::ordinal(pair.1)))
    }
}

#[test]
fn test_ordinal_pair_i32() {
    assert_eq!(<&(i32, i32)>::ordinal(&(-1, -1)), 4);
    assert_eq!(<&(i32, i32)>::ordinal(&(-1, 0)), 2);
    assert_eq!(<&(i32, i32)>::ordinal(&(-1, 1)), 7);
    assert_eq!(<&(i32, i32)>::ordinal(&(0, -1)), 1);
    assert_eq!(<&(i32, i32)>::ordinal(&(0, 0)), 0);
    assert_eq!(<&(i32, i32)>::ordinal(&(0, 1)), 3);
    assert_eq!(<&(i32, i32)>::ordinal(&(1, -1)), 8);
    assert_eq!(<&(i32, i32)>::ordinal(&(1, 0)), 5);
    assert_eq!(<&(i32, i32)>::ordinal(&(1, 1)), 12);
}


#[derive(Default, Clone)]
pub struct DSU<T> {
    union_find: UnionFind<u32>,
    phantom: PhantomData<T>,
}

impl<T: Ordinal> DSU<T> {
    fn get_index(&mut self, a: T) -> u32 {
        let a_i = Ordinal::ordinal(a);
        while self.union_find.len() as u32 <= a_i {
            self.union_find.alloc();
        }
        a_i
    }

    pub fn union(&mut self, a: T, b: T) -> bool {
        let a_i = self.get_index(a);
        let b_i = self.get_index(b);
        self.union_find.union(a_i, b_i)
    }

    pub fn equiv(&self, a: T, b: T) -> bool {
        let a_i = Ordinal::ordinal(a);
        let b_i = Ordinal::ordinal(b);
        if a_i >= self.union_find.len() as u32
            || b_i >= self.union_find.len() as u32
        {
            a_i == b_i
        } else {
            self.union_find.equiv(a_i, b_i)
        }
    }
}

#[test]
fn test_dsu() {
    let mut dsu = DSU::<&(i32, i32)>::new();
    assert!(!dsu.equiv(&(0, 0), &(1, 0)));
    assert!(dsu.equiv(&(0, 0), &(0, 0)));
    dsu.union(&(0, 0), &(0, 1));
    dsu.union(&(1, 0), &(1, 1));
    dsu.union(&(2, 1), &(2, 0));
    assert!(!dsu.equiv(&(0, 0), &(1, 0)));
    dsu.union(&(0, 1), &(1, 1));
    assert!(dsu.equiv(&(0, 0), &(1, 0)));
    assert!(!dsu.equiv(&(0, 0), &(2, 0)));
}
