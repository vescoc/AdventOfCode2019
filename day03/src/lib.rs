#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

use std::cmp;
use std::collections::{HashMap, HashSet};
use std::convert;
use std::ops;

use serde_derive::{Deserialize, Serialize};

#[derive(PartialEq, Hash, Deserialize, Serialize, Debug, Clone, Copy)]
pub struct Point(i32,i32);

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self(x, y)
    }

    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn abs(&self) -> u32 {
        (self.0.abs() + self.1.abs()) as u32
    }

    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn x(&self) -> i32 {
        self.0
    }

    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn y(&self) -> i32 {
        self.1
    }
}

impl ops::Sub for &Point {
    type Output = Point;

    fn sub(self, rhs: &Point) -> Point {
        Point::new(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl ops::AddAssign for Point {
    fn add_assign(&mut self, point: Point) {
        self.0 += point.0;
        self.1 += point.1;
    }
}

impl convert::From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Self {
        Self::new(x, y)
    }
}

impl cmp::Eq for Point {}

type Path = (HashSet<Point>, HashMap<Point, u32>);

pub const SEPARATOR: char = ',';

lazy_static! {
    pub static ref INPUT: &'static str = include_str!("../data.txt");
    
    pub static ref DATA: Vec<Path> = INPUT
        .lines()
        .map(|l| make_path(l.trim().split(SEPARATOR)))
        .collect();
    pub static ref ORIGIN: Point = Point::new(0, 0);
}

#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn manhattan_distance(p1: &Point, p2: &Point) -> u32 {
    (p1 - p2).abs()
}

pub fn closest_intersection<'a>(path1: &'a Path, path2: &'a Path) -> &'a Point {
    path1
        .0
        .intersection(&path2.0)
        .min_by(|a, b| (a.abs()).cmp(&(b.abs())))
        .unwrap()
}

pub fn better_intersection<'a>(path1: &'a Path, path2: &'a Path) -> (&'a Point, u32) {
    path1
        .0
        .intersection(&path2.0)
        .map(|p| (p, path1.1[p] + path2.1[p]))
        .min_by(|(_, d1), (_, d2)| d1.cmp(d2))
        .unwrap()
}

pub fn make_path<'a>(actions: impl Iterator<Item = &'a str>) -> Path {
    actions
        .scan(((0, 0), 0), |state, s| {
            let mut chars = s.chars();

            let d = chars.next();
            let steps = chars.collect::<String>().parse::<u32>().unwrap();

            let ((x, y), s) = *state;
            let (dx, dy) = match d {
                Some('R') => (1, 0),
                Some('D') => (0, 1),
                Some('L') => (-1, 0),
                Some('U') => (0, -1),
                _ => panic!("invalid action {}", s),
            };
            *state = (((x + steps as i32 * dx), y + steps as i32 * dy), s + steps);
            Some((1..=steps).map(move |c| ((x + c as i32 * dx, y + c as i32 * dy), s + c)))
        })
        .flatten()
        .fold((HashSet::new(), HashMap::new()), |mut acc, p| {
            acc.0.insert(Point::from(p.0));
            acc.1
                .entry(Point::from(p.0))
                .and_modify(|e| *e = cmp::min(*e, p.1))
                .or_insert(p.1);
            acc
        })
}

pub fn part_1() -> u32 {
    manhattan_distance(closest_intersection(&DATA[0], &DATA[1]), &ORIGIN)
}

pub fn part_2() -> u32 {
    better_intersection(&DATA[0], &DATA[1]).1
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example_1_0() {
        assert_eq!(
            manhattan_distance(
                closest_intersection(
                    &make_path(String::from("R8,U5,L5,D3").split(SEPARATOR)),
                    &make_path(String::from("U7,R6,D4,L4").split(SEPARATOR))
                ),
                &ORIGIN
            ),
            6
        )
    }

    #[test]
    fn example_1_1() {
        assert_eq!(
            manhattan_distance(
                closest_intersection(
                    &make_path(String::from("R75,D30,R83,U83,L12,D49,R71,U7,L72").split(SEPARATOR)),
                    &make_path(String::from("U62,R66,U55,R34,D71,R55,D58,R83").split(SEPARATOR))
                ),
                &ORIGIN
            ),
            159
        )
    }

    #[test]
    fn example_1_2() {
        assert_eq!(
            manhattan_distance(
                closest_intersection(
                    &make_path(
                        String::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51")
                            .split(SEPARATOR)
                    ),
                    &make_path(
                        String::from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7").split(SEPARATOR)
                    )
                ),
                &ORIGIN
            ),
            135
        )
    }

    #[test]
    fn example_2_0() {
        assert_eq!(
            better_intersection(
                &make_path(String::from("R8,U5,L5,D3").split(SEPARATOR)),
                &make_path(String::from("U7,R6,D4,L4").split(SEPARATOR))
            )
            .1,
            30
        )
    }

    #[test]
    fn example_2_1() {
        assert_eq!(
            better_intersection(
                &make_path(String::from("R75,D30,R83,U83,L12,D49,R71,U7,L72").split(SEPARATOR)),
                &make_path(String::from("U62,R66,U55,R34,D71,R55,D58,R83").split(SEPARATOR))
            )
            .1,
            610
        )
    }

    #[test]
    fn example_2_2() {
        assert_eq!(
            better_intersection(
                &make_path(
                    String::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51").split(SEPARATOR)
                ),
                &make_path(String::from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7").split(SEPARATOR))
            )
            .1,
            410
        )
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        b.iter(part_1);
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        b.iter(part_2);
    }
}
