use std::cmp;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Mul, Sub};

use crate::{contains, Point, DATA};

pub fn solve_1<'a, T>(points: &'a [Point<T>]) -> (&'a Point<T>, usize)
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
            HashMap::<&'a Point<T>, usize>::new(),
            |mut acc, (p1, p2)| {
                acc.entry(&p1).and_modify(|v| *v += 1).or_insert(1);
                acc.entry(&p2).and_modify(|v| *v += 1).or_insert(1);
                acc
            },
        )
        .into_iter()
        .max_by(|(_, va), (_, vb)| va.cmp(&vb))
        .unwrap()
}

pub fn part_1() -> usize {
    solve_1(&DATA).1
}

pub fn part_2() -> u32 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::parse;

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
}
