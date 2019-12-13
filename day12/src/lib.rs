#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

use std::cmp::{Ord, Ordering};
use std::collections::HashMap;
use std::fmt;
use std::ops::{AddAssign, SubAssign};
use std::str::FromStr;

use regex::Regex;

lazy_static! {
    pub static ref RE: Regex =
        Regex::new(r"<x=(?P<x>-?\d+), y=(?P<y>-?\d+), z=(?P<z>-?\d+)>").unwrap();
    pub static ref DATA: Vec<Moon<i64>> = Moon::parse(include_str!("../data.txt"));
}

pub struct Moon<T>((T, T, T), (T, T, T));

impl<T> fmt::Debug for Moon<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt.write_fmt(format_args!(
            r"Moon[pos=<x={:?}, y={:?}, z={:?}> vel=<x={:?}, y={:?}, z={:?}>]",
            self.pos().0,
            self.pos().1,
            self.pos().2,
            self.vel().0,
            self.vel().1,
            self.vel().2
        ))
    }
}

impl<T> Clone for Moon<T>
where
    T: Copy,
{
    fn clone(&self) -> Self {
        Moon(*self.pos(), *self.vel())
    }
}

impl<T> PartialEq for Moon<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Moon<T>) -> bool {
        self.pos() == other.pos() && self.vel() == other.vel()
    }
}

impl<T> Moon<T> {
    pub fn pos(&self) -> &(T, T, T) {
        &self.0
    }

    pub fn pos_mut(&mut self) -> &mut (T, T, T) {
        &mut self.0
    }

    pub fn vel(&self) -> &(T, T, T) {
        &self.1
    }

    pub fn vel_mut(&mut self) -> &mut (T, T, T) {
        &mut self.1
    }
}

impl<T> Moon<T>
where
    T: Ord + Copy + AddAssign<T> + SubAssign<T> + From<i64>,
{
    pub fn gravity_interaction(&mut self, other: &mut Moon<T>) -> &Self {
        let g = |(pa, pb): (T, T), (va, vb): (&mut T, &mut T)| match pa.cmp(&pb) {
            Ordering::Less => {
                *va += T::from(1);
                *vb -= T::from(1);
            }
            Ordering::Greater => {
                *va += T::from(-1);
                *vb -= T::from(-1);
            }
            Ordering::Equal => {}
        };

        g(
            (self.pos().0, other.pos().0),
            (&mut self.vel_mut().0, &mut other.vel_mut().0),
        );
        g(
            (self.pos().1, other.pos().1),
            (&mut self.vel_mut().1, &mut other.vel_mut().1),
        );
        g(
            (self.pos().2, other.pos().2),
            (&mut self.vel_mut().2, &mut other.vel_mut().2),
        );

        self
    }

    pub fn step(&mut self) -> &Self {
        let s = |v: T, p: &mut T| {
            *p += v;
        };

        s(self.vel().0, &mut self.pos_mut().0);
        s(self.vel().1, &mut self.pos_mut().1);
        s(self.vel().2, &mut self.pos_mut().2);

        self
    }
}

impl Moon<i64> {
    pub fn total_energy(&self) -> i64 {
        (self.pos().0.abs() + self.pos().1.abs() + self.pos().2.abs())
            * (self.vel().0.abs() + self.vel().1.abs() + self.vel().2.abs())
    }
}

impl<T> fmt::Display for Moon<T>
where
    T: fmt::Display,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt.write_fmt(format_args!(
            r"Moon[pos=<x={}, y={}, z={}> vel=<x={}, y={}, z={}>]",
            self.pos().0,
            self.pos().1,
            self.pos().2,
            self.vel().0,
            self.vel().1,
            self.vel().2
        ))
    }
}

#[derive(Debug)]
pub enum Error {
    InvalidFormat,
    InvalidX,
    InvalidY,
    InvalidZ,
}

impl<T> FromStr for Moon<T>
where
    T: FromStr + Default + Copy,
{
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        if let Some(c) = RE.captures_iter(s).next() {
            Ok(Moon::<T>(
                (
                    c["x"].parse().map_err(|_| Error::InvalidX)?,
                    c["y"].parse().map_err(|_| Error::InvalidY)?,
                    c["z"].parse().map_err(|_| Error::InvalidZ)?,
                ),
                (T::default(), T::default(), T::default()),
            ))
        } else {
            Err(Error::InvalidFormat)
        }
    }
}

impl<T> Moon<T>
where
    T: FromStr + Default + Copy,
{
    pub fn parse(s: &str) -> Vec<Moon<T>> {
        s.lines().map(|l| l.trim().parse().unwrap()).collect()
    }
}

pub fn step(moons: &mut [Moon<i64>]) -> &[Moon<i64>] {
    for i in 0..moons.len() - 1 {
        let (a, b) = moons.split_at_mut(i + 1);
        for m in b {
            a[i].gravity_interaction(m);
        }
    }
    
    moons.iter_mut().for_each(|m| { m.step(); });

    moons
}

pub fn total_energy(moons: &[Moon<i64>]) -> i64 {
    moons.iter().map(|m| m.total_energy()).sum()
}

pub fn cycle(p: &mut [i64], v: &mut [i64]) -> (usize, usize) {
    let mut map = HashMap::<(Vec<i64>, Vec<i64>), usize>::new();
    for i in 0.. {
        let snapshot = (p.to_vec(), v.to_vec());

        if let Some(&index) = map.get(&snapshot) {
            return (i, index);
        } else {
            map.insert(snapshot, i);
        }

        for i in 0..p.len() {
            for j in i..p.len() {
                match p[i].cmp(&p[j]) {
                    Ordering::Less => {
                        v[i] += 1;
                        v[j] -= 1;
                    }
                    Ordering::Greater => {
                        v[i] += -1;
                        v[j] -= -1;
                    }
                    Ordering::Equal => {}
                }
            }
        }
        
        for i in 0..p.len() {
            p[i] += v[i];
        }
    }

    unreachable!()
}

pub fn solve_1(moons: &[Moon<i64>], steps: usize) -> i64 {
    let mut moons = moons.to_vec();
    let moons = moons.as_mut_slice();

    (0..steps).for_each(|_| {
        step(moons);
    });

    total_energy(moons)
}

pub fn gcd(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

pub fn solve_2(moons: &[Moon<i64>]) -> usize {
    let ((mut px, mut py, mut pz), (mut vx, mut vy, mut vz)) = moons.iter().fold(
        ((vec![], vec![], vec![]), (vec![], vec![], vec![])),
        |mut r, m| {
            (r.0).0.push(m.pos().0);
            (r.0).1.push(m.pos().1);
            (r.0).2.push(m.pos().2);

            (r.1).0.push(m.vel().0);
            (r.1).1.push(m.vel().1);
            (r.1).2.push(m.vel().2);

            r
        },
    );

    let (px, dx) = cycle(&mut px, &mut vx);
    let (py, dy) = cycle(&mut py, &mut vy);
    let (pz, dz) = cycle(&mut pz, &mut vz);

    assert_eq!(dx, 0);
    assert_eq!(dy, 0);
    assert_eq!(dz, 0);

    let a = px * py / gcd(px, py);
    
    pz * a / gcd(pz, a)
}

pub fn part_1() -> i64 {
    solve_1(&DATA, 1000)
}

pub fn part_2() -> usize {
    solve_2(&DATA)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_display() {
        assert_eq!(
            format!("{}", Moon((1, 2, 3), (4, 5, 6))),
            String::from(r"Moon[pos=<x=1, y=2, z=3> vel=<x=4, y=5, z=6>]")
        );
    }

    #[test]
    fn test_debug() {
        assert_eq!(
            format!("{:?}", Moon((1, 2, 3), (4, 5, 6))),
            String::from(r"Moon[pos=<x=1, y=2, z=3> vel=<x=4, y=5, z=6>]")
        );
    }

    #[test]
    fn test_gravity_interation() {
        let mut ganimede = Moon::<i64>((3, 0, 0), (0, 0, 0));
        let mut callisto = Moon::<i64>((5, 0, 0), (0, 0, 0));

        ganimede.gravity_interaction(&mut callisto);

        assert_eq!(ganimede.vel(), &(1, 0, 0));
        assert_eq!(callisto.vel(), &(-1, 0, 0));
    }

    #[test]
    fn test_step() {
        let mut ganimede = Moon::<i64>((3, 0, 0), (1, 2, 3));

        assert_eq!(ganimede.step().pos(), &(4, 2, 3));
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            "<x=16, y=-11, z=2>".parse::<Moon<_>>().unwrap(),
            Moon((16, -11, 2), (0, 0, 0))
        );
    }

    #[test]
    fn test_moons_step() {
        let mut moons = Moon::<i64>::parse(
            r#"<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>"#,
        );
        let moons = moons.as_mut_slice();

        step(moons);

        assert_eq!(moons[0], Moon((2, -1, 1), (3, -1, -1)));
    }

    #[test]
    fn test_example_1_1() {
        let mut moons = Moon::<i64>::parse(
            r#"<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>"#,
        );
        let moons = moons.as_mut_slice();

        assert_eq!(solve_1(moons, 10), 179);
    }

    #[test]
    fn test_example_1_2() {
        let mut moons = Moon::<i64>::parse(
            r#"<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>"#,
        );
        let moons = moons.as_mut_slice();

        assert_eq!(solve_1(moons, 100), 1940);
    }

    #[test]
    fn test_example_2_1() {
        let moons = Moon::<i64>::parse(
            r#"<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>"#,
        );

        assert_eq!(solve_2(&moons), 2772);
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
