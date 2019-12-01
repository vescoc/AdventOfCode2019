#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

use std::iter;

pub fn calculate_fuel(mass: u32) -> Option<u32> {
    mass.checked_div(3).and_then(|v| v.checked_sub(2))
}

pub fn calculate_fuel_i(mass: u32) -> impl Iterator<Item = u32> {
    iter::once(mass)
}

pub fn calculate_total_fuel(mass: u32) -> Option<u32> {
    let mut total = 0;
    let mut current = mass;
    loop {
        match calculate_fuel(current) {
            Some(c) if c > 0 => {
                current = c;
                total += c;
            }
            _ => break Some(total),
        }
    }
}

pub fn calculate_total_fuel_i(mass: u32) -> impl Iterator<Item = u32> {
    iter::successors(Some(mass), |mass| {
        mass.checked_div(3).and_then(|v| v.checked_sub(2))
    })
}

pub fn calculate_total_fuel_f(mass: u32) -> Option<u32> {
    (0..)
        .try_fold((mass, 0), |(mass, total), _| match calculate_fuel(mass) {
            Some(c) if c > 0 => Ok((c, total + c)),
            _ => Err(total),
        })
        .err()
}

pub fn part<I: IntoIterator<Item = u32>>(masses: &[u32], f: fn(u32) -> I) -> u32 {
    masses.iter().copied().flat_map(f).sum()
}

lazy_static! {
    pub static ref DATA: Vec<u32> = include_str!("../data.txt")
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();
}

fn main() -> Result<(), ()> {
    println!("part 1: {}", part(&DATA, calculate_fuel));
    println!("part 2: {}", part(&DATA, calculate_total_fuel));

    Ok(())
}

#[cfg(test)]
#[allow(clippy::unreadable_literal)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example_1() {
        assert_eq!(calculate_fuel(12), Some(2));
        assert_eq!(calculate_fuel(14), Some(2));
        assert_eq!(calculate_fuel(1969), Some(654));
        assert_eq!(calculate_fuel(100756), Some(33583));
    }

    #[test]
    fn example_2() {
        assert_eq!(calculate_total_fuel(12), Some(2));
        assert_eq!(calculate_total_fuel(1969), Some(966));
        assert_eq!(calculate_total_fuel(100756), Some(50346));
    }

    #[test]
    fn example_2_f() {
        assert_eq!(calculate_total_fuel_f(12), Some(2));
        assert_eq!(calculate_total_fuel_f(1969), Some(966));
        assert_eq!(calculate_total_fuel_f(100756), Some(50346));
    }

    #[bench]
    fn bench_example_1(b: &mut Bencher) {
        b.iter(|| calculate_fuel(100756))
    }

    #[bench]
    fn bench_example_2(b: &mut Bencher) {
        b.iter(|| calculate_total_fuel(100756))
    }

    #[bench]
    fn bench_example_2_f(b: &mut Bencher) {
        b.iter(|| calculate_total_fuel_f(100756))
    }

    #[bench]
    fn bench_part(b: &mut Bencher) {
        b.iter(|| part(&DATA, calculate_fuel));
        b.iter(|| part(&DATA, calculate_total_fuel));
    }

    #[bench]
    fn bench_part_i(b: &mut Bencher) {
        b.iter(|| part(&DATA, calculate_fuel_i));
        b.iter(|| part(&DATA, calculate_total_fuel_i));
    }

    #[bench]
    fn bench_part_f(b: &mut Bencher) {
        b.iter(|| part(&DATA, calculate_fuel));
        b.iter(|| part(&DATA, calculate_total_fuel_f));
    }
}
