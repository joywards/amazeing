extern crate disjoint_sets;

use std::marker::PhantomData;
use disjoint_sets::UnionFind;


/// Represents bijection between elements of type `Self` and nonnegative integers.
pub trait Ordinal {
    fn ordinal(object: Self) -> usize;

    fn from_ordinal(_ordinal: usize) -> Self
    where Self: std::marker::Sized {
        unimplemented!();
    }
}

impl Ordinal for isize {
    fn ordinal(number: isize) -> usize {
        if number >= 0 {
            number as usize * 2
        } else {
            -number as usize * 2 - 1
        }
    }

    fn from_ordinal(ordinal: usize) -> isize {
        if ordinal % 2 == 0 {
            (ordinal / 2) as isize
        } else {
            -(((ordinal + 1) / 2) as isize)
        }
    }
}

#[test]
fn test_ordinal_isize() {
    assert_eq!(isize::ordinal(0), 0);
    assert_eq!(isize::ordinal(-1), 1);
    assert_eq!(isize::ordinal(1), 2);
    assert_eq!(isize::ordinal(-2), 3);
    assert_eq!(isize::ordinal(2), 4);
    assert_eq!(isize::ordinal(-3), 5);
    assert_eq!(isize::ordinal(1 << 30), 1 << 31);
    assert_eq!(isize::ordinal(-(1 << 30)), (1 << 31) - 1);
}

#[test]
fn test_from_ordinal_isize() {
    for i in (-16..16).chain([1 << 30, -(1 << 30)].iter().map(|&x| x)) {
        assert_eq!(i, isize::from_ordinal(isize::ordinal(i)));
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
impl Ordinal for &(usize, usize) {
    fn ordinal(pair: Self) -> usize {
        let sum = pair.0 + pair.1;
        sum * (sum + 1) / 2 + pair.0
    }
}

#[test]
fn test_ordinal_pair_usize() {
    assert_eq!(<&(usize, usize)>::ordinal(&(0, 0)), 0);
    assert_eq!(<&(usize, usize)>::ordinal(&(0, 1)), 1);
    assert_eq!(<&(usize, usize)>::ordinal(&(1, 0)), 2);
    assert_eq!(<&(usize, usize)>::ordinal(&(0, 2)), 3);
    assert_eq!(<&(usize, usize)>::ordinal(&(1, 1)), 4);
    assert_eq!(<&(usize, usize)>::ordinal(&(2, 0)), 5);
    assert_eq!(<&(usize, usize)>::ordinal(&(0, 1 << 15)), (1 << 14) * ((1 << 15) + 1));
}


impl Ordinal for &(isize, isize) {
    fn ordinal(pair: Self) -> usize {
        Ordinal::ordinal(&(isize::ordinal(pair.0), isize::ordinal(pair.1)))
    }
}

#[test]
fn test_ordinal_pair_isize() {
    assert_eq!(<&(isize, isize)>::ordinal(&(-1, -1)), 4);
    assert_eq!(<&(isize, isize)>::ordinal(&(-1, 0)), 2);
    assert_eq!(<&(isize, isize)>::ordinal(&(-1, 1)), 7);
    assert_eq!(<&(isize, isize)>::ordinal(&(0, -1)), 1);
    assert_eq!(<&(isize, isize)>::ordinal(&(0, 0)), 0);
    assert_eq!(<&(isize, isize)>::ordinal(&(0, 1)), 3);
    assert_eq!(<&(isize, isize)>::ordinal(&(1, -1)), 8);
    assert_eq!(<&(isize, isize)>::ordinal(&(1, 0)), 5);
    assert_eq!(<&(isize, isize)>::ordinal(&(1, 1)), 12);
}


#[derive(Default, Clone)]
pub struct DSU<T> {
    union_find: UnionFind<usize>,
    phantom: PhantomData<T>,
}

impl<T: Ordinal> DSU<T> {
    fn get_index(&mut self, a: T) -> usize {
        let a_i = Ordinal::ordinal(a);
        while self.union_find.len() <= a_i {
            self.union_find.alloc();
        }
        a_i
    }

    pub fn new() -> Self {
        DSU::<T> {
            union_find: UnionFind::<usize>::new(0),
            phantom: PhantomData,
        }
    }

    pub fn union(&mut self, a: T, b: T) -> bool {
        let a_i = self.get_index(a);
        let b_i = self.get_index(b);
        self.union_find.union(a_i, b_i)
    }

    pub fn equiv(&self, a: T, b: T) -> bool {
        let a_i = Ordinal::ordinal(a);
        let b_i = Ordinal::ordinal(b);
        if a_i >= self.union_find.len() || b_i >= self.union_find.len() {
            a_i == b_i
        } else {
            self.union_find.equiv(a_i, b_i)
        }
    }
}

#[test]
fn test_dsu() {
    let mut dsu = DSU::<&(isize, isize)>::new();
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
