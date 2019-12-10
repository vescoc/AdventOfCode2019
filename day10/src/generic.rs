use std::cmp;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Mul, Sub, Add};
use std::convert::TryFrom;

use crate::{INPUT, ASTEROID, EMPTY_SPACE};

lazy_static! {
    static ref WIDTH: i32 = INPUT
        .lines()
        .count() as i32;

    static ref HEIGHT: i32 = INPUT
        .trim()
        .lines()
        .last()
        .unwrap()
        .len() as i32;

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
    let abs = |v| if v < T::default() { T::default() - v } else { v };

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
        .fold(
            HashMap::<&Point<T>, usize>::new(),
            |mut acc, (p1, p2)| {
                acc.entry(p1).and_modify(|v| *v += 1).or_insert(1);
                acc.entry(p2).and_modify(|v| *v += 1).or_insert(1);
                acc
            },
        )
        .into_iter()
        .max_by(|(_, va), (_, vb)| va.cmp(&vb))
        .unwrap()
}

pub fn solve_2<'a, T>(points: &'a [Point<T>], center: &Point<T>, width: T, height: T) -> &'a Point<T>
where
    T: Eq
        + Hash
        + Copy
        + Sub<T, Output = T>
    + Mul<T, Output = T>
    + Add<T, Output = T>
        + Default
        + PartialEq
        + PartialOrd
    + Debug
    + From<i32>
    + cmp::Ord
{
    println!("center: {:?} width: {:?} height: {:?}", center, width, height);
    
    (0..)
        .scan(
            (
                vec![center].into_iter().collect::<HashSet<_>>(),
                Point(center.x(), T::from(-110)),
                vec![
                    (T::from(230), (T::from(0), T::from(1))),
                    (T::from(230), (T::from(-1), T::from(0))),
                    (T::from(230), (T::from(0), T::from(-1))),
                    (T::from(230), (T::from(1), T::from(0))),
                ]
                    .into_iter()
                    .cycle(),
                center.x(),
                (width - T::from(1), (T::from(1), T::from(0))),
            ),
            |state, _| {
                let r = if let Some(p) = points
                    .iter()
                    .filter(|p| !state.0.contains(p))
                    .filter_map(|p| if contains(center, &state.1, p) { Some((p, distance(center, p))) } else { None })
                    .min_by(|(_, d1), (_, d2)| d1.cmp(&d2))
                {
                    state.0.insert(p.0);
                    Some(Some(p.0))
                } else {
                    Some(None)
                };

                println!("i: {:?} r: {:?} border: {:?} limits: {:?} len: {}", state.3, r, state.1, state.4, state.0.len());

                let (limit, (dx, dy)) = state.4;
                if limit == state.3 {
                    state.4 = state.2.next().unwrap();
                    let (_, (dx, dy)) = state.4;

                    state.1 = Point(state.1.x() + dx.to_owned(), state.1.y() + dy.to_owned());
                    state.3 = T::default();
                } else {
                    state.1 = Point(state.1.x() + dx.to_owned(), state.1.y() + dy.to_owned());
                    state.3 = state.3 + T::from(1);
                }
                                
                r
            }
        )
        .flatten()
        .skip(199)
        .next()
        .unwrap()
}

pub fn part_1() -> usize {
    solve_1(&DATA).1
}

pub fn part_2() -> i32 {
    let p = solve_1(&DATA).0;
    
    let p = solve_2(&DATA, p, *WIDTH, *HEIGHT);

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
        assert_eq!(solve_1(&parse::<i32>(r#"......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####"#)), (&Point(5, 8), 33));
    }
    
    #[test]
    fn test_example_1_2() {
        assert_eq!(solve_1(&parse::<i32>(r#"#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###."#)), (&Point(1, 2), 35));
    }
    
    #[test]
    fn test_example_1_3() {
        assert_eq!(solve_1(&parse::<i32>(r#".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#.."#)), (&Point(6, 3), 41));
    }

    #[test]
    fn test_example_1_large() {
        assert_eq!(solve_1(&parse(&EXAMPLE)), (&Point(11, 13), 210));
    }

    #[test]
    fn test_distance() {
        assert_eq!(distance(&Point(0, 0), &Point(10, 10)), 20);
    }
}
