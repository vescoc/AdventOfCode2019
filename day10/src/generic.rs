use std::cmp::{self, Ordering};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Add, Mul, Sub};

use crate::{ASTEROID, EMPTY_SPACE, INPUT};

lazy_static! {
    static ref DATA: Vec<Point<i32>> = parse(&INPUT);
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

pub fn distance<T>(p1: &Point<T>, p2: &Point<T>) -> T
where
    T: Copy + Hash + Eq + Debug + Default + Sub<T, Output = T> + Add<T, Output = T> + cmp::Ord,
{
    let abs = |v| {
        if v < T::default() {
            T::default() - v
        } else {
            v
        }
    };

    abs(p1.x() - p2.x()) + abs(p1.y() + p2.y())
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

pub fn solve_1<T>(points: &[Point<T>]) -> (&Point<T>, usize)
where
    T: Eq
        + Hash
        + Copy
        + Sub<T, Output = T>
        + Mul<T, Output = T>
        + Default
        + PartialEq
        + PartialOrd
        + Debug
        + cmp::Ord,
{
    let mut los = vec![];

    let points = &points[..];

    for a in 0..points.len() {
        let pa = &points[a];
        for b in a + 1..points.len() {
            let pb = &points[b];

            if !points
                .iter()
                .enumerate()
                .any(|(i, p)| i != a && i != b && contains(&pa, &pb, p))
            {
                los.push((pa, pb));
            }
        }
    }

    los.iter()
        .fold(HashMap::<&Point<T>, usize>::new(), |mut acc, (p1, p2)| {
            acc.entry(p1).and_modify(|v| *v += 1).or_insert(1);
            acc.entry(p2).and_modify(|v| *v += 1).or_insert(1);
            acc
        })
        .into_iter()
        .max_by(|(_, va), (_, vb)| va.cmp(&vb))
        .unwrap()
}

pub fn solve_2<'a, T>(points: &'a [Point<T>], center: &Point<T>) -> &'a Point<T>
where
    T: Eq
        + Hash
        + Copy
        + Sub<T, Output = T>
        + Add<T, Output = T>
        + Mul<T, Output = T>
        + Default
        + PartialEq
        + PartialOrd
        + Debug
        + From<i32>
        + cmp::Ord
        + Into<f64>,
{
    let mut v = points
        .iter()
        .filter_map(|p| {
            if p != center
                && !points
                    .iter()
                    .any(|c| c != p && c != center && contains(center, p, c))
            {
                Some((
                    p,
                    ((p.x() - center.x()).into()).atan2((p.y() - center.y()).into()),
                ))
            } else {
                None
            }
        })
        .collect::<Vec<(&'a Point<T>, f64)>>();

    v.sort_by(|(_, b), (_, a)| {
        if a > b {
            Ordering::Greater
        } else if (a - b).abs() < std::f64::EPSILON {
            Ordering::Equal
        } else {
            Ordering::Less
        }
    });

    v[199].0
}

pub fn part_1() -> usize {
    solve_1(&DATA).1
}

pub fn part_2() -> i32 {
    let p = solve_1(&DATA).0;

    let p = solve_2(&DATA, p);

    p.0 * 100 + p.1
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref EXAMPLE: &'static str = r#".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##"#;
    }

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

    #[test]
    fn test_example_1() {
        let q = parse::<i32>(
            r######".#..#
.....
#####
....#
...##"######,
        );

        assert_eq!(solve_1(&q), (&Point(3, 4), 8));
    }

    #[test]
    fn test_example_1_1() {
        assert_eq!(
            solve_1(&parse::<i32>(
                r#"......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####"#
            )),
            (&Point(5, 8), 33)
        );
    }

    #[test]
    fn test_example_1_2() {
        assert_eq!(
            solve_1(&parse::<i32>(
                r#"#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###."#
            )),
            (&Point(1, 2), 35)
        );
    }

    #[test]
    fn test_example_1_3() {
        assert_eq!(
            solve_1(&parse::<i32>(
                r#".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#.."#
            )),
            (&Point(6, 3), 41)
        );
    }

    #[test]
    fn test_example_1_large() {
        assert_eq!(solve_1(&parse(&EXAMPLE)), (&Point(11, 13), 210));
    }

    #[test]
    fn test_example_2_large() {
        assert_eq!(solve_2(&parse(&EXAMPLE), &Point(11, 13)), &Point(8, 2));
    }

    #[test]
    fn test_distance() {
        assert_eq!(distance(&Point(0, 0), &Point(10, 10)), 20);
    }
}
