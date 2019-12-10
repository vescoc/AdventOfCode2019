#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

use std::cmp;
use std::convert::TryFrom;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Mul, Sub};

pub mod simple;

const ASTEROID: char = '#';
const EMPTY_SPACE: char = '.';

lazy_static! {
    pub static ref DATA: Vec<Point<i32>> = parse(include_str!("../data.txt"));
}

#[derive(Debug, PartialEq, Hash, Eq)]
pub struct Point<T>(T, T)
where
    T: Hash + Eq + Debug;

impl<T> Point<T>
where
    T: Copy + Hash + Eq + Debug,
{
    pub fn x(&self) -> T {
        self.0
    }

    pub fn y(&self) -> T {
        self.1
    }
}

pub fn parse<T>(s: &str) -> Vec<Point<T>>
where
    T: From<u16> + Hash + Eq + Debug,
{
    s.lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.trim()
                .chars()
                .enumerate()
                .filter_map(move |(x, c)| match c {
                    ASTEROID => {
                        let x = T::try_from(x as u16).unwrap_or_else(|_| panic!("invalid x index"));
                        let y = T::try_from(y as u16).unwrap_or_else(|_| panic!("invalid y index"));
                        Some(Point(x, y))
                    }
                    EMPTY_SPACE => None,
                    _ => unreachable!(),
                })
        })
        .collect()
}

pub fn contains<T>(pa: &Point<T>, pb: &Point<T>, p: &Point<T>) -> bool
where
    T: Copy
        + Sub<T, Output = T>
        + Mul<T, Output = T>
        + Default
        + PartialEq
        + PartialOrd
        + Hash
        + Eq
        + Debug
        + cmp::Ord,
{
    let dx = pb.x() - pa.x();
    let dy = pb.y() - pa.y();
    let in_x = || p.x() >= cmp::min(pa.x(), pb.x()) && p.x() <= cmp::max(pa.x(), pb.x());
    let in_y = || p.y() >= cmp::min(pa.y(), pb.y()) && p.y() <= cmp::max(pa.y(), pb.y());

    if dx == T::default() {
        if dy == T::default() {
            false
        } else {
            p.x() == pa.x() && in_y()
        }
    } else if dy == T::default() {
        p.y() == pa.y() && in_x()
    } else {
        in_x() && in_y() && (p.x() - pa.x()) * dy == (p.y() - pa.y()) * dx
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            parse::<u16>(
                r###"##
.#"###
            ),
            vec![Point(0, 0), Point(1, 0), Point(1, 1)]
        );
    }

    #[test]
    fn test_contains() {
        assert!(contains(&Point(0, 0), &Point(2, 2), &Point(1, 1)));
    }

    #[test]
    fn test_contains_1() {
        assert!(contains(&Point(1, 0), &Point(3, 4), &Point(2, 2)));
        assert!(contains(&Point(3, 4), &Point(1, 0), &Point(2, 2)));
        assert!(!contains(&Point(1, 0), &Point(2, 2), &Point(3, 4)));
        assert!(!contains(&Point(3, 4), &Point(2, 2), &Point(1, 0)));
        assert!(!contains(&Point(1, 0), &Point(4, 0), &Point(1, 2)));
        assert!(!contains(&Point(4, 3), &Point(3, 4), &Point(4, 4)));
    }
}
