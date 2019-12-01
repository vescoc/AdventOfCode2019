#![feature(test)]
extern crate test;

use std::fs::File;
use std::io;
use std::io::prelude::*;

fn calculate_fuel(mass: i32) -> i32 {
    mass / 3 - 2
}

fn calculate_total_fuel(mass: i32) -> i32 {
    let mut total = 0;
    let mut current = mass;
    loop {
        current = calculate_fuel(current);
        if current <= 0 {
            break total;
        } else {
            total += current;
        }
    }
}

#[allow(dead_code)]
fn calculate_total_fuel_f(mass: i32) -> i32 {
    (0..)
        .try_fold((mass, 0), |(mass, total), _| {
            let current = calculate_fuel(mass);
            if current <= 0 {
                Err(total)
            } else {
                Ok((current, total + current))
            }
        })
        .err()
        .unwrap()
}

fn part(masses: &[i32], f: fn(i32) -> i32) -> i32 {
    masses.iter().copied().map(f).sum()
}

fn main() -> io::Result<()> {
    let data = {
        let mut contents = String::new();
        File::open("data.txt")?.read_to_string(&mut contents)?;
        contents
            .lines()
            .map(|l| l.parse().unwrap())
            .collect::<Vec<i32>>()
    };

    println!("part 1: {}", part(&data, calculate_fuel));
    println!("part 2: {}", part(&data, calculate_total_fuel));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example_1() {
        assert_eq!(calculate_fuel(12), 2);
        assert_eq!(calculate_fuel(14), 2);
        assert_eq!(calculate_fuel(1969), 654);
        assert_eq!(calculate_fuel(100756), 33583);
    }

    #[test]
    fn example_2() {
        assert_eq!(calculate_total_fuel(12), 2);
        assert_eq!(calculate_total_fuel(1969), 966);
        assert_eq!(calculate_total_fuel(100756), 50346);
    }

    #[test]
    fn example_2_f() {
        assert_eq!(calculate_total_fuel_f(12), 2);
        assert_eq!(calculate_total_fuel_f(1969), 966);
        assert_eq!(calculate_total_fuel_f(100756), 50346);
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
}
